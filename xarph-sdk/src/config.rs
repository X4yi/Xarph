use notify::{Event as NotifyEvent, EventKind, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

// ── Wallpaper Configuration ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WallpaperConfig {
    #[serde(rename = "image")]
    Image {
        path: String,
        #[serde(default = "default_wallpaper_mode")]
        mode: WallpaperMode,
    },
    #[serde(rename = "color")]
    Color { hex: String },
}

fn default_wallpaper_mode() -> WallpaperMode {
    WallpaperMode::Fill
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WallpaperMode {
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "stretch")]
    Stretch,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "tile")]
    Tile,
}

impl Default for WallpaperMode {
    fn default() -> Self {
        WallpaperMode::Fill
    }
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        WallpaperConfig::Color {
            hex: "#1a1b26".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceWallpaper {
    pub wallpaper: Option<WallpaperConfig>,
}

// ── Main Configuration ──────────────────────────────────────────────────

#[derive(Clone)]
pub struct XarphConfig {
    pub theme: Option<String>,
    pub launcher_grid_size: Option<u32>,
    pub clock_format: Option<String>,
    pub system_refresh_ms: Option<u64>,
    pub show_cpu: Option<bool>,
    pub show_ram: Option<bool>,
    pub network_interface: Option<String>,
    pub theme_config: Option<ThemeConfig>,
    pub widget_config: WidgetConfig,
    pub keybind_config: KeybindConfig,
    pub automation_rules: Vec<AutomationRule>,
    pub notification_config: NotificationConfig,
    pub gaming_mode: GamingModeConfig,
    pub wallpaper_global: WallpaperConfig,
    pub workspace_wallpapers: HashMap<u8, WorkspaceWallpaper>,
    pub pinned_apps: Vec<String>,
    pub shell: ShellConfig,
}

impl Default for XarphConfig {
    fn default() -> Self {
        Self {
            theme: None,
            launcher_grid_size: None,
            clock_format: Some("%H:%M  %a %d %b".to_string()),
            system_refresh_ms: Some(2000),
            show_cpu: Some(true),
            show_ram: Some(true),
            network_interface: None,
            theme_config: None,
            widget_config: WidgetConfig {
                launcher: WidgetVisibility {
                    visible: true,
                    config: None,
                },
                workspaces: WidgetVisibility {
                    visible: true,
                    config: None,
                },
                clock: WidgetVisibility {
                    visible: true,
                    config: None,
                },
                system: WidgetVisibility {
                    visible: true,
                    config: None,
                },
                network: WidgetVisibility {
                    visible: true,
                    config: None,
                },
            },
            keybind_config: KeybindConfig::default(),
            automation_rules: Vec::new(),
            notification_config: NotificationConfig::default(),
            gaming_mode: GamingModeConfig::default(),
            wallpaper_global: WallpaperConfig::default(),
            workspace_wallpapers: HashMap::new(),
            pinned_apps: Vec::new(),
            shell: ShellConfig::default(),
        }
    }
}

impl XarphConfig {
    pub fn load() -> Self {
        ConfigLoader::load_default().unwrap_or_else(|e| {
            eprintln!("Failed to load Xarph config: {e}");
            Self::default()
        })
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        ConfigLoader::load_path(path.as_ref())
    }

    fn load_legacy_file(path: &Path) -> Self {
        if let Ok(contents) = fs::read_to_string(path) {
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self).unwrap_or_default();
        fs::write(path, contents)
    }

    pub fn load_with_themes(manager: &mut ThemeManager) -> std::io::Result<XarphConfig> {
        let mut config = Self::load();
        manager.load_themes()?;
        if let Some(name) = config.theme.clone() {
            manager.set_current_theme(&name);
        }
        Ok(config)
    }

    pub fn get_theme_name(&self) -> Option<String> {
        self.theme.clone()
    }

    pub fn set_theme(&mut self, theme: String) {
        self.theme = Some(theme);
    }

    pub fn get_widget_visibility(&self, widget_id: &str) -> bool {
        match widget_id {
            "launcher" => self.widget_config.launcher.visible,
            "workspaces" => self.widget_config.workspaces.visible,
            "clock" => self.widget_config.clock.visible,
            "system" => self.widget_config.system.visible,
            "network" => self.widget_config.network.visible,
            _ => false,
        }
    }

    pub fn set_widget_visibility(&mut self, widget_id: &str, visible: bool) {
        match widget_id {
            "launcher" => self.widget_config.launcher.visible = visible,
            "workspaces" => self.widget_config.workspaces.visible = visible,
            "clock" => self.widget_config.clock.visible = visible,
            "system" => self.widget_config.system.visible = visible,
            "network" => self.widget_config.network.visible = visible,
            _ => {}
        }
    }

    pub fn get_notification_config(&self) -> NotificationConfig {
        self.notification_config.clone()
    }

    pub fn set_notification_config(&mut self, config: NotificationConfig) {
        self.notification_config = config;
    }

    pub fn get_gaming_mode_config(&self) -> GamingModeConfig {
        self.gaming_mode.clone()
    }

    pub fn set_gaming_mode_config(&mut self, config: GamingModeConfig) {
        self.gaming_mode = config;
    }

    pub fn get_wallpaper_for_workspace(&self, workspace_idx: u8) -> &WallpaperConfig {
        self.workspace_wallpapers
            .get(&workspace_idx)
            .and_then(|ww| ww.wallpaper.as_ref())
            .unwrap_or(&self.wallpaper_global)
    }

    pub fn set_workspace_wallpaper(&mut self, workspace_idx: u8, wallpaper: WallpaperConfig) {
        self.workspace_wallpapers
            .entry(workspace_idx)
            .or_default()
            .wallpaper = Some(wallpaper);
    }

    pub fn clear_workspace_wallpaper(&mut self, workspace_idx: u8) {
        if let Some(ww) = self.workspace_wallpapers.get_mut(&workspace_idx) {
            ww.wallpaper = None;
        }
    }
}

// ── Shell Layout Configuration ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShellConfig {
    #[serde(default = "default_config_version")]
    pub version: u32,
    #[serde(default = "default_metrics_interval_secs")]
    pub metrics_interval_secs: u64,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub panels: Vec<PanelConfig>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            version: default_config_version(),
            metrics_interval_secs: default_metrics_interval_secs(),
            includes: vec!["panels/*.conf".to_string()],
            panels: vec![PanelConfig::default()],
        }
    }
}

