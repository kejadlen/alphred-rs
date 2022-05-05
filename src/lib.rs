use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct Workflow {
    items: Vec<Item>,
}

impl Workflow {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> Result<Vec<Item>>,
    {
        let items = f().unwrap_or_else(|err| {
            let mut items: Vec<_> = err
                .chain()
                .map(|cause| Item::new(cause.to_string()))
                .collect();
            if let Ok(error_icon) = error_icon() {
                items[0].icon = Some(error_icon);
            }
            items
        });

        Workflow { items }
    }
}

impl fmt::Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", json!({ "items": self.items }))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
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
            uid: None,
            title: title.into(),
            subtitle: None,
            arg: None,
            icon: None,
            valid: None,
            variables: None,
        }
    }

    pub fn uid(mut self, uid: &str) -> Self {
        self.uid = Some(uid.into());
        self
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

    pub fn variables(mut self, variables: HashMap<String, String>) -> Self {
        self.variables = Some(variables);
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

fn cache_path() -> Result<PathBuf> {
    let var = env::var("alfred_workflow_cache")?;
    let path = PathBuf::from(var);
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn cached<F>(file_name: &str, f: F) -> Result<PathBuf>
where
    F: Fn() -> Result<Vec<u8>>,
{
    let file_path = cache_path()?.join(file_name);
    if file_path.exists() {
        return Ok(file_path);
    }

    let data = f()?;
    let mut file = fs::File::create(file_path.clone())?;
    file.write_all(&data)?;
    Ok(file_path)
}

fn error_icon() -> Result<Icon> {
    let error_png = include_bytes!("error.png");

    let path_buf = cached(".alphred.error", || Ok(error_png.to_vec()))?;
    Ok(path_buf.as_path().into())
}
