mod models;
mod parser;
mod ml;

use anyhow::Result;
use models::LogEntry;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chemin vers le fichier de log à analyser
    #[arg(short, long, default_value = "access.log")]
    file: String,
}

fn main() -> Result<()> 
{
    let args = Args::parse();
    let file_path = args.file;

    println!("==================================");
    println!("Lancement de LogGuardian...");
    println!("==================================");

    let logs = parser::parse_log_file(&file_path)?;
    println!("{} lignes analysées avec succès.", logs.len());

    let mut matrix = LogEntry::to_matrix(&logs);
    LogEntry::normalize_matrix(&mut matrix);

    let threshold = 0.1;
    let anomalies = ml::detect_anomalies(&matrix, threshold);
    let mut anomaly_count = 0;
    for (log, &is_anomaly) in logs.iter().zip(anomalies.iter()) 
    {
        if is_anomaly 
        {
            anomaly_count += 1;
            println!("ANOMALIE : ip={}, status={}, size={}", log.ip, log.status, log.size);
        }
    }

    println!("\n==================================");
    println!("Résultat : {} anomalies détectées sur {} logs.", anomaly_count, logs.len());
    println!("==================================");

    Ok(())
}
