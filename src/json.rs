use std::{collections::HashMap, fs};
use serde_json;

type SpinnJSON = HashMap<String, Vec<String>>;

pub fn read_config (source: Option<String>) -> SpinnJSON {
    let default_path = format!("{}/spinners.json", env!("CARGO_MANIFEST_DIR"));
    let default_config_txt: String = match fs::read_to_string(default_path) {
        Ok(v) => v,
        Err(_) => "{}".to_owned()
    };
    let mut default: SpinnJSON = match serde_json::from_str(&default_config_txt) {
        Ok(e) => e,
        Err(_) => HashMap::new(),
    };

    match source {
        Some(path) => {
            let source_txt = match fs::read_to_string(path) {
                Ok(v) => v,
                Err(_) => "{}".to_owned()
            };
            let source: SpinnJSON = match serde_json::from_str(&source_txt) {
                Ok(e) => e,
                Err(_) => HashMap::new(),
            };

            default.extend(source);
            return default;
        },
        None => {
            return default;
        }
    }
}
