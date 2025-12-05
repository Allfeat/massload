//! Matrix Registry - Store and reuse transformation matrices
//!
//! Saves matrices to disk and automatically matches them to CSV formats based on columns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::transform::dsl::matrix::TransformationMatrix;

/// Directory where matrices are stored (relative to current dir)
const DEFAULT_REGISTRY_DIR: &str = ".massload/matrices";

/// A stored matrix with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMatrix {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// The transformation matrix
    pub matrix: TransformationMatrix,
    /// CSV columns this matrix was created for
    pub csv_columns: Vec<String>,
    /// Creation timestamp
    pub created_at: String,
    /// Last time this matrix was used
    pub last_used: Option<String>,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Number of times used
    pub use_count: u32,
}

/// Registry for managing transformation matrices
pub struct MatrixRegistry {
    /// Directory where matrices are stored
    registry_dir: PathBuf,
    /// Loaded matrices (id -> matrix)
    matrices: HashMap<String, StoredMatrix>,
}

impl MatrixRegistry {
    /// Create a new registry, loading existing matrices from disk
    pub fn new() -> Self {
        Self::with_dir(DEFAULT_REGISTRY_DIR)
    }

    /// Create a registry with a custom directory
    pub fn with_dir(dir: impl AsRef<Path>) -> Self {
        let registry_dir = PathBuf::from(dir.as_ref());
        let mut registry = Self {
            registry_dir,
            matrices: HashMap::new(),
        };
        registry.load_all();
        registry
    }

    /// Load all matrices from the registry directory
    fn load_all(&mut self) {
        if !self.registry_dir.exists() {
            return;
        }

        let entries = match fs::read_dir(&self.registry_dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(matrix) = serde_json::from_str::<StoredMatrix>(&content) {
                        self.matrices.insert(matrix.id.clone(), matrix);
                    }
                }
            }
        }
    }

    /// Get all stored matrices
    pub fn list(&self) -> Vec<&StoredMatrix> {
        self.matrices.values().collect()
    }

    /// Get a matrix by ID
    pub fn get(&self, id: &str) -> Option<&StoredMatrix> {
        self.matrices.get(id)
    }

    /// Find compatible matrices for given CSV columns
    /// Returns matrices sorted by compatibility score and success rate
    pub fn find_compatible(&self, csv_columns: &[String]) -> Vec<(&StoredMatrix, f64)> {
        let mut compatible: Vec<_> = self
            .matrices
            .values()
            .filter_map(|m| {
                let score = self.calculate_compatibility(&m.csv_columns, csv_columns);
                if score > 0.5 {
                    Some((m, score))
                } else {
                    None
                }
            })
            .collect();

        // Sort by: compatibility score * success rate (descending)
        compatible.sort_by(|a, b| {
            let score_a = a.1 * a.0.success_rate;
            let score_b = b.1 * b.0.success_rate;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        compatible
    }

    /// Calculate compatibility score between stored columns and CSV columns
    fn calculate_compatibility(&self, stored: &[String], csv: &[String]) -> f64 {
        if stored.is_empty() {
            return 0.0;
        }

        let csv_lower: Vec<String> = csv.iter().map(|c| c.to_lowercase()).collect();
        let match_count = stored
            .iter()
            .filter(|col| csv_lower.contains(&col.to_lowercase()))
            .count();

        match_count as f64 / stored.len() as f64
    }

    /// Save a new matrix to the registry
    pub fn save(
        &mut self,
        matrix: TransformationMatrix,
        name: &str,
        csv_columns: Vec<String>,
    ) -> Result<String, std::io::Error> {
        // Ensure directory exists
        fs::create_dir_all(&self.registry_dir)?;

        let id = self.generate_id(name);
        let stored = StoredMatrix {
            id: id.clone(),
            name: name.to_string(),
            matrix,
            csv_columns,
            created_at: chrono::Utc::now().to_rfc3339(),
            last_used: None,
            success_rate: 1.0,
            use_count: 0,
        };

        // Save to disk
        let path = self.registry_dir.join(format!("{}.json", id));
        let content = serde_json::to_string_pretty(&stored)?;
        fs::write(&path, content)?;

        self.matrices.insert(id.clone(), stored);
        Ok(id)
    }

    /// Import a matrix from a JSON file
    pub fn import(&mut self, path: &Path, name: Option<&str>) -> Result<String, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let matrix: TransformationMatrix = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid matrix JSON: {}", e))?;

        let matrix_name = name.unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("imported")
        });

        // Extract columns from the matrix transforms
        let csv_columns: Vec<String> = matrix
            .transforms
            .values()
            .filter_map(|t| t.source.clone())
            .collect();

        self.save(matrix, matrix_name, csv_columns)
            .map_err(|e| format!("Failed to save: {}", e))
    }

    /// Update statistics after using a matrix
    pub fn update_stats(&mut self, id: &str, success: bool) {
        if let Some(matrix) = self.matrices.get_mut(id) {
            // Exponential moving average
            matrix.success_rate = if success {
                matrix.success_rate * 0.9 + 0.1
            } else {
                matrix.success_rate * 0.9
            };
            matrix.last_used = Some(chrono::Utc::now().to_rfc3339());
            matrix.use_count += 1;

            // Save updated stats
            let path = self.registry_dir.join(format!("{}.json", id));
            if let Ok(content) = serde_json::to_string_pretty(matrix) {
                let _ = fs::write(&path, content);
            }
        }
    }

    /// Delete a matrix from the registry
    pub fn delete(&mut self, id: &str) -> Result<(), String> {
        if self.matrices.remove(id).is_some() {
            let path = self.registry_dir.join(format!("{}.json", id));
            fs::remove_file(&path).map_err(|e| format!("Failed to delete file: {}", e))?;
            Ok(())
        } else {
            Err(format!("Matrix not found: {}", id))
        }
    }

    /// Generate a unique ID from a name
    fn generate_id(&self, name: &str) -> String {
        let slug: String = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");

        let timestamp = chrono::Utc::now().timestamp_millis();
        format!("{}-{}", slug, timestamp)
    }
}

impl Default for MatrixRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_compatibility_score() {
        let registry = MatrixRegistry::with_dir(tempdir().unwrap().path());
        
        let stored = vec!["ISWC".to_string(), "Title".to_string(), "Role".to_string()];
        let csv = vec!["ISWC".to_string(), "Title".to_string(), "Creator".to_string()];
        
        let score = registry.calculate_compatibility(&stored, &csv);
        assert!((score - 0.666).abs() < 0.01); // 2/3 match
    }

    #[test]
    fn test_case_insensitive_match() {
        let registry = MatrixRegistry::with_dir(tempdir().unwrap().path());
        
        let stored = vec!["iswc".to_string(), "TITLE".to_string()];
        let csv = vec!["ISWC".to_string(), "title".to_string()];
        
        let score = registry.calculate_compatibility(&stored, &csv);
        assert!((score - 1.0).abs() < 0.01); // 100% match (case insensitive)
    }
}

