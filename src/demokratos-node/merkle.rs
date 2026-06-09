// DEMOKRATOS: Motor Criptográfico del Árbol de Merkle (Estilo Bitcoin)
// v1.0.0 - Verificación de Padrón Electoral Inmutable

use sha2::{Sha256, Digest};

/// Toma dos hashes SHA-256 en formato de texto (Hexadecimal), los concatena
/// y calcula un nuevo hash SHA-256 combinando ambos nodos del árbol.
pub fn combinar_nodos(hash_izq: &str, hash_der: &str) -> String {
    let mut hasher = Sha256::new();
    
    // Convertir los textos hexadecimales a bytes e introducirlos al procesador
    let bytes_izq = hex::decode(hash_izq).unwrap_or_default();
    let bytes_der = hex::decode(hash_der).unwrap_or_default();
    
    hasher.update(&bytes_izq);
    hasher.update(&bytes_der);
    
    // Retornar el resultado final en formato de cadena de texto hexadecimal
    format!("{:x}", hasher.finalize())
}

/// Calcula la Raíz de Merkle de forma iterativa a partir de una lista de hashes de votantes.
/// Emula el comportamiento exacto de bloques de Bitcoin para el padrón electoral.
pub fn calcular_raiz_merkle(mut hashes_votantes: Vec<String>) -> Option<String> {
    if hashes_votantes.is_empty() {
        return None;
    }

    // Mientras tengamos más de un nodo en el nivel actual del árbol, seguimos combinando
    while hashes_votantes.len() > 1 {
        // Si el nivel tiene un número impar de nodos, duplicamos el último (Regla Bitcoin)
        if hashes_votantes.len() % 2 != 0 {
            let ultimo = hashes_votantes.last().unwrap().clone();
            hashes_votantes.push(ultimo);
        }

        let mut nivel_superior = Vec::new();
        
        // Agrupar de 2 en 2 para calcular el nivel de arriba
        for chunk in hashes_votantes.chunks(2) {
            let nuevo_padre = combinar_nodos(&chunk[0], &chunk[1]);
            nivel_superior.push(nuevo_padre);
        }
        
        hashes_votantes = nivel_superior;
    }

    // El último nodo sobreviviente es la Raíz de Merkle Oficial
    Some(hashes_votantes[0].clone())
}
