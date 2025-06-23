#[derive(Serialize, Deserialize, Debug)]
pub struct Macros {
    pub keys: String,
    pub action: String,
}

impl Macros {
    pub fn new(keys: String, action: String) -> Self {
        Self { keys, action }
    }
}

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use directories::ProjectDirs;

pub fn read_macros(file_path: &PathBuf) -> io::Result<Vec<Macros>> {
    let text = fs::read_to_string(file_path)?;
    let macros: Vec<Macros> =
        serde_json::from_str(&text).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    Ok(macros)
}

pub fn get_config_file(file_name: &str) -> io::Result<PathBuf> {
    // Lin: /home/<username>/.config/mach
    // Win: C:\Users\<username>\AppData\Roaming\mach\config
    // Mac: \Users\Alice\Library\Application Support\mach
    let proj = ProjectDirs::from("", "", "mach").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine project directory",
        )
    })?;

    let config_dir = proj.config_dir();
    fs::create_dir_all(config_dir)?;

    Ok(config_dir.join(format!("{}", file_name)))
}

pub fn write_default_macros(path: &PathBuf) -> io::Result<Vec<Macros>> {
    if path.exists() {
        return read_macros(path);
    }
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let defaults = vec![Macros::new("Ctrl+N".into(), "New Tab".into())];

    let json = serde_json::to_string_pretty(&defaults)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    fs::write(path, json)?;
    Ok(defaults)
}
