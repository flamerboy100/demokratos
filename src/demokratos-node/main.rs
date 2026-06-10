// DEMOKRATOS: Nodo Comunitario P2P v1.2.0
// Procesamiento de elecciones y simulación de transacciones de votación criptográfica

mod merkle;
#[cfg(test)]
mod tests;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use sha2::{Sha256, Digest};

// Estructuras de datos para la configuración electoral
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

// =========================================================================
// NUEVA ESTRUCTURA: Transacción de Voto Criptográfico (Estilo Bitcoin)
// =========================================================================
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TransaccionVoto {
    id_eleccion: String,
    id_mesa: String,
    voto_cifrado_planilla: String, // Representa la planilla cifrada homomórficamente
    prueba_zkp_votante: String,     // Hash de validación del Árbol de Merkle del padrón
    timestamp_utc: u64,
}

impl TransaccionVoto {
    // Genera el hash identificador único de la transacción (TXID)
    fn calcular_txid(&self) -> String {
        let mut hasher = Sha256::new();
        let datos_a_firmar = format!(
            "{}{}{}{}{}",
            self.id_eleccion, self.id_mesa, self.voto_cifrado_planilla, self.prueba_zkp_votante, self.timestamp_utc
        );
        hasher.update(datos_a_firmar.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

fn cargar_configuracion_electoral(ruta: &str) -> Result<ConfiguracionEleccion, Box<dyn std::error::Error>> {
    let mut archivo = File::open(ruta)?;
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;
    let config: ConfiguracionEleccion = serde_json::from_str(&contenido)?;
    Ok(config)
}

fn main() {
    println!("====================================================");
    println!("       SISTEMA DEMOKRATOS: RED DE VOTACIÓN          ");
    println!("====================================================");

    let ruta_planilla = "../identity-provider/schemas/planilla_modelo.json";

    if !Path::new(ruta_planilla).exists() {
        println!("[ADVERTENCIA] Archivo de planilla modelo no encontrado en la ruta por defecto.");
        return;
    }

    match cargar_configuracion_electoral(ruta_planilla) {
        Ok(config) => {
            println!("[ÉXITO] Archivo de configuración electoral integrado correctamente.");
            println!("[EVENTO] Registrado: {}\n", config.eleccion_metadata.nombre_evento);
            
            // -----------------------------------------------------------------
            // SIMULACIÓN: Emisión y propagación P2P de un voto biométrico
            // -----------------------------------------------------------------
            println!("[URNA-KERNEL] Elector verificado mediante sensor biométrico.");
            println!("[URNA-KERNEL] Generando prueba criptográfica de pertenencia al Padrón...");
            
            // Simulamos un voto hacia la "PLANILLA-10" (Frente de la Comunidad Abierta)
            let voto_ejemplo = TransaccionVoto {
                id_eleccion: config.eleccion_metadata.id_unico_eleccion.clone(),
                id_mesa: String::from("MESA-URBANA-004"),
                voto_cifrado_planilla: String::from("Cifrado(PLANILLA-10)"), 
                prueba_zkp_votante: String::from("8f3c95e8756c7d12f342674e2d31a56bc741a6b738918234e45c719efcf67a2f"),
                timestamp_utc: 1792965600, // Tiempo simulado en formato Unix Epoch
            };

            let txid = voto_ejemplo.calcular_txid();
            println!("\n[P2P] >>> TRANSMITIENDO VOTO INMUTABLE A LA RED <<<");
            println!("  [TXID (ID Voto)] : {}", txid);
            println!("  [Mesa Electoral] : {}", voto_ejemplo.id_mesa);
            println!("  [Cripto-Voto]    : {}", voto_ejemplo.voto_cifrado_planilla);
            println!("  [Prueba Padrón]  : {}", voto_ejemplo.prueba_zkp_votante);
            
            println!("\n[NODOS COMUNIDAD] Transacción recibida y validada por la red de observadores.");
            println!("[MONITOREO] Sumando voto de forma matemática al total acumulado en tiempo real.");
            println!("====================================================");
        },
        Err(e) => {
            eprintln!("[ERROR CRÍTICO] El archivo JSON está corrupto o mal estructurado: {}", e);
        }
    }
}
