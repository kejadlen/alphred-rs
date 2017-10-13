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
    icon: Icon,
}

impl Item {
    fn new(title: String, icon: Icon) -> Self {
        let subtitle = None;
        let arg = None;
        Item { title, subtitle, arg, icon }
    }

    fn subtitle<'a>(&'a mut self, subtitle: &str) -> &'a mut Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    fn arg<'a>(&'a mut self, arg: &str) -> &'a mut Self {
        self.arg = Some(arg.into());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct Icon {
    pub path: PathBuf,
}
