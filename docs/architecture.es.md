# Arquitectura de X4Shell

## Visión General
X4Shell es un shell de escritorio Wayland construido sobre Hyprland, que consta de tres componentes principales:

```
┌─────────────────────────────────────────────────────┐
│                    Gestor de Pantalla                  │
│                   (GDM, SDDM, etc)                 │
└──────────────────┬──────────────────────────────────┘
                   │ inicia
┌──────────────────▼──────────────────────────────────┐
│              script x4-shell-session                │
│  (inicia daemon, UI y Hyprland)                │
└─────┬──────────────┬───────────────┬──────────────┘
      │              │               │
┌─────▼─────┐ ┌────▼─────┐ ┌─────▼──────┐
│  Daemon   │ │    UI    │ │  Hyprland  │
│ (Rust)    │ │ (QML)   │ │ (Wayland)  │
└─────┬─────┘ └────┬─────┘ └─────┬──────┘
      │              │               │
      └────── D-Bus ─┼──────────────┘
                (comunicación IPC)
```

## Componentes

### 1. Daemon (`daemon/`)
- **Lenguaje**: Rust
- **Propósito**: Servicio de backend que gestiona espacios de trabajo, ventanas y estado del sistema
- **Comunicación**: sesión D-Bus
- **Archivos Clave**:
  - `src/main.rs`
  - `src/core/`
  - `src/services/`
  - `src/adapters/`
  - `src/ipc/`

### 2. UI (`ui/`)
- **Lenguaje**: QML (Quickshell)
- **Tiempo de Ejecución**: Quickshell
- **Propósito**: Paneles de interfaz de usuario, widgets y temas
- **Archivos Clave**:
  - `main.qml` - Punto de entrada
  - `panels/` - Paneles de UI (barra lateral, etc.)
  - `components/` - Componentes reutilizables
  - `services/` - Clientes de servicio D-Bus
  - `themes/` - Temas visuales

### 3. Script de Configuración (`setup.sh`)
- **Lenguaje**: Bash
- **Propósito**: Instalación, reparación, actualización y desinstalación


### 4. Configuración (`config/`)
- **Propósito**: Todas las plantillas y valores predeterminados para X4Shell
- **Subcarpetas**:
  - `hyprland/` - Configs de Hyprland
  - `daemon/` - Configs del daemon
  - `systemd/`
  - `session/` - Scripts de sesión y archivos .desktop
  - `ui/` - Temas y diseños de UI
  - `defaults/` - Configuraciones predeterminadas de usuario

## Flujo de Datos

1. **El usuario inicia sesión** → El greeter inicia `x4-shell-session`
2. **El script de sesión** inicia:
   - `x4shell-daemon` (en segundo plano)
   - `quickshell`
   - `Hyprland` (en primer plano, toma el control de la sesión)
3. **El daemon** se conecta al socket de Hyprland y expone una interfaz D-Bus
4. **La UI** se conecta al daemon vía D-Bus, muestra información de espacios de trabajo, etc.

## Interfaz D-Bus

- **Dbus**: Dbusde sesión
- **Servicio**: `org.x4yi.X4Shell.v1`
- **Ruta**: `/org/x4yi/X4Shell/v1`

## Ubicaciones de Archivos

| Componente | Instalación de Usuario | Instalación del Sistema |
|------------|------------------------|-------------------------|
| Binario del daemon | `~/.local/bin/x4shell-daemon` | `/usr/local/bin/x4shell-daemon` |
| Archivos de UI | `~/.local/share/x4-shell/ui/` | - |
| Configuración | `~/.config/x4-shell/` | - |
| Datos | `~/.local/share/x4-shell/` | - |
| Caché | `~/.cache/x4-shell/` | - |
| Servicio systemd | `~/.config/systemd/user/` | - |
| Script de sesión | - | `/usr/local/bin/x4-shell-session` |
| Archivo de escritorio | - | `/usr/share/wayland-sessions/` |

## Construcción e Instalación

```bash
# Construir daemon
cd daemon/
cargo build --release

# Instalar/Configurar
./setup.sh install
```

(Fin del archivo - total 111 líneas)