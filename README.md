# X4Shell

Shell de escritorio para Wayland basado en Hyprland. Proyecto personal sin afiliación a ninguna empresa u organización.

> **Estado actual**: Prototipo temprano. La infraestructura base está implementada pero no hay funcionalidad end-to-end completa.

---

## Características implementadas.

### Backend (x4shell-daemon)
- Conexión al socket de Hyprland (`.socket2.sock`) con lógica de reintento
- Parseo de 5 tipos de eventos de Hyprland: `workspace`, `windowOpen`, `windowClose`, `monitorAdded`, `monitorRemoved`
- Servidor D-Bus en el bus de sesión exponiendo la ruta `/org/x4yi/X4Shell/v1`
- Almacenamiento de estado con `ArcStateStore` (áreas de trabajo)
- Apagado graceful vía señales SIGINT/SIGTERM
- Métodos D-Bus: `GetWorkspaces`, `SwitchWorkspace`, `Ping`
- Emisión de señal `WorkspaceChanged` cuando cambia el estado

### UI (x4-shell-ui)
- Punto de entrada QML (`main.qml`) que carga un panel lateral
- Servicios: `WorkspaceService`, `TimeService`, `SystemService`, `ConfigService`
- Componentes: `Clock`, `WorkspaceList`, `SystemIndicators`, `SystemIcon`
- `ConfigService.qml` implementado: carga configuración desde `settings.json`
- Tema básico (`DefaultTheme.qml`)

---

## Instalación

### Requisitos previos
- Sistema basado en Arch Linux
- Hyprland instalado
- Quickshell instalado
- Rust toolchain (para compilar)
- Qt6 (qt6-base, qt6-declarative)

### Pasos
```bash
git clone https://github.com/X4yi/X4Shell.git
cd X4Shell/x4-shell-manager
cargo run -- install
```

Para instalación system-wide:
```bash
cargo run -- install --system-wide
```

Para probar sin cambios reales:
```bash
cargo run -- install --dry-run
```

### Desinstalación
```bash
cargo run -- uninstall
```

---

## Configuración

### Archivos de configuración
- **Hyprland**: `$XDG_CONFIG_HOME/x4-shell/hypr/hyprland.conf`
- **Shell**: `$XDG_CONFIG_HOME/x4-shell/shell/settings.json`

### Ejemplo de `settings.json`
```json
{
    "sidebar": {
        "position": "left"
    },
    "clock": {
        "format": "24h"
    }
}
```

---

## Qué falta (No implementado)

### Funcionalidad core
- **Sincronización end-to-end**: Los eventos de Hyprland se reciben pero el estado de workspaces no se actualiza consistentemente
- **Métodos D-Bus incompletos**: `SwitchWorkspace` envía comandos a Hyprland pero no verifica respuesta
- **Manejo de ventanas/monitores**: Los tipos `Window`, `Monitor` están definidos pero no se usan
- **UI no se conecta a señales D-Bus**: `WorkspaceService.qml` no escucha `workspace_changed` correctamente
- **Falta servicio systemd**: El instalador no crea el servicio systemd automáticamente

### Comandos del manager
- `update`, `repair`, `status` están incompletos
- `uninstall` no maneja correctamente archivos faltantes

### Dependencias
- Quickshell no se instala automáticamente (solo se verifica)
- No hay verificación de versión de Quickshell

## Requisitos del sistema

- **OS**: Arch Linux Based distro
- **Wayland compositor**: Hyprland
- **UI toolkit**: Quickshell (QML)
- **Rust**: Para compilación
- **Qt6**: qt6-base, qt6-declarative

---

## Estado

El proyecto está en una fase **muy temprana**. El esqueleto está ahí: el daemon se compila, el manager instala archivos, la UI tiene estructura. Pero no hay un flujo completo que funcione de principio a fin: los workspaces no se sincronizan, la UI no reacciona a cambios, y faltan comandos importantes.

**No usar en producción**. Es un proyecto personal incompleto

## Capturas de pantalla

*Por ahora no hay capturas: la UI no es completamente funcional.*

---

## Cómo contribuir

Como es un proyecto personal, no se aceptan contribuciones externas por el momento. Pero siéntete libre de hacer fork y adaptarlo a tus necesidades.
