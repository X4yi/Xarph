# Guía de Instalación de X4Shell

## Prerrequisitos

### Dependencias Requeridas
- **Hyprland** - Compositor Wayland
- **Quickshell** - Tiempo de ejecución QML
- **systemd** - Gestión de servicios
- **D-Bus** - Comunicación IPC
- **Rust & Cargo** - Para compilar el daemon

### Opcional
- **git** - Para actualizaciones mediante `setup.sh update`

## Métodos de Instalación

### Método 1: Interactivo (Recomendado)
```bash
cd X4Shell/
./setup.sh
# Luego seleccione la opción [1] Instalar
```

### Método 2: Comando Directo
```bash
./setup.sh install
```

### Método 3: Prueba en Seco (Probar sin cambios)
```bash
./setup.sh --dry-run install
```

## Qué se Instala

### 1. Binario del Daemon
- Se compila desde el directorio `daemon/`
- Se instala en `/usr/local/bin/x4shell-daemon` (sistema) o `~/.local/bin/x4shell-daemon` (usuario)

### 2. Archivos de UI
- Copia `ui/` a `~/.local/share/x4-shell/ui/`

### 3. Configuración
- Crea `~/.config/x4-shell/` con:
  - `hypr/hyprland.conf` - Configuración de Hyprland
  - `shell/settings.json` - Configuración del daemon
- Crea `~/.local/share/x4-shell/` para datos de tiempo de ejecución
- Crea `~/.cache/x4-shell/` para caché

### 4. Integración de Sesión
- Instala el script `/usr/local/bin/x4-shell-session`
- Instala `/usr/share/wayland-sessions/x4-shell.desktop` para gestores de pantalla

### 5. Servicio systemd
- Crea `~/.config/systemd/user/x4-shell-daemon.service`
- Habilita e inicia el servicio

## Post-Instalación

1. **Cierre de sesión y seleccione "X4 Shell"** desde su gestor de pantalla
2. O ejecute manualmente: `/usr/local/bin/x4-shell-session`

## Reparar Instalación
```bash
./setup.sh repair
```

## Actualizar Instalación
```bash
./setup.sh update
```

## Desinstalación
```bash
# Mantener archivos de configuración
./setup.sh uninstall

# Eliminar todo (purga)
./setup.sh uninstall --purge
```

## Solución de Problemas

### ¿El daemon no inicia?
```bash
./setup.sh status
journalctl --user -u x4-shell-daemon.service
```

### ¿La UI no se muestra?
Verifique si Quickshell está instalado: `which quickshell`

### ¿Problemas de configuración de Hyprland?
Restablezca a los valores predeterminados: `./setup.sh repair` (use `--force` para sobrescribir configuraciones)