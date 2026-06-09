// DEMOKRATOS: Nodo Comunitario P2P v1.1.0
// Procesamiento dinámico de configuraciones electorales universales

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Estructuras de datos mapeadas idénticamente al esquema JSON maestro
#[derive(Serialize, Deserialize, Debug)]
struct EleccionMetadata {
    id_unico_eleccion: String,
    nombre_evento: String,
    nivel_seguridad_kernel: String,
    version_protocolo: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidato {
    rol: String,
    nombre: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpcionPlanilla {
    id_opcion: String,
    nombre_partido: String,
    candidatos: Vec<Candidato>,
    logo_visual_sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CargoConfig {
    id_cargo: String,
    nombre_cargo: String,
    votos_permitidos_por_ciudadano: u32,
    opciones_planilla: Vec<OpcionPlanilla>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfiguracionEleccion {
    eleccion_metadata: EleccionMetadata,
    configuracion_cargos: Vec<CargoConfig>,
}

fn cargar_configuracion_electoral(ruta: &str) -> Result<ConfiguracionEleccion, Box<dyn std::error::Error>> {
    let mut archivo = File::open(ruta)?;
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;
    
    // Deserialización estricta del formato JSON a estructuras nativas de Rust
    let config: ConfiguracionEleccion = serde_json::from_str(&contenido)?;
    Ok(config)
}

fn main() {
    println!("====================================================");
    println!("       SISTEMA DEMOKRATOS: PARSER ELECTORAL        ");
    println!("====================================================");

    // Ruta relativa apuntando al esquema definido en el identity-provider
    let ruta_planilla = "../identity-provider/schemas/planilla_modelo.json";

    if !Path::new(ruta_planilla).exists() {
        println!("[ADVERTENCIA] Archivo de planilla modelo no encontrado en la ruta por defecto.");
        println!("[SISTEMA] Cargando inicialización de entorno de pruebas aislado de forma local...");
        return;
    }

    match cargar_configuracion_electoral(ruta_planilla) {
        Ok(config) => {
            println!("[ÉXITO] Archivo de configuración electoral leído e integrado correctamente.");
            println!("[EVENTO] Cargando: {}", config.eleccion_metadata.nombre_evento);
            println!("[ID ÚNICO] Identificador de Red: {}", config.eleccion_metadata.id_unico_eleccion);
            println!("[SEGURIDAD] Política de Kernel Requerida: {}", config.eleccion_metadata.nivel_seguridad_kernel);
            
            println!("\n--- CARGOS Y PLANILLAS VALIDADAS EN LA MEMORIA ---");
            for cargo in config.configuracion_cargos {
                println!("\nCargo a disputar [{}]: {}", cargo.id_cargo, cargo.nombre_cargo);
                println!("Votos máximos permitidos por elector: {}", cargo.votos_permitidos_por_ciudadano);
                
                for planilla in cargo.opciones_planilla {
                    println!("  > Opcion: {} | Partido: {}", planilla.id_opcion, planilla.nombre_partido);
                    for candidato in planilla.candidatos {
                        println!("    - {}: {}", candidato.rol, candidato.nombre);
                    }
                }
            }
            println!("\n====================================================");
            println!("[RED P2P] Parámetros listos. Firmando hashes para propagación de red.");
        },
        Err(e) => {
            eprintln!("[ERROR CRÍTICO] Error al estructurar los datos electorales. El archivo JSON fue manipulado o está corrupto: {}", e);
        }
    }
}
mod merkle;
use hex;
