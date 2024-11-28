use clap::Parser;
use std::fs;
use std::path::Path;

// argumentos de la linea de comandos
#[derive(Parser, Debug)]
#[command(
    name = "YAML to CSV Converter",
    about = "Convierte un archivo YAML a CSV compatible con ETS6",
    author = "Carlos Cintero"
)]
pub struct CliArgs {
    // Ruta al archivo de configuración YAML
    #[arg(short, long)]
    pub config_path: String,

    // Directorio de salida para los archivos CSV generados
    #[arg(short, long, default_value = "./output")]
    pub output_dir: String,

    // Nivel de detalle del log (info, debug, error)
    #[arg(short, long, default_value = "info")]
    pub log_level: String,
}

// Procesa y devuelve los argumentos de la linea de comandos
pub fn parse_arguments() -> Result<CliArgs, Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    
    // Validar que el archivo de configuración YAML exista
    if !Path::new(&args.config_path).exists() {
        return Err(format!("El archivo de YAML de configuración '{}' no existe,", args.config_path).into());
    }

    // Validar o crear el directorio de salida
    let output_path = Path::new(&args.output_dir);
    if !output_path.exists() {
        println!("El directorio de salida '{}' no existe. Creándolo...", args.output_dir);
        fs::create_dir_all(output_path)?;
    } else if !output_path.is_dir() {
        return Err(format!(
            "La ruta '{}' no es un directorio válido.",
            args.output_dir
        )
        .into());
    }
    
    
    Ok(args)
}