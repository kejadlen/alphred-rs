use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Serialize};
use serde_json::json;

#[derive(Debug)]
pub struct Workflow {
    items: Vec<Item>,
}

impl Workflow {
    pub fn cache() -> Result<PathBuf> {
        let var = env::var("alfred_workflow_cache")?;
        let path = PathBuf::from(var);
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        Ok(path)
    }

    pub fn new(items: &[Item]) -> Self {
        Self { items: items.to_vec() }
    }
}

impl fmt::Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", json!({ "items": self.items }).to_string())
    }
}

#[derive(Clone, Debug, Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<HashMap<String, String>>,
}

impl Item {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Item {
            title: title.into(),
            subtitle: None,
            arg: None,
            icon: None,
            valid: None,
            variables: None,
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

    pub fn variables(mut self, variables: &HashMap<String, String>) -> Self {
        self.variables = Some(variables.clone());
        self
    }
}

#[derive(Clone, Debug, Serialize)]
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
