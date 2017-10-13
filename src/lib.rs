#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct Item {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid: Option<bool>,
}

impl Item {
    pub fn new(title: String) -> Self {
        Item {
            title: title,
            subtitle: None,
            arg: None,
            icon: None,
            valid: None,
        }
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.arg = Some(arg.into());
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn valid(mut self, valid: bool) -> Self {
        self.valid = Some(valid);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Icon {
    pub path: PathBuf,
}
