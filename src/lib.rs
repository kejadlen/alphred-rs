#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};

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
    pub fn new<S: Into<String>>(title: S) -> Self {
        Item {
            title: title.into(),
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

    pub fn icon<I: Into<Icon>>(mut self, icon: I) -> Self {
        self.icon = Some(icon.into());
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

impl<'a> From<&'a str> for Icon {
    fn from(s: &str) -> Self {
        let path = s.into();
        Self { path }
    }
}

impl<'a> From<&'a Path> for Icon {
    fn from(p: &Path) -> Self {
        let path = p.into();
        Self { path }
    }
}
