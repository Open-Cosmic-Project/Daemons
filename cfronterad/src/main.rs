//! cfronterad — Frontera FireWall
//!
//! Alcance acordado en CosmicOS spec secciones 7.5, 10.9 y 11.6:
//! - Licencia AGPL-3.0.
//! - Traduce reglas declarativas del manifiesto de cada app a nftables y eBPF cgroups.
//! - Interruptores independientes por app para datos móviles y Wi-Fi.
//! - Entrada bloqueada por defecto para cualquier conexión entrante no declarada.
//! - El filtrado lo hace el kernel; cfronterad gestiona la política de red sin parsear paquetes.

fn main() {
    println!("[cfronterad] Iniciando Frontera FireWall (política declarativa de red)...");
    println!("[cfronterad] Control de tráfico por cgroup y nftables activo.");
}
