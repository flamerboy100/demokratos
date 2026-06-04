markdown# Guía de Colaboración en DEMOKRATOS 🗳️

¡Gracias por querer ser parte de DEMOKRATOS! Este proyecto busca devolver el control de los procesos democráticos a la sociedad civil de forma transparente. Para mantener el orden y la seguridad matemática del sistema, dividimos los aportes en tres pilares:

## 1. Perfil Técnico (Desarrolladores C/Rust/Go)
El núcleo operativo requiere máxima estabilidad y revisiones estrictas:
* **Módulo del Kernel:** Cualquier cambio o controlador para sensores biométricos debe pasar por auditorías estáticas de código para evitar desbordamientos de memoria (*Memory Corruption*).
* **Nodos P2P (Rust):** Las optimizaciones del protocolo de red deben probarse primero en entornos locales simulados (`Docker/Testnet`) antes de proponer un *Pull Request*.
* **Criptografía:** Todo algoritmo criptográfico nuevo propuesto (ZK-SNARKs o Homomórfico) debe estar acompañado de su respectiva demostración matemática en formato técnico.

## 2. Perfil de Activismo y Traducción
No necesitas programar para auditar una elección; necesitamos interfaces comprensibles para todos:
* **Traducciones:** Ayúdanos a traducir las interfaces de usuario del sistema electoral en la carpeta `/src/identity-provider/locales/`.
* **Diseño UX/UI:** Las planillas y pantallas de votación deben ser tan claras que una persona de cualquier edad pueda usarlas sin asistencia técnica. Puedes enviar tus prototipos o ideas en la pestaña de *Discusiones*.

## 3. Propuestas de Mejora de Demokratos (DIPs)
Si deseas proponer un cambio estructural en el funcionamiento de la red o en las políticas de seguridad:
1. Crea un documento siguiendo la plantilla base en `docs/dips/DIP-template.md`.
2. Explica detalladamente el problema y la solución matemática o social propuesta.
3. Abre un *Issue* con la etiqueta `Propuesta de Mejora` para que toda la comunidad pueda debatirla públicamente.

## 🚫 Reglas de Oro
* Ningún código se integra de forma directa sin la aprobación y revisión cruzada de al menos dos desarrolladores del núcleo.
* Mantenemos un ambiente científico, respetuoso y abierto en los debates. La confianza se construye con argumentos técnicos, nunca personales.
