use crate::export;

#[derive(Debug)]
pub struct SpinnConfig {
    pub show_help: Option<bool>,
    pub export_as: Option<export::SpinnExport>,
    pub multi_line: Option<bool>,
    pub quote: Option<char>,

    pub source: Option<String>,
    pub pick: Option<String>,
    pub interval: Option<u32>,
}

pub fn get_config () -> SpinnConfig {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let mut config: SpinnConfig = SpinnConfig {
        show_help: None,
        export_as: None,
        multi_line: None,
        quote: None,

        source: None,
        pick: None,
        interval: None,
    };
    let mut position = 0;

    while position < args.len() {
        let item = args[position].to_owned();

        if item.starts_with("--") && item.contains('=') {
            let parts: Vec<&str> = item.trim_start_matches("--").split('=').collect();

            if parts[0] == "interval" {
                config.interval = match parts[1].parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
            } else if parts[0] == "source" {
                config.source = Some(parts[1].to_owned());
            } else if parts[0] == "multiline" {
                config.multi_line = match parts[1].parse::<bool>() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
            } else if parts[0] == "quote" {
                config.quote = match parts[1].parse::<char>() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
            } else if parts[0] == "export" {
                match parts[1] {
                    "list" => {
                        config.export_as = Some(export::SpinnExport::List);
                    },
                    "array" => {
                        config.export_as = Some(export::SpinnExport::Array);
                    },
                    "string" => {
                        config.export_as = Some(export::SpinnExport::String);
                    },
                    _ => {}
                };

                // config.export_as = Some(parts[1].to_owned());
            }
        } else if item.starts_with("-") && item.contains('=') {
            let parts: Vec<&str> = item.trim_start_matches("-").split('=').collect();

            if parts[0] == "i" {
                config.interval = match parts[1].parse::<u32>() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
            } else if parts[0] == "s" {
                config.source = Some(parts[1].to_owned());
            } else if parts[0] == "m" {
                config.multi_line = match parts[1].parse::<bool>() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
            } else if parts[0] == "q" {
                config.quote = match parts[1].parse::<char>() {
                    Ok(v) => Some(v),
                    Err(_) => None
                };
            } else if parts[0] == "e" {
                match parts[1] {
                    "l" => {
                        config.export_as = Some(export::SpinnExport::List);
                    },
                    "a" => {
                        config.export_as = Some(export::SpinnExport::Array);
                    },
                    "s" => {
                        config.export_as = Some(export::SpinnExport::String);
                    },
                    _ => {}
                };
            }
        } else if item == "--help" || item == "-h" {
            config.show_help = Some(true);
            break;
        } else {
            config.pick = Some(item.to_owned());
        }

        position += 1;
    }

    config
}
