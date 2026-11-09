# Daemons

Demonios del sistema. Cada uno es un proceso aislado, con su propio dominio de SELinux.

| Demonio | Función | Licencia |
|---|---|---|
| `clogd` | Logging único del sistema (`raw` y `json`) | GPL-3.0-or-later |
| `cnotifd` | Notificaciones | GPL-3.0-or-later |
| `cbrokerd` | Intermediario de permisos y servicios | GPL-3.0-or-later |
| `cpowerd` | Energía | GPL-3.0-or-later |
| `csettingsd` | Ajustes | GPL-3.0-or-later |
| `cupdated` | Actualizaciones del sistema | GPL-3.0-or-later |
| `cpushd` | Push propio, autoalojable | AGPL-3.0-or-later |
| `cfronterad` | Frontera FireWall | AGPL-3.0-or-later |

Los módulos que tocan la red van con AGPL; el resto con GPL. La licencia de cada crate está en su `Cargo.toml` y en sus cabeceras SPDX.

## Estado

Experimental. Aún no forman parte del hito 1.

## Compilar

```sh
cargo check -p clogd
```
