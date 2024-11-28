
# Domoloom

[![Rust](https://img.shields.io/badge/Rust-Edition%202021-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue)](./LICENSE)

Convierte archivos YAML que describen edificios, plantas, habitaciones y dispositivos en archivos CSV compatibles con **ETS6**.

---

## 🚀 Características

- **Jerarquía YAML**: Procesa estructuras detalladas.
- **Generación CSV**: Crea archivos CSV organizados para integrar datos en ETS6.

---

## 🛠️ Instalación

1. Clona este repositorio:
   ```bash
   git clone https://github.com/ccinterod02/domoloom.git
   cd domoloom
   ```

2. Construye el proyecto:
   ```bash
   cargo build
   ```

---

## 🚀 Uso

1. Estructura requerida:

   ```yaml
   buildings:
     - name: Building A
       floors:
         - name: Ground Floor
           rooms:
             - name: Living Room
               devices:
                 lights:
                   - Light 1
                   - Light 2
                 climate:
                   - AC 1
   ```

2. Ejecuta el programa:
   ```bash
   cargo run -- --config-path config.yaml --output-dir ./csv_output --log-level info
   ```

   - `--config-path`: Ruta al archivo YAML.
   - `--output-dir`: Directorio donde se guardarán los CSV (opcional, predeterminado: `./output`).
   - `--log-level`: Nivel de logging (`info`, `debug`, `error`, etc.).

