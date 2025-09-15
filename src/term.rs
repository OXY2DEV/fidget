// Styler for the terminal
#![allow(dead_code)]

pub fn rgb (r: u32, g: u32, b: u32) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

pub fn bg (hex: &str) -> String {
    let inner = hex.trim_start_matches("#");

    let r: u32;
    let g: u32;
    let b: u32;

    match inner.chars().count() {
        3 => {
            r = u32::from_str_radix(&inner[0..1], 16).expect("Failed to get value of `r`");
            g = u32::from_str_radix(&inner[1..2], 16).expect("Failed to get value of `g`");
            b = u32::from_str_radix(&inner[2..3], 16).expect("Failed to get value of `b`");
        },
        6 => {
            r = u32::from_str_radix(&inner[0..2], 16).expect("Failed to get value of `r`");
            g = u32::from_str_radix(&inner[2..4], 16).expect("Failed to get value of `g`");
            b = u32::from_str_radix(&inner[4..6], 16).expect("Failed to get value of `b`");
        },

        _ => {
            r = 255;
            g = 255;
            b = 255;
        }
    }

    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

pub fn fg (hex: &str) -> String {
    let inner = hex.trim_start_matches("#");

    let r: u32;
    let g: u32;
    let b: u32;

    match inner.chars().count() {
        3 => {
            r = u32::from_str_radix(&inner[0..1], 16).expect("Failed to get value of `r`");
            g = u32::from_str_radix(&inner[1..2], 16).expect("Failed to get value of `g`");
            b = u32::from_str_radix(&inner[2..3], 16).expect("Failed to get value of `b`");
        },
        6 => {
            r = u32::from_str_radix(&inner[0..2], 16).expect("Failed to get value of `r`");
            g = u32::from_str_radix(&inner[2..4], 16).expect("Failed to get value of `g`");
            b = u32::from_str_radix(&inner[4..6], 16).expect("Failed to get value of `b`");
        },

        _ => {
            r = 255;
            g = 255;
            b = 255;
        }
    }

    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

pub fn reset () -> String { "\x1b[0m".to_string() }
pub fn bold () -> String { "\x1b[1m".to_string() }

/// Hello
pub fn italic () -> String { "\x1b[3m".to_string() }
