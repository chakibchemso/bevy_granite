use std::sync::LazyLock;
pub const INTERFACE_CONFIG_TOML: &str = include_str!("../../config/config.toml");

#[derive(Debug, Clone)]
pub struct InputConfig {
    pub fps_camera_speed: f32,
    pub fps_camera_sensitivity: f32,
    pub pan_camera_sensitivity: f32,
    pub obit_camera_sensitivity: f32,
    pub zoom_camera_sensitivity: f32,
    pub zoom_clip_distance: f32,
}

impl InputConfig {
    fn from_toml() -> Self {
        let config: toml::Value = toml::from_str(INTERFACE_CONFIG_TOML)
            .expect("Failed to parse config.toml configuration");

        let popup_section = config
            .get("input")
            .expect("Missing [input] section in config.toml");

        fn get_float(section: &toml::Value, key: &str) -> f32 {
            section.get(key).and_then(|v| v.as_float()).unwrap_or(0.0) as f32
        }

        Self {
            fps_camera_speed: get_float(popup_section, "fps_camera_speed"),
            fps_camera_sensitivity: get_float(popup_section, "fps_camera_sensitivity"),
            pan_camera_sensitivity: get_float(popup_section, "pan_camera_sensitivity"),
            obit_camera_sensitivity: get_float(popup_section, "orbit_camera_sensitivity"),
            zoom_camera_sensitivity: get_float(popup_section, "zoom_camera_sensitivity"),
            zoom_clip_distance: get_float(popup_section, "zoom_clip_distance"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UiConfig {
    pub spacing: f32,
    pub small_spacing: f32,
    pub large_spacing: f32,
}

impl UiConfig {
    fn from_toml() -> Self {
        let config: toml::Value = toml::from_str(INTERFACE_CONFIG_TOML)
            .expect("Failed to parse config.toml configuration");

        let ui_section = config
            .get("ui")
            .expect("Missing [ui] section in config.toml");

        Self {
            spacing: ui_section
                .get("spacing")
                .and_then(|v| v.as_float())
                .map(|f| f as f32)
                .unwrap_or(5.0),
            small_spacing: ui_section
                .get("small_spacing")
                .and_then(|v| v.as_float())
                .map(|f| f as f32)
                .unwrap_or(2.0),
            large_spacing: ui_section
                .get("large_spacing")
                .and_then(|v| v.as_float())
                .map(|f| f as f32)
                .unwrap_or(8.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PopupHelpText {
    pub header: String,
    pub body: String,
    pub youtube_text: String,
    pub youtube_link: String,
}

impl PopupHelpText {
    fn from_toml() -> Self {
        let config: toml::Value = toml::from_str(INTERFACE_CONFIG_TOML)
            .expect("Failed to parse config.toml configuration");

        let popup_section = config
            .get("popup_help_text")
            .expect("Missing [popup_help_text] section in config.toml");

        fn get_string(section: &toml::Value, key: &str) -> String {
            section
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("Error")
                .to_string()
        }

        Self {
            header: get_string(popup_section, "header"),
            body: get_string(popup_section, "body"),
            youtube_text: get_string(popup_section, "youtube_text"),
            youtube_link: get_string(popup_section, "youtube_link"),
        }
    }
}

pub static UI_CONFIG: LazyLock<UiConfig> = LazyLock::new(UiConfig::from_toml);
pub static HELP_CONFIG: LazyLock<PopupHelpText> = LazyLock::new(PopupHelpText::from_toml);
pub static INPUT_CONFIG: LazyLock<InputConfig> = LazyLock::new(InputConfig::from_toml);

pub static INTERFACE_CONFIG: LazyLock<toml::Value> = LazyLock::new(|| {
    toml::from_str(INTERFACE_CONFIG_TOML).expect("Failed to parse config.toml configuration")
});

pub fn get_interface_config(path: &str) -> Option<&'static toml::Value> {
    path.split('.')
        .fold(Some(&*INTERFACE_CONFIG), |current, key| current?.get(key))
}
pub fn get_interface_config_float(path: &str) -> f32 {
    get_interface_config(path)
        .and_then(|config| config.as_float().map(|f| f as f32))
        .unwrap_or(0.0)
}
pub fn get_interface_config_str(path: &str) -> Option<&'static str> {
    get_interface_config(path)?.as_str()
}