fn default_config_version() -> u32 {
    1
}
fn default_metrics_interval_secs() -> u64 {
    2
}
fn default_panel_id() -> String {
    "top".to_string()
}
fn default_panel_size() -> i32 {
    46
}
fn default_panel_spacing() -> i32 {
    6
}
fn default_widget_visible() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PanelConfig {
    #[serde(default = "default_panel_id")]
    pub id: String,
    #[serde(default)]
    pub output: Option<String>,
    #[serde(default)]
    pub position: PanelPosition,
    #[serde(default)]
    pub anchor: PanelAnchor,
    #[serde(default = "default_panel_size")]
    pub size: i32,
    #[serde(default = "default_panel_spacing")]
    pub spacing: i32,
    #[serde(default)]
    pub exclusive_zone: Option<i32>,
    #[serde(default)]
    pub widgets: Vec<PanelWidgetConfig>,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            id: default_panel_id(),
            output: None,
            position: PanelPosition::Top,
            anchor: PanelAnchor::Fill,
            size: default_panel_size(),
            spacing: default_panel_spacing(),
            exclusive_zone: None,
            widgets: default_panel_widgets(),
        }
    }
}

fn default_panel_widgets() -> Vec<PanelWidgetConfig> {
    vec![
        PanelWidgetConfig {
            id: "start".to_string(),
            kind: WidgetKind::StartButton,
            section: PanelSection::Start,
            ..Default::default()
        },
        PanelWidgetConfig {
            id: "workspaces".to_string(),
            kind: WidgetKind::Workspaces,
            section: PanelSection::Start,
            ..Default::default()
        },
        PanelWidgetConfig {
            id: "clock".to_string(),
            kind: WidgetKind::Clock,
            section: PanelSection::Center,
            clock_format: Some("%H:%M".to_string()),
            date_format: Some("%a %d %b".to_string()),
            ..Default::default()
        },
        PanelWidgetConfig {
            id: "network".to_string(),
            kind: WidgetKind::Network,
            section: PanelSection::End,
            ..Default::default()
        },
        PanelWidgetConfig {
            id: "system".to_string(),
            kind: WidgetKind::System,
            section: PanelSection::End,
            ..Default::default()
        },
        PanelWidgetConfig {
            id: "tray".to_string(),
            kind: WidgetKind::Tray,
            section: PanelSection::End,
            ..Default::default()
        },
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PanelPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for PanelPosition {
    fn default() -> Self {
        PanelPosition::Top
    }
}

impl PanelPosition {
    pub fn is_horizontal(self) -> bool {
        matches!(self, PanelPosition::Top | PanelPosition::Bottom)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PanelAnchor {
    Fill,
    Start,
    End,
}

impl Default for PanelAnchor {
    fn default() -> Self {
        PanelAnchor::Fill
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PanelSection {
    Start,
    Center,
    End,
}

impl Default for PanelSection {
    fn default() -> Self {
        PanelSection::Start
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WidgetKind {
    StartButton,
    Launcher,
    Workspaces,
    Clock,
    Network,
    System,
    Tray,
    ConfigButton,
}

impl Default for WidgetKind {
    fn default() -> Self {
        WidgetKind::Clock
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PanelWidgetConfig {
    pub id: String,
    #[serde(default)]
    pub kind: WidgetKind,
    #[serde(default)]
    pub section: PanelSection,
    #[serde(default = "default_widget_visible")]
    pub visible: bool,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub clock_format: Option<String>,
    #[serde(default)]
    pub date_format: Option<String>,
    #[serde(default)]
    pub icon_size: Option<i32>,
    #[serde(default)]
    pub show_cpu: Option<bool>,
    #[serde(default)]
    pub show_ram: Option<bool>,
    #[serde(default)]
    pub network_interface: Option<String>,
    #[serde(default)]
    pub interval_ms: Option<u64>,
    #[serde(default)]
    pub max_visible: Option<usize>,
}

impl Default for PanelWidgetConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            kind: WidgetKind::Clock,
            section: PanelSection::Start,
            visible: true,
            label: None,
            clock_format: None,
            date_format: None,
            icon_size: None,
            show_cpu: None,
            show_ram: None,
            network_interface: None,
            interval_ms: None,
            max_visible: None,
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    DuplicatePanelId(String),
    MissingManifest(PathBuf),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, source } => write!(f, "{}: {source}", path.display()),
            ConfigError::Parse { path, source } => write!(f, "{}: {source}", path.display()),
            ConfigError::DuplicatePanelId(id) => write!(f, "duplicate panel id `{id}`"),
            ConfigError::MissingManifest(path) => {
                write!(f, "missing shell manifest {}", path.display())
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Deserialize)]
struct ShellManifest {
    shell: ShellConfig,
}

#[derive(Debug, Deserialize)]
struct PanelDocument {
    panel: PanelConfig,
    #[serde(default)]
    #[serde(rename = "widget")]
    widgets: Vec<PanelWidgetConfig>,
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_default() -> Result<XarphConfig, ConfigError> {
        let user_dir = config_dir();
        if user_dir.join("shell.conf").exists() {
            return Self::load_dir(&user_dir);
        }

        let legacy = config_path();
        if legacy.exists() {
            return Ok(XarphConfig::load_legacy_file(&legacy));
        }

        let system_dir = system_config_dir();
        if system_dir.join("shell.conf").exists() {
            seed_config_if_missing(&user_dir, &system_dir)?;
            return Self::load_dir(&user_dir);
        }

        let repo_dir = repo_config_dir();
        if repo_dir.join("shell.conf").exists() {
            seed_config_if_missing(&user_dir, &repo_dir)?;
            return Self::load_dir(&user_dir);
        }

        Ok(XarphConfig::default())
    }

    pub fn load_path(path: &Path) -> Result<XarphConfig, ConfigError> {
        if path.is_dir() {
            Self::load_dir(path)
        } else if path.file_name().and_then(|n| n.to_str()) == Some("shell.conf") {
            Self::load_dir(path.parent().unwrap_or_else(|| Path::new(".")))
        } else {
            Ok(XarphConfig::load_legacy_file(path))
        }
    }

    pub fn load_dir(dir: &Path) -> Result<XarphConfig, ConfigError> {
        let manifest_path = dir.join("shell.conf");
        if !manifest_path.exists() {
            return Err(ConfigError::MissingManifest(manifest_path));
        }

        let manifest_text = read_to_string(&manifest_path)?;
        let manifest: ShellManifest =
            toml::from_str(&manifest_text).map_err(|source| ConfigError::Parse {
                path: manifest_path.clone(),
                source,
            })?;

        let mut config = XarphConfig::default();
        let mut shell = manifest.shell;
        let mut panels = Vec::new();
        let mut ids = HashSet::new();

        for include in &shell.includes {
            for path in resolve_include(dir, include) {
                let text = read_to_string(&path)?;
                let mut doc: PanelDocument =
                    toml::from_str(&text).map_err(|source| ConfigError::Parse {
                        path: path.clone(),
                        source,
                    })?;
                if !doc.widgets.is_empty() {
                    doc.panel.widgets = doc.widgets;
                }
                if !ids.insert(doc.panel.id.clone()) {
                    return Err(ConfigError::DuplicatePanelId(doc.panel.id));
                }
                panels.push(doc.panel);
            }
        }

        shell.panels = if panels.is_empty() {
            vec![PanelConfig::default()]
        } else {
            panels
        };
        config.shell = shell;
        Ok(config)
    }
}

fn read_to_string(path: &Path) -> Result<String, ConfigError> {
    fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn resolve_include(base: &Path, pattern: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(prefix) = pattern.strip_suffix("/*.conf") {
        let dir = base.join(prefix);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("conf") {
                    paths.push(path);
                }
            }
        }
    } else {
        let path = base.join(pattern);
        if path.exists() {
            paths.push(path);
        }
    }
    paths.sort();
    paths
}

fn seed_config_if_missing(user_dir: &Path, source_dir: &Path) -> Result<(), ConfigError> {
    if user_dir.join("shell.conf").exists() {
        return Ok(());
    }

    copy_dir_all(source_dir, user_dir)?;
    Ok(())
}

fn copy_dir_all(source: &Path, dest: &Path) -> Result<(), ConfigError> {
    fs::create_dir_all(dest).map_err(|source_err| ConfigError::Io {
        path: dest.to_path_buf(),
        source: source_err,
    })?;

    for entry in fs::read_dir(source).map_err(|source_err| ConfigError::Io {
        path: source.to_path_buf(),
        source: source_err,
    })? {
        let entry = entry.map_err(|source_err| ConfigError::Io {
            path: source.to_path_buf(),
            source: source_err,
        })?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_all(&source_path, &dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent).map_err(|source_err| ConfigError::Io {
                    path: parent.to_path_buf(),
                    source: source_err,
                })?;
            }
            fs::copy(&source_path, &dest_path).map_err(|source_err| ConfigError::Io {
                path: source_path.clone(),
                source: source_err,
            })?;
        }
    }

    Ok(())
}

fn repo_config_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("data/conf")
}

// ── Theme Config ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    pub name: String,
    #[serde(default)]
    pub variant: ThemeVariant,
    pub colors: Option<ThemeColors>,
    pub fonts: Option<ThemeFonts>,
    pub icons: Option<ThemeIcons>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeVariant {
    Light,
    Dark,
    Auto,
}

impl Default for ThemeVariant {
    fn default() -> Self {
        ThemeVariant::Dark
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub surface: String,
    pub error: String,
    pub on_primary: String,
    pub on_secondary: String,
    pub on_background: String,
    pub on_surface: String,
    pub on_error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeFonts {
    pub family: String,
    pub size: f32,
    pub bold_size: f32,
    pub monospace_family: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeIcons {
    pub theme: String,
    pub fallback: String,
}

// ── Widget Config ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetConfig {
    pub launcher: WidgetVisibility,
    pub workspaces: WidgetVisibility,
    pub clock: WidgetVisibility,
    pub system: WidgetVisibility,
    pub network: WidgetVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetVisibility {
    pub visible: bool,
    pub config: Option<WidgetSpecificConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetSpecificConfig {
    pub position: Option<WidgetPosition>,
    pub size: Option<WidgetSize>,
    pub styling: Option<WidgetStyling>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WidgetStyling {
    pub color: String,
    pub background_color: String,
    pub font_size: f32,
    pub border_radius: f32,
}

// ── Keybind Configuration ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeybindConfig {
    pub key_quit: String,
    pub key_lock: String,
    pub key_terminal: String,
    pub key_launcher: String,
    pub key_close_window: String,
    pub key_toggle_floating: String,
    pub key_toggle_overview: String,
    pub key_screenshot: String,
    pub key_workspace_prev: String,
    pub key_workspace_next: String,
    pub key_focus_left: String,
    pub key_focus_right: String,
    pub key_focus_up: String,
    pub key_focus_down: String,
    pub key_move_left: String,
    pub key_move_right: String,
    pub key_move_up: String,
    pub key_move_down: String,
}

impl Default for KeybindConfig {
    fn default() -> Self {
        Self {
            key_quit: "Ctrl+Alt+Backspace".into(),
            key_lock: "Super+Alt+L".into(),
            key_terminal: "Super+T".into(),
            key_launcher: "Super+D".into(),
            key_close_window: "Super+Q".into(),
            key_toggle_floating: "Super+Space".into(),
            key_toggle_overview: "Super+O".into(),
            key_screenshot: "Print".into(),
            key_workspace_prev: "Super+Page_Down".into(),
            key_workspace_next: "Super+Page_Up".into(),
            key_focus_left: "Super+Left".into(),
            key_focus_right: "Super+Right".into(),
            key_focus_up: "Super+Up".into(),
            key_focus_down: "Super+Down".into(),
            key_move_left: "Super+Shift+Left".into(),
            key_move_right: "Super+Shift+Right".into(),
            key_move_up: "Super+Shift+Up".into(),
            key_move_down: "Super+Shift+Down".into(),
        }
    }
}

// ── Automation ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomationRule {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub trigger: AutomationTrigger,
    pub actions: Vec<AutomationAction>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationTrigger {
    OnStartup,
    OnTime { hour: u8, minute: u8 },
    OnWindowEvent { event_type: WindowEventType },
    OnSystemEvent { event_type: SystemEventType },
    OnKeyboardShortcut { keys: Vec<String> },
}

impl Default for AutomationTrigger {
    fn default() -> Self {
        AutomationTrigger::OnStartup
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowEventType {
    WindowOpened,
    WindowClosed,
    WindowFocused,
    WindowMoved,
    WindowResized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    BatteryLevel { level: u8 },
    NetworkConnected,
    NetworkDisconnected,
    AudioVolumeChanged,
    MicrophoneMuted,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomationAction {
    #[serde(default)]
    pub action_type: AutomationActionType,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationActionType {
    LaunchApp { app_id: String },
    ExecuteCommand { command: String },
    SetTheme { theme_name: String },
    ToggleWidget { widget_id: String },
    SendNotification { title: String, body: String },
    LockScreen,
    Sleep,
    Hibernate,
    Custom { action_id: String },
}

impl Default for AutomationActionType {
    fn default() -> Self {
        AutomationActionType::LockScreen
    }
}

// ── Notifications ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub position: NotificationPosition,
    #[serde(default = "default_duration")]
    pub duration: u64,
    #[serde(default = "default_max_notifications")]
    pub max_notifications: u32,
    #[serde(default)]
    pub theme: NotificationTheme,
}

fn default_duration() -> u64 {
    5000
}
fn default_max_notifications() -> u32 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
}

impl Default for NotificationPosition {
    fn default() -> Self {
        NotificationPosition::BottomRight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationTheme {
    System,
    Light,
    Dark,
    Custom { colors: ThemeColors },
}

impl Default for NotificationTheme {
    fn default() -> Self {
        NotificationTheme::System
    }
}

// ── Gaming Mode ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GamingModeConfig {
    pub enabled: bool,
    #[serde(default)]
    pub profile: GamingProfile,
    pub performance: PerformanceSettings,
    pub visual: VisualSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamingProfile {
    Productivity,
    Gaming,
    Custom { name: String },
}

impl Default for GamingProfile {
    fn default() -> Self {
        GamingProfile::Gaming
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceSettings {
    pub target_fps: u32,
    pub vsync: bool,
    pub hardware_acceleration: bool,
    pub gpu_scaling: bool,
    pub reduce_background: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VisualSettings {
    pub reduce_effects: bool,
    pub hide_widgets: bool,
    pub reduce_animations: bool,
    pub high_contrast: bool,
    pub frame_rate_limit: Option<u32>,
}

// ── Theme Management ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub config: ThemeConfig,
    pub metadata: ThemeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeMetadata {
    pub created: String,
    pub modified: String,
    pub tags: Vec<String>,
    pub preview: Option<String>,
}

pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    current_theme: Option<String>,
    theme_dir: PathBuf,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut theme_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
        theme_dir.push("xarph");
        theme_dir.push("themes");
        Self {
            themes: HashMap::new(),
            current_theme: None,
            theme_dir,
        }
    }

    pub fn load_themes(&mut self) -> std::io::Result<()> {
        if !self.theme_dir.exists() {
            fs::create_dir_all(&self.theme_dir)?;
        }
        let entries = fs::read_dir(&self.theme_dir)?;
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file()
                    && entry.path().extension().map_or(false, |ext| ext == "toml")
                {
                    if let Ok(theme) = self.load_theme_from_file(&entry.path()) {
                        self.themes.insert(theme.name.clone(), theme);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn load_theme_from_file(&self, path: &PathBuf) -> std::io::Result<Theme> {
        let contents = fs::read_to_string(path)?;
        let mut theme: Theme = toml::from_str(&contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        if let Ok(metadata) = path.metadata() {
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            theme.metadata.modified = modified;
        }
        Ok(theme)
    }

    pub fn save_theme(&self, theme: &Theme) -> std::io::Result<()> {
        let theme_path = self.theme_dir.join(format!("{}.toml", theme.name));
        let contents = toml::to_string_pretty(theme)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(theme_path, contents)
    }

    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    pub fn get_theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    pub fn set_current_theme(&mut self, name: &str) -> bool {
        if self.themes.contains_key(name) {
            self.current_theme = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.current_theme
            .as_ref()
            .and_then(|name| self.themes.get(name))
    }

    pub fn apply_theme(&self, config: &mut XarphConfig) {
        if let Some(theme) = self.get_current_theme() {
            config.theme = Some(theme.name.clone());
            config.theme_config = Some(theme.config.clone());
        }
    }

    pub fn create_default_theme(&self) -> Theme {
        Theme {
            name: "default".to_string(),
            description: "Default Xarph theme".to_string(),
            version: "1.0.0".to_string(),
            author: "Xarph Team".to_string(),
            config: ThemeConfig {
                name: "default".to_string(),
                variant: ThemeVariant::Dark,
                colors: Some(ThemeColors {
                    primary: "#6200ee".to_string(),
                    secondary: "#03dac6".to_string(),
                    background: "#121212".to_string(),
                    surface: "#1e1e1e".to_string(),
                    error: "#cf6679".to_string(),
                    on_primary: "#ffffff".to_string(),
                    on_secondary: "#000000".to_string(),
                    on_background: "#ffffff".to_string(),
                    on_surface: "#b3b3b3".to_string(),
                    on_error: "#000000".to_string(),
                }),
                fonts: Some(ThemeFonts {
                    family: "Roboto".to_string(),
                    size: 14.0,
                    bold_size: 16.0,
                    monospace_family: "Roboto Mono".to_string(),
                }),
                icons: Some(ThemeIcons {
                    theme: "Papirus".to_string(),
                    fallback: "hicolor".to_string(),
                }),
            },
            metadata: ThemeMetadata {
                created: "2024-01-01".to_string(),
                modified: "2024-01-01".to_string(),
                tags: vec!["default".to_string(), "dark".to_string()],
                preview: None,
            },
        }
    }
}

// ── XarphConfig serde requires theme_manager ────────────────────────────
// We store theme_manager as a separate non-serialized field via a wrapper.
// For simplicity, we skip serializing theme_manager entirely.

impl Serialize for XarphConfig {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("XarphConfig", 17)?;
        state.serialize_field("theme", &self.theme)?;
        state.serialize_field("launcher_grid_size", &self.launcher_grid_size)?;
        state.serialize_field("clock_format", &self.clock_format)?;
        state.serialize_field("system_refresh_ms", &self.system_refresh_ms)?;
        state.serialize_field("show_cpu", &self.show_cpu)?;
        state.serialize_field("show_ram", &self.show_ram)?;
        state.serialize_field("network_interface", &self.network_interface)?;
        state.serialize_field("theme_config", &self.theme_config)?;
        state.serialize_field("widget_config", &self.widget_config)?;
        state.serialize_field("keybind_config", &self.keybind_config)?;
        state.serialize_field("automation_rules", &self.automation_rules)?;
        state.serialize_field("notification_config", &self.notification_config)?;
        state.serialize_field("gaming_mode", &self.gaming_mode)?;
        state.serialize_field("wallpaper_global", &self.wallpaper_global)?;
        state.serialize_field("workspace_wallpapers", &self.workspace_wallpapers)?;
        state.serialize_field("pinned_apps", &self.pinned_apps)?;
        state.serialize_field("shell", &self.shell)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for XarphConfig {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct RawXarphConfig {
            theme: Option<String>,
            launcher_grid_size: Option<u32>,
            clock_format: Option<String>,
            system_refresh_ms: Option<u64>,
            show_cpu: Option<bool>,
            show_ram: Option<bool>,
            network_interface: Option<String>,
            theme_config: Option<ThemeConfig>,
            widget_config: Option<WidgetConfig>,
            keybind_config: Option<KeybindConfig>,
            automation_rules: Option<Vec<AutomationRule>>,
            notification_config: Option<NotificationConfig>,
            gaming_mode: Option<GamingModeConfig>,
            wallpaper_global: Option<WallpaperConfig>,
            workspace_wallpapers: Option<HashMap<u8, WorkspaceWallpaper>>,
            pinned_apps: Option<Vec<String>>,
            shell: Option<ShellConfig>,
        }

        let raw = RawXarphConfig::deserialize(deserializer)?;
        Ok(XarphConfig {
            theme: raw.theme,
            launcher_grid_size: raw.launcher_grid_size,
            clock_format: raw.clock_format,
            system_refresh_ms: raw.system_refresh_ms,
            show_cpu: raw.show_cpu,
            show_ram: raw.show_ram,
            network_interface: raw.network_interface,
            theme_config: raw.theme_config,
            widget_config: raw.widget_config.unwrap_or_default(),
            keybind_config: raw.keybind_config.unwrap_or_default(),
            automation_rules: raw.automation_rules.unwrap_or_default(),
            notification_config: raw.notification_config.unwrap_or_default(),
            gaming_mode: raw.gaming_mode.unwrap_or_default(),
            wallpaper_global: raw.wallpaper_global.unwrap_or_default(),
            workspace_wallpapers: raw.workspace_wallpapers.unwrap_or_default(),
            pinned_apps: raw.pinned_apps.unwrap_or_default(),
            shell: raw.shell.unwrap_or_default(),
        })
    }
}

// ── Utility Functions ───────────────────────────────────────────────────

pub fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("xarph");
    path.push("xarph.toml");
    path
}

pub fn config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    path.push("xarph");
    path.push("conf");
    path
}

pub fn system_config_dir() -> PathBuf {
    PathBuf::from("/usr/share/xarph/conf")
}

pub fn watch_config(callback: impl Fn(XarphConfig) + Send + 'static) {
    watch_config_in(config_dir(), callback);
}

pub fn watch_config_in(path: impl AsRef<Path>, callback: impl Fn(XarphConfig) + Send + 'static) {
    let (tx, rx) = mpsc::channel();
    let watch_path = path.as_ref().to_path_buf();

    thread::spawn(move || {
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<NotifyEvent>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    let _ = tx.send(());
                }
            }
        })
        .expect("Failed to initialize config watcher");

        let _ = fs::create_dir_all(&watch_path);
        let _ = watcher.watch(&watch_path, RecursiveMode::Recursive);

        while rx.recv().is_ok() {
            thread::sleep(std::time::Duration::from_millis(100));
            let new_config = XarphConfig::load();
            callback(new_config);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock went backwards")
            .as_nanos();
        path.push(format!("xarph-config-{label}-{stamp}"));
        fs::create_dir_all(&path).expect("create temp dir");
        path
    }

    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent dirs");
        }
        let mut file = fs::File::create(path).expect("create file");
        file.write_all(content.as_bytes()).expect("write file");
    }

    #[test]
    fn load_dir_uses_sorted_panel_includes() {
        let dir = temp_dir("sorted");
        write_file(
            &dir.join("shell.conf"),
            r#"
[shell]
includes = ["panels/*.conf"]
"#,
        );
        write_file(
            &dir.join("panels/b.conf"),
            r#"
[panel]
id = "b"

[[widget]]
id = "clock"
kind = "clock"
"#,
        );
        write_file(
            &dir.join("panels/a.conf"),
            r#"
[panel]
id = "a"

[[widget]]
id = "clock"
kind = "clock"
"#,
        );

        let config = ConfigLoader::load_dir(&dir).expect("load config");
        let ids: Vec<_> = config
            .shell
            .panels
            .iter()
            .map(|panel| panel.id.as_str())
            .collect();
        assert_eq!(ids, vec!["a", "b"]);
    }

    #[test]
    fn duplicate_panel_ids_are_rejected() {
        let dir = temp_dir("dupe");
        write_file(
            &dir.join("shell.conf"),
            r#"
[shell]
includes = ["panels/*.conf"]
"#,
        );
        write_file(
            &dir.join("panels/one.conf"),
            r#"
[panel]
id = "same"

[[widget]]
id = "clock"
kind = "clock"
"#,
        );
        write_file(
            &dir.join("panels/two.conf"),
            r#"
[panel]
id = "same"

[[widget]]
id = "clock"
kind = "clock"
"#,
        );

        match ConfigLoader::load_dir(&dir) {
            Err(ConfigError::DuplicatePanelId(id)) => assert_eq!(id, "same"),
            _ => panic!("expected duplicate panel id error"),
        }
    }

    #[test]
    fn legacy_xarph_toml_is_still_supported() {
        let dir = temp_dir("legacy");
        let path = dir.join("xarph.toml");
        write_file(
            &path,
            r#"
theme = "retro"
launcher_grid_size = 7
"#,
        );

        let config = XarphConfig::load_from_path(&path).expect("load legacy config");
        assert_eq!(config.theme.as_deref(), Some("retro"));
        assert_eq!(config.launcher_grid_size, Some(7));
        assert_eq!(config.shell.panels.len(), 1);
    }

    #[test]
    fn seeds_missing_user_config_from_source_defaults() {
        let source = temp_dir("source");
        write_file(
            &source.join("shell.conf"),
            r#"
[shell]
includes = ["panels/*.conf"]
"#,
        );
        write_file(
            &source.join("panels/top.conf"),
            r#"
[panel]
id = "top"

[[widget]]
id = "clock"
kind = "clock"
"#,
        );

        let user = temp_dir("user");
        fs::remove_file(user.join("shell.conf")).ok();
        fs::remove_dir_all(&user).ok();
        fs::create_dir_all(&user).expect("recreate user dir");

        seed_config_if_missing(&user, &source).expect("seed config");
        let seeded = ConfigLoader::load_dir(&user).expect("load seeded config");
        assert_eq!(seeded.shell.panels.len(), 1);
        assert_eq!(seeded.shell.panels[0].id, "top");
    }
}
