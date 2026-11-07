//! cnotifd — Demonio de notificaciones
//!
//! Alcance acordado en CosmicOS spec secciones 7.5 y 10.1:
//! - Demonio de notificaciones separado del compositor (PhanteonWM) y del shell.
//! - Las apps generan un token efímero de 10 a 30 segundos.
//! - Dibuja la notificación en overlay y la desecha del sistema.
//! - Respeta modo 'No molestar'.

fn main() {
    println!("[cnotifd] Iniciando demonio de notificaciones efímeras...");
    println!("[cnotifd] Esperando eventos de notificación...");
}
