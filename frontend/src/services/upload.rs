//! Service HTTP pour upload de fichiers CSV vers le backend

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::{File, FormData};

/// Response du backend pour l'upload
/// Les musical_works sont en format MIDDS natif
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub job_id: String,
    pub status: String,
    /// Musical works en format MIDDS - prêts pour la blockchain
    pub musical_works: Vec<Value>,
    pub metadata: ResponseMetadata,
}

/// Métadonnées de la réponse
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    pub total_works: usize,
    pub estimated_cost: String,
    pub matrix_id: Option<String>,
    pub cached: bool,
    pub csv_info: CsvInfo,
    pub validation: ValidationStats,
}

/// Info CSV
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvInfo {
    pub encoding: String,
    pub delimiter: String,
    pub row_count: usize,
    pub columns: Vec<String>,
}

/// Stats validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationStats {
    pub valid: usize,
    pub invalid: usize,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub record_index: usize,
    pub errors: Vec<String>,
}

/// Upload un fichier CSV vers le backend
pub async fn upload_csv(file: File, backend_url: &str) -> Result<UploadResponse, String> {
    // Créer FormData
    let form_data = FormData::new().map_err(|e| format!("Failed to create FormData: {:?}", e))?;
    
    // Ajouter le fichier
    form_data
        .append_with_blob("file", &file)
        .map_err(|e| format!("Failed to append file: {:?}", e))?;

    // Envoyer la requête
    let url = format!("{}/api/upload", backend_url);
    let request = Request::post(&url)
        .body(form_data)
        .map_err(|e| format!("Failed to build request: {}", e))?;
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    // Vérifier le status
    if !response.ok() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Server error ({}): {}", response.status(), error_text));
    }

    // Parser la réponse JSON
    response
        .json::<UploadResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_deserialization() {
        // Format MIDDS natif retourné par massload
        let json = r#"{
            "jobId": "123e4567-e89b-12d3-a456-426614174000",
            "status": "ready",
            "musicalWorks": [
                {
                    "iswc": "T1234567890",
                    "title": "My Song",
                    "creationYear": 2024,
                    "instrumental": false,
                    "language": "English",
                    "creators": [
                        {"id": {"Ipi": 123456789}, "role": "Composer"}
                    ],
                    "workType": "Original"
                }
            ],
            "metadata": {
                "totalWorks": 1,
                "estimatedCost": "0.05 AFT",
                "matrixId": "MusicalWorks-123",
                "cached": false,
                "csvInfo": {
                    "encoding": "utf-8",
                    "delimiter": ",",
                    "rowCount": 1,
                    "columns": ["ISWC", "Title"]
                },
                "validation": {
                    "valid": 1,
                    "invalid": 0,
                    "errors": []
                }
            }
        }"#;

        let result: Result<UploadResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.status, "ready");
        assert_eq!(response.metadata.total_works, 1);
        assert_eq!(response.metadata.cached, false);
        assert_eq!(response.metadata.csv_info.encoding, "utf-8");
    }
}
