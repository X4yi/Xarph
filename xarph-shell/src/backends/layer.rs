/// Wayland Layer Shell backend for the shell
/// Uses wayland-client and wayland-protocols for zwlr_layer_shell_v1

#[allow(dead_code)]
pub struct LayerShellBackend {
    pub output_name: Option<String>,
    pub namespace: String,
}

impl LayerShellBackend {
    #[allow(dead_code)]
    pub fn new(output_name: Option<String>, namespace: impl Into<String>) -> Self {
        Self {
            output_name,
            namespace: namespace.into(),
        }
    }

    #[allow(dead_code)]
    pub fn setup_layer_shell(
        &self,
        layer: Layer,
        anchor: Anchor,
        exclusive_zone: i32,
    ) -> LayerConfig {
        LayerConfig {
            layer,
            anchor,
            exclusive_zone,
            namespace: self.namespace.clone(),
            output: self.output_name.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Anchor {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Anchor {
    #[allow(dead_code)]
    pub const TOP: Self = Self { top: true, bottom: false, left: false, right: false };
    #[allow(dead_code)]
    pub const BOTTOM: Self = Self { top: false, bottom: true, left: false, right: false };
    #[allow(dead_code)]
    pub const LEFT: Self = Self { top: false, bottom: false, left: true, right: false };
    #[allow(dead_code)]
    pub const RIGHT: Self = Self { top: false, bottom: false, left: false, right: true };
    #[allow(dead_code)]
    pub const ALL: Self = Self { top: true, bottom: true, left: true, right: true };
    #[allow(dead_code)]
    pub const TOP_LEFT: Self = Self { top: true, bottom: false, left: true, right: false };
    #[allow(dead_code)]
    pub const TOP_RIGHT: Self = Self { top: true, bottom: false, left: false, right: true };
    #[allow(dead_code)]
    pub const BOTTOM_LEFT: Self = Self { top: false, bottom: true, left: true, right: false };
    #[allow(dead_code)]
    pub const BOTTOM_RIGHT: Self = Self { top: false, bottom: true, left: false, right: true };
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LayerConfig {
    pub layer: Layer,
    pub anchor: Anchor,
    pub exclusive_zone: i32,
    pub namespace: String,
    pub output: Option<String>,
}

impl LayerConfig {
    #[allow(dead_code)]
    pub fn for_panel_position(position: &xarph_sdk::config::PanelPosition) -> Self {
        let (anchor, layer, exclusive_zone) = match position {
            xarph_sdk::config::PanelPosition::Top => (Anchor::TOP, Layer::Top, 40),
            xarph_sdk::config::PanelPosition::Bottom => (Anchor::BOTTOM, Layer::Top, 40),
            xarph_sdk::config::PanelPosition::Left => (Anchor::LEFT, Layer::Top, 40),
            xarph_sdk::config::PanelPosition::Right => (Anchor::RIGHT, Layer::Top, 40),
        };
        Self {
            layer,
            anchor,
            exclusive_zone,
            namespace: "xarph-panel".to_string(),
            output: None,
        }
    }

    #[allow(dead_code)]
    pub fn for_desktop() -> Self {
        Self {
            layer: Layer::Background,
            anchor: Anchor::ALL,
            exclusive_zone: -1,
            namespace: "xarph-shell-background".to_string(),
            output: None,
        }
    }
}
