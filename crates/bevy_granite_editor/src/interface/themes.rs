use bevy::platform::collections::HashMap;
use bevy::prelude::Resource;
use bevy_egui::egui;
use bevy_egui::egui::{Stroke, TextStyle};
use egui::Color32;
use serde::{Deserialize, Serialize};

fn default_font_baseline() -> HashMap<SerializableTextStyle, f32> {
    let mut baseline = HashMap::new();
    baseline.insert(SerializableTextStyle::Small, 9.0);
    baseline.insert(SerializableTextStyle::Body, 12.5);
    baseline.insert(SerializableTextStyle::Button, 12.5);
    baseline.insert(SerializableTextStyle::Heading, 18.0);
    baseline.insert(SerializableTextStyle::Monospace, 11.0);
    baseline
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SerializableTextStyle {
    Small,
    Body,
    Button,
    Heading,
    Monospace,
}

impl From<TextStyle> for SerializableTextStyle {
    fn from(ts: TextStyle) -> Self {
        match ts {
            TextStyle::Small => Self::Small,
            TextStyle::Body => Self::Body,
            TextStyle::Button => Self::Button,
            TextStyle::Heading => Self::Heading,
            TextStyle::Monospace => Self::Monospace,
            _ => Self::Body, // fallback if needed
        }
    }
}

impl From<SerializableTextStyle> for TextStyle {
    fn from(s: SerializableTextStyle) -> Self {
        match s {
            SerializableTextStyle::Small => TextStyle::Small,
            SerializableTextStyle::Body => TextStyle::Body,
            SerializableTextStyle::Button => TextStyle::Button,
            SerializableTextStyle::Heading => TextStyle::Heading,
            SerializableTextStyle::Monospace => TextStyle::Monospace,
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Theme {
    Light,
    #[default]
    Dark,
    Warm,
    Blue,
    Transparent,
}

impl Theme {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Light,
            Self::Dark,
            Self::Warm,
            Self::Blue,
            Self::Transparent,
        ]
    }

    pub fn apply_to_context(&self, ctx: &egui::Context) {
        match self {
            Theme::Light => ctx.set_visuals(light_theme()),
            Theme::Dark => ctx.set_visuals(dark_theme()),
            Theme::Warm => ctx.set_visuals(warm_theme()),
            Theme::Blue => ctx.set_visuals(blue_theme()),
            Theme::Transparent => ctx.set_visuals(transparent_theme()),
        }
    }
}

#[derive(Resource, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct ThemeState {
    pub theme: Theme,

    #[serde(skip)]
    pub theme_changed: bool,

    pub font_baseline: HashMap<SerializableTextStyle, f32>,
    pub font_scale: f32,

    #[serde(skip)]
    pub font_scale_changed: bool,

    pub spacing: f32,

    #[serde(skip)]
    pub spacing_changed: bool,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            theme_changed: false,
            font_baseline: default_font_baseline(),
            font_scale: 1.1,
            font_scale_changed: false,
            spacing: 0.0,
            spacing_changed: false,
        }
    }
}

pub fn light_theme() -> egui::Visuals {
    let mut v = egui::Visuals::light();
    // Beige and light tones

    let main = Color32::from_rgb(225, 215, 200); // Slightly darker light beige
    let panel = Color32::from_rgb(210, 195, 180); // Darker beige for panels
    let extreme = Color32::from_rgb(190, 175, 160); // Deeper warm base

    let text = Color32::from_rgb(50, 40, 30); // Dark brown for readability    let stroke_strength = 0.6;

    let stroke_strength = 0.8;
    let stroke = Stroke::new(stroke_strength, extreme);

    // Stroke (sep lines)
    v.window_stroke = stroke;
    v.widgets.noninteractive.bg_stroke = stroke;
    v.widgets.inactive.bg_stroke = stroke;
    v.widgets.active.bg_stroke = stroke;
    v.widgets.hovered.bg_stroke = stroke;

    // Main fills
    v.widgets.noninteractive.bg_fill = main;
    v.widgets.noninteractive.weak_bg_fill = main;
    v.widgets.inactive.bg_fill = main;
    v.widgets.inactive.weak_bg_fill = main;
    v.widgets.active.bg_fill = main;
    v.widgets.active.weak_bg_fill = main;
    v.widgets.hovered.bg_fill = main;
    v.widgets.hovered.weak_bg_fill = main;
    v.selection.bg_fill = main;

    // Panel and window
    v.panel_fill = panel;
    v.window_fill = panel;

    // Darker parts
    v.extreme_bg_color = extreme;

    // Text
    v.override_text_color = Some(text);

    v
}

