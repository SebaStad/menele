use std::collections::HashMap;
// use std::cell::LazyLock;
use std::sync::{RwLock, LazyLock};

// Enum for all the classes
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum FrontendLabels {
    MainPage,
    MainPageEdit,
    Edit,
    Preview,
    HtmlCode,
    Settings,
    Load,
    Search
}

#[derive(Eq, PartialEq, Debug)]
pub struct LabelsConfig {
    labels_lookup: HashMap<FrontendLabels, String>
}

impl LabelsConfig {
    pub fn new() -> Self {
        let mut labels_lookup = HashMap::new();
        labels_lookup.insert(
            FrontendLabels::MainPage,
            "Hauptseite".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::MainPageEdit,
            "Neu / Editieren".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::Edit,
            "Editieren".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::Preview,
            "Vorschau".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::HtmlCode,
            "HTML-Code".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::Settings,
            "Einstellungen".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::Load,
            "Newsletter laden".to_string(),
        );
        labels_lookup.insert(
            FrontendLabels::Search,
            "Datei suchen".to_string(),
        );

        Self {
            labels_lookup
        }
    }

    pub fn get_label(&self, class: FrontendLabels) -> Option<&String> {
        self.labels_lookup.get(&class)
    }
}

pub static GLOBAL_LABELS: LazyLock<RwLock<LabelsConfig>> = LazyLock::new(|| RwLock::new(LabelsConfig::new()));