#[derive(Debug)]
pub struct FidgetConfig {
    pub show_help: Option<bool>,

    pub source: Option<String>,
    pub pick: Option<String>,
    pub interval: Option<u32>,
    pub name: Option<String>
}

pub fn get_config () -> FidgetConfig {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let mut config: FidgetConfig = FidgetConfig {
        show_help: None,

        source: None,
        pick: None,
        interval: None,
        name: None,
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
            } else if parts[0] == "pick" {
                config.pick = Some(parts[1].to_owned());
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
            } else if parts[0] == "p" {
                config.pick = Some(parts[1].to_owned());
            }
        } else if item == "--help" || item == "-h" {
            config.show_help = Some(true);
            break;
        } else if config.name == None {
            config.name = Some(item.to_owned());
        }

        position += 1;
    }

    config
}
