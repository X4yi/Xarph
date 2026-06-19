/// Windowed backend for debugging (no layer shell)

#[allow(dead_code)]
pub struct WindowedBackend {
    pub title: String,
    pub width: i32,
    pub height: i32,
}

impl WindowedBackend {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            title: "Xarph Shell - Windowed Mode".to_string(),
            width: 900,
            height: 40,
        }
    }

    #[allow(dead_code)]
    pub fn for_horizontal_panel() -> Self {
        Self {
            title: "Xarph Shell - Windowed Mode".to_string(),
            width: 900,
            height: 40,
        }
    }

    #[allow(dead_code)]
    pub fn for_vertical_panel() -> Self {
        Self {
            title: "Xarph Shell - Windowed Mode".to_string(),
            width: 40,
            height: 700,
        }
    }

    #[allow(dead_code)]
    pub fn for_desktop() -> Self {
        Self {
            title: "Xarph Shell - Desktop (Windowed)".to_string(),
            width: 1920,
            height: 1080,
        }
    }
}

impl Default for WindowedBackend {
    fn default() -> Self {
        Self::new()
    }
}
