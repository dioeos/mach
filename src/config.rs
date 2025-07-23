use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use directories::ProjectDirs;

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

pub fn load_macros(file_name: &str) -> io::Result<Vec<Macros>> {
    //gets file, read, handles error
    let file_path: PathBuf = get_config_file(file_name)?;
    match read_macros(&file_path) {
        Ok(json_macros) => Ok(json_macros),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let defaults = define_default_macros();
            write_macros(&file_path, &defaults)?;
            Ok(defaults)
        }
        Err(e) => Err(e),
    }
}

fn read_macros(file_path: &PathBuf) -> io::Result<Vec<Macros>> {
    let text = fs::read_to_string(file_path)?;
    let macros: Vec<Macros> =
        serde_json::from_str(&text).map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    Ok(macros)
}

fn get_config_file(file_name: &str) -> io::Result<PathBuf> {
    // Lin: /home/<username>/.config/mach
    // Win: C:\Users\<username>\AppData\Roaming\mach\config
    // Mac: \Users\<username>\Library\Application Support\mach
    let proj = ProjectDirs::from("", "", "mach").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine project directory",
        )
    })?;

    let config_dir = proj.config_dir();
    fs::create_dir_all(config_dir)?;

    Ok(config_dir.join(file_name))
}

fn write_macros(path: &PathBuf, macros: &[Macros]) -> io::Result<()> {
    let json_content = serde_json::to_string_pretty(macros)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, json_content)
}

fn define_default_macros() -> Vec<Macros> {
    vec![
        Macros::new("Alt + /".into(), "Open MACH".into()),
        Macros::new("Alt + /".into(), "Hide MACH".into()),
        Macros::new("Ctrl+N".into(), "New browser tab".into()),
    ]
}
