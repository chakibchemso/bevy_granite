// For identifying a type into a larger group
// Mainly useful for UI dropdowns, etc.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum ClassCategory {
    Mesh,
    Gameplay,
    Light,
    Empty,
    UI,
    Unknown,
}

impl ClassCategory {
    pub fn get_friendly_name(&self) -> String {
        match self {
            Self::Mesh => "Mesh".to_string(),
            Self::Gameplay => "Gameplay".to_string(),
            Self::Light => "Light".to_string(),
            Self::Empty => "Empty".to_string(),
            Self::UI => "UI".to_string(),
            Self::Unknown => "Unknown".to_string(),
        }
    }
    // How each category should be ordered for UI elements when looping through all
    fn order_value(&self) -> u8 {
        match self {
            ClassCategory::Mesh => 0,
            ClassCategory::Light => 1,
            ClassCategory::Gameplay => 2,
            ClassCategory::Empty => 3,
            ClassCategory::UI => 4,
            ClassCategory::Unknown => 5,
        }
    }
}

impl Ord for ClassCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_value().cmp(&other.order_value())
    }
}

impl PartialOrd for ClassCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
