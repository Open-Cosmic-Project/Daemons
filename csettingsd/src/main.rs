//! csettingsd — Servicio de configuración y ajustes del sistema
//!
//! Alcance acordado en CosmicOS spec secciones 5, 7.5 y 10.10:
//! - Almacén tipado de ajustes del sistema en ~/.var/daemons/csettingsd/.
//! - Exporta preferencias globales de solo lectura a aplicaciones (tema, idioma, accesibilidad)
//!   vía interfaz Settings de xdg-desktop-portal (org.freedesktop.appearance).
//! - Separa ajustes portables (tema, idioma) de ajustes de dispositivo (brillo, calibración).

fn main() {
    println!("[csettingsd] Iniciando demonio de ajustes y configuración del sistema...");
    println!("[csettingsd] Almacén persistente en ~/.var/daemons/csettingsd/");
}
