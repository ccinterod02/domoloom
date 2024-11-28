use log::{Level, LevelFilter, SetLoggerError};
use std::io::Write;
use chrono::Local;
use colored::*;

/// Inicializa el sistema de logs con un nivel de detalle específico
pub fn init_logger(log_level: &str) -> Result<(), SetLoggerError> {
    // Establece el nivel de log según el argumento del usuario
    let level_filter = match log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info, // Nivel predeterminado
    };

    env_logger::Builder::new()
        .filter_level(level_filter) // Configura el nivel de detalle
        .format(|buf, record| {
            // Obtener timestamp formateado
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

            // Nivel de log coloreado
            let level = match record.level() {
                Level::Error => "ERROR".red().bold(),
                Level::Warn => "WARN".yellow().bold(),
                Level::Info => "INFO".green().bold(),
                Level::Debug => "DEBUG".blue().bold(),
                Level::Trace => "TRACE".magenta().bold(),
            };

            writeln!(
                buf,
                "[{}] [{}] {}",
                timestamp,
                level,
                record.args()
            )
        })
        .init();

    Ok(())
}