pub fn dark_theme() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();

    let stroke_width = 0.6;

    visuals.widgets.noninteractive.bg_stroke.width = stroke_width;
    visuals.widgets.noninteractive.fg_stroke.width = stroke_width;
    visuals.widgets.inactive.bg_stroke.width = stroke_width;
    visuals.widgets.inactive.fg_stroke.width = stroke_width;

    visuals
}

pub fn warm_theme() -> egui::Visuals {
    let mut v = egui::Visuals::dark();
    let main = Color32::from_rgb(140, 80, 60); // Earthy orange-red
    let panel = Color32::from_rgb(90, 50, 40); // Muted background
    let extreme = Color32::from_rgb(50, 30, 25); // Very dark brown-red

    let text = Color32::from_rgb(235, 220, 210);
    let stroke_strength = 0.85;

    let stroke = Stroke::new(stroke_strength, extreme);

    // Stroke (separator lines)
    v.window_stroke = stroke;
    v.widgets.noninteractive.bg_stroke = stroke;
    v.widgets.inactive.bg_stroke = stroke;
    v.widgets.active.bg_stroke = stroke;
    v.widgets.hovered.bg_stroke = stroke;

    // Fill (buttons, backgrounds)
    v.widgets.noninteractive.bg_fill = main;
    v.widgets.noninteractive.weak_bg_fill = main;
    v.widgets.inactive.bg_fill = main;
    v.widgets.inactive.weak_bg_fill = main;
    v.widgets.active.bg_fill = main;
    v.widgets.active.weak_bg_fill = main;
    v.widgets.hovered.bg_fill = main;
    v.widgets.hovered.weak_bg_fill = main;
    v.selection.bg_fill = main;

    // Panel and window backgrounds
    v.panel_fill = panel;
    v.window_fill = panel;

    // Extreme background (very dark UI areas)
    v.extreme_bg_color = extreme;

    // Text color
    v.override_text_color = Some(text);

    v
}

pub fn blue_theme() -> egui::Visuals {
    let mut v = egui::Visuals::dark();
    let main = egui::Color32::from_rgb(35, 55, 90);
    let panel = egui::Color32::from_rgb(25, 45, 75);
    let extreme = egui::Color32::from_rgb(15, 30, 55);
    let text = egui::Color32::from_rgb(225, 230, 240);
    let stroke_strength = 0.85;
    let stroke = Stroke::new(stroke_strength, extreme);
    let _transparent_stroke = Stroke::new(0.0, egui::Color32::TRANSPARENT);

    // Stroke (sep lines)
    v.window_stroke = stroke;
    v.widgets.noninteractive.bg_stroke = stroke;
    v.widgets.inactive.bg_stroke = stroke;
    v.widgets.active.bg_stroke = stroke;
    v.widgets.hovered.bg_stroke = stroke;

    // Lighter
    v.widgets.noninteractive.bg_fill = main;
    v.widgets.noninteractive.weak_bg_fill = main;
    v.widgets.inactive.bg_fill = main;
    v.widgets.inactive.weak_bg_fill = main;
    v.widgets.active.bg_fill = main;
    v.widgets.active.weak_bg_fill = main;
    v.widgets.hovered.bg_fill = main;
    v.widgets.hovered.weak_bg_fill = main;
    v.selection.bg_fill = main;

    // Darker
    v.panel_fill = panel;
    v.window_fill = panel;

    // Darkest
    v.extreme_bg_color = extreme;

    // Text
    v.override_text_color = Some(text);

    v
}

pub fn transparent_theme() -> egui::Visuals {
    let mut v = egui::Visuals::dark();

    let bg_transparency = 30;
    let text_color = Color32::GRAY;

    let bg_alpha = ((100 - bg_transparency) * 255) / 100;
    let selected_item_color = Color32::from_rgba_premultiplied(80, 60, 40, bg_alpha as u8);
    let semi_transparent = Color32::from_rgba_premultiplied(0, 0, 0, bg_alpha as u8);

    let no_stroke = Stroke::new(0.0, Color32::TRANSPARENT);

    v.panel_fill = semi_transparent;
    v.window_fill = semi_transparent;
    v.extreme_bg_color = semi_transparent;

    v.selection.bg_fill = selected_item_color;

    v.widgets.active.bg_fill = semi_transparent;
    v.window_stroke = no_stroke;
    v.widgets.noninteractive.bg_stroke = no_stroke;
    v.widgets.inactive.bg_stroke = no_stroke;
    v.widgets.active.bg_stroke = no_stroke;
    v.widgets.noninteractive.fg_stroke = no_stroke;
    v.widgets.active.fg_stroke = no_stroke;
    v.widgets.open.fg_stroke = no_stroke;
    v.override_text_color = Some(text_color);

    v
}
