//! Massload CLI - Transform CSV files to MIDDS format
//!
//! # Main Commands
//!
//! ```bash
//! massload serve                    # Start HTTP server (port 3000)
//! massload transform input.csv     # Transform CSV to MIDDS JSON
//! massload template list           # Manage transformation templates
//! ```
//!
//! # Debug Commands (for development)
//!
//! ```bash
//! massload parse input.csv         # Just parse CSV to JSON
//! massload validate input.json     # Validate JSON against schema
//! massload group input.json        # Group flat records by ISWC
//! massload operations              # Show available DSL operations
//! massload example-matrix          # Show example transformation matrix
//! ```

use clap::{Parser, Subcommand};
use massload::{
    flat_to_grouped, validate_musical_work_flat,
    parse_csv_file_auto, MatrixRegistry,
    transform_csv, transform_with_matrix, TransformOptions,
};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "massload")]
#[command(about = "Transform CSV files to MIDDS Musical Work format", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a CSV file and output JSON
    Parse {
        /// Input CSV file
        input: PathBuf,

        /// CSV delimiter (auto-detect if not specified)
        #[arg(short, long)]
        delimiter: Option<char>,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Full transformation pipeline: CSV → AI Matrix → MIDDS JSON
    Transform {
        /// Input CSV file
        input: PathBuf,

        /// CSV delimiter (auto-detect if not specified)
        #[arg(short, long)]
        delimiter: Option<char>,

        /// Use existing matrix file instead of AI
        #[arg(short, long)]
        matrix: Option<PathBuf>,

        /// Save generated matrix to file
        #[arg(long)]
        save_matrix: Option<PathBuf>,

        /// Output file for flat records (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Also output grouped records
        #[arg(short, long)]
        grouped: Option<PathBuf>,

        /// Number of preview rows for AI (default: 10)
        #[arg(long, default_value = "10")]
        preview_rows: usize,

        /// Skip validation
        #[arg(long)]
        no_validate: bool,
    },

    /// Validate JSON records against MIDDS flat schema
    Validate {
        /// Input JSON file (array of records)
        input: PathBuf,
    },

    /// Group flat records by ISWC
    Group {
        /// Input JSON file (array of flat records)
        input: PathBuf,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Show example transformation matrix
    ExampleMatrix,

    /// Show available DSL operations
    Operations,

    /// Start HTTP server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Manage transformation matrix templates
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    /// List all stored templates
    List,

    /// Import a matrix JSON file as template
    Import {
        /// Matrix JSON file to import
        file: PathBuf,
        /// Name for the template
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Show details of a template
    Show {
        /// Template ID
        id: String,
    },

    /// Delete a template
    Delete {
        /// Template ID
        id: String,
    },

    /// Use a specific template to transform a CSV
    Use {
        /// Template ID
        id: String,
        /// Input CSV file
        input: PathBuf,
        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Also output grouped records
        #[arg(short, long)]
        grouped: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    // Load .env file (if present)
    dotenvy::dotenv().ok();
    
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Parse {
            input,
            delimiter,
            output,
        } => cmd_parse(&input, delimiter, output.as_deref()),

        Commands::Transform {
            input,
            delimiter,
            matrix,
            save_matrix,
            output,
            grouped,
            preview_rows,
            no_validate,
        } => {
            cmd_transform(
                &input,
                delimiter,
                matrix.as_deref(),
                save_matrix.as_deref(),
                output.as_deref(),
                grouped.as_deref(),
                preview_rows,
                no_validate,
            )
            .await
        }

        Commands::Validate { input } => cmd_validate(&input),

        Commands::Group { input, output } => cmd_group(&input, output.as_deref()),

        Commands::ExampleMatrix => cmd_example_matrix(),

        Commands::Operations => cmd_operations(),

        Commands::Serve { port } => cmd_serve(port).await,

        Commands::Template { action } => cmd_template(action).await,
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn cmd_parse(
    input: &Path,
    delimiter: Option<char>,
    output: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("📄 Parsing CSV: {}", input.display());

    let result = parse_csv_file_auto(input)?;
    
    let used_delimiter = delimiter.unwrap_or(result.delimiter);
    eprintln!("   Encoding: {}", result.encoding);
    eprintln!("   Delimiter: '{}'{}", 
        match used_delimiter {
            '\t' => "\\t".to_string(),
            c => c.to_string(),
        },
        if delimiter.is_none() { " (auto-detected)" } else { "" }
    );
    eprintln!("   Columns: {}", result.headers.join(", "));
    eprintln!("✅ Parsed {} records", result.records.len());

    let json = serde_json::to_string_pretty(&result.records)?;
    write_output(&json, output)?;

    Ok(())
}

async fn cmd_transform(
    input: &Path,
    _delimiter: Option<char>,
    matrix_path: Option<&Path>,
    save_matrix: Option<&Path>,
    output: Option<&Path>,
    grouped_output: Option<&Path>,
    preview_rows: usize,
    no_validate: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("📄 Processing: {}", input.display());

    // Build options
    let options = TransformOptions {
        matrix_path: matrix_path.map(|p| p.to_string_lossy().to_string()),
        preview_rows,
        skip_validation: no_validate,
        no_cache: false,
        no_save: false,
    };

    // Run pipeline
    let result = transform_csv(input, options).await?;

    // Display info
    eprintln!("   Encoding: {}", result.csv_info.encoding);
    eprintln!("   Delimiter: '{}'", format_delimiter(result.csv_info.delimiter));
    eprintln!("   Rows: {}", result.csv_info.row_count);
    eprintln!("   Columns: {}", result.csv_info.headers.join(", "));

    if let Some(ref tid) = result.template_id {
        eprintln!("   Template: {}", tid);
    }

    eprintln!("\n⚙️  Transformed: {} flat records", result.flat.len());

    // Validation results
    if !no_validate {
        eprintln!("\n✔️  Validation:");
        if result.invalid_count > 0 {
            eprintln!("   ✅ Valid: {}", result.valid_count);
            eprintln!("   ❌ Invalid: {}", result.invalid_count);
            for (i, errors) in result.validation_errors.iter().take(5) {
                eprintln!("\n   Record {}:", i);
                for err in errors.iter().take(3) {
                    eprintln!("     - {}", err);
                }
            }
        } else {
            eprintln!("   ✅ All {} records valid!", result.valid_count);
        }
    }

    // Save matrix if requested
    if let Some(save_path) = save_matrix {
        let matrix_json = result.matrix.to_json()?;
        fs::write(save_path, &matrix_json)?;
        eprintln!("   💾 Matrix saved to: {}", save_path.display());
    }

    // Output flat records
    let flat_json = serde_json::to_string_pretty(&result.flat)?;
    write_output(&flat_json, output)?;

    // Grouped output
    if let Some(grouped_path) = grouped_output {
        eprintln!("\n📦 Grouped: {} unique works", result.grouped.len());
        let grouped_json = serde_json::to_string_pretty(&result.grouped)?;
        fs::write(grouped_path, &grouped_json)?;
        eprintln!("   💾 Saved to: {}", grouped_path.display());
    }

    eprintln!("\n✨ Done!");
    Ok(())
}

fn format_delimiter(d: char) -> String {
    match d {
        '\t' => "\\t".to_string(),
        c => c.to_string(),
    }
}

fn cmd_validate(input: &Path) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("✔️  Validating: {}", input.display());

    let content = fs::read_to_string(input)?;
    let records: Vec<Value> = serde_json::from_str(&content)?;

    let mut valid = 0;
    let mut invalid = 0;

    for (i, record) in records.iter().enumerate() {
        match validate_musical_work_flat(record) {
            Ok(()) => valid += 1,
            Err(errors) => {
                invalid += 1;
                if invalid <= 5 {
                    eprintln!("\n❌ Record {} invalid:", i);
                    for err in errors.iter().take(3) {
                        eprintln!("   - {}", err);
                    }
                }
            }
        }
    }

    eprintln!("\n📊 Results: {} valid, {} invalid", valid, invalid);

    if invalid > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn cmd_group(input: &Path, output: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("📦 Grouping: {}", input.display());

    let content = fs::read_to_string(input)?;
    let records: Vec<Value> = serde_json::from_str(&content)?;

    eprintln!("   {} flat records", records.len());

    let grouped = flat_to_grouped(records);
    eprintln!("   {} unique works", grouped.len());

    let json = serde_json::to_string_pretty(&grouped)?;
    write_output(&json, output)?;

    Ok(())
}

fn cmd_example_matrix() -> Result<(), Box<dyn std::error::Error>> {
    let matrix = massload::example_matrix();
    let json = matrix.to_json()?;
    println!("{}", json);
    Ok(())
}

fn cmd_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", massload::operations_description());
    Ok(())
}

async fn cmd_serve(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    massload::server::start_server(port).await
}

fn write_output(content: &str, path: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    match path {
        Some(p) => {
            fs::write(p, content)?;
            eprintln!("💾 Output written to: {}", p.display());
        }
        None => {
            println!("{}", content);
        }
    }
    Ok(())
}

async fn cmd_template(action: TemplateAction) -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = MatrixRegistry::new();

    match action {
        TemplateAction::List => {
            let templates = registry.list();
            if templates.is_empty() {
                eprintln!("📋 No templates stored yet.");
                eprintln!("   Use 'massload template import <file>' to add one.");
                return Ok(());
            }

            eprintln!("📋 Stored templates ({}):\n", templates.len());
            for t in templates {
                println!("  📄 {} ({})", t.name, t.id);
                println!("     Columns: {}", t.csv_columns.join(", "));
                println!("     Success rate: {:.0}%", t.success_rate * 100.0);
                println!("     Uses: {}", t.use_count);
                if let Some(ref last) = t.last_used {
                    println!("     Last used: {}", last);
                }
                println!();
            }
        }

        TemplateAction::Import { file, name } => {
            let template_name = name.as_deref().unwrap_or_else(|| {
                file.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("imported")
            });

            eprintln!("📥 Importing template from: {}", file.display());
            let id = registry.import(&file, Some(template_name))?;
            eprintln!("✅ Template saved with ID: {}", id);
        }

        TemplateAction::Show { id } => {
            match registry.get(&id) {
                Some(t) => {
                    println!("📄 Template: {} ({})\n", t.name, t.id);
                    println!("CSV Columns: {}", t.csv_columns.join(", "));
                    println!("Created: {}", t.created_at);
                    println!("Success rate: {:.0}%", t.success_rate * 100.0);
                    println!("Uses: {}", t.use_count);
                    println!("\nMatrix:");
                    println!("{}", serde_json::to_string_pretty(&t.matrix)?);
                }
                None => {
                    return Err(format!("Template not found: {}", id).into());
                }
            }
        }

        TemplateAction::Delete { id } => {
            registry.delete(&id)?;
            eprintln!("🗑️  Template deleted: {}", id);
        }

        TemplateAction::Use { id, input, output, grouped } => {
            let template = registry.get(&id)
                .ok_or_else(|| format!("Template not found: {}", id))?;

            eprintln!("📄 Using template: {} ({})", template.name, template.id);
            
            // Parse and transform
            let parse_result = parse_csv_file_auto(&input)?;
            eprintln!("   Found {} rows", parse_result.records.len());

            let result = transform_with_matrix(&parse_result.records, &template.matrix, true);
            eprintln!("   Transformed: {} records", result.flat.len());

            // Update stats
            let mut registry_mut = MatrixRegistry::new();
            registry_mut.update_stats(&id, result.invalid_count == 0);

            if result.invalid_count == 0 {
                eprintln!("   ✅ All {} records valid!", result.valid_count);
            } else {
                eprintln!("   ⚠️  {} valid, {} invalid", 
                    result.valid_count, result.invalid_count);
            }

            // Output flat
            let flat_json = serde_json::to_string_pretty(&result.flat)?;
            write_output(&flat_json, output.as_deref())?;

            // Grouped output
            if let Some(grouped_path) = grouped {
                eprintln!("📦 Grouped: {} works", result.grouped.len());
                let grouped_json = serde_json::to_string_pretty(&result.grouped)?;
                fs::write(&grouped_path, &grouped_json)?;
                eprintln!("💾 Saved to: {}", grouped_path.display());
            }
        }
    }

    Ok(())
}

