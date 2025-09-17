#[derive(Debug)]
pub enum SpinnExport {
    List,
    Array,
    String,
}

pub fn export (export_as: &SpinnExport, quote: &char, multiline: &bool, frames: &Vec<String>) {
    match export_as {
        SpinnExport::List => {
            as_list(multiline, quote, frames);
        },
        SpinnExport::Array => {
            as_array(multiline, quote, frames);
        },
        SpinnExport::String => {
            as_string(multiline, quote, frames);
        }
    };

    println!("");
}

pub fn as_list (multiline: &bool, quote: &char, frames: &Vec<String>) {
    if multiline == &true {
        println!("[");
    } else {
        print!("[ ");
    }

    for (f, frame) in frames.iter().enumerate() {
        if multiline == &true {
            println!("\t{}{}{},", quote, frame, quote);
        } else if f < frames.len() - 1 {
            print!("{}{}{}, ", quote, frame, quote);
        } else {
            print!("{}{}{} ", quote, frame, quote);
        }
    }

    if multiline == &true {
        println!("]");
    } else {
        print!("]\n");
    }
}

pub fn as_array (multiline: &bool, quote: &char, frames: &Vec<String>) {
    if multiline == &true {
        println!("{{");
    } else {
        print!("{{ ");
    }

    for (f, frame) in frames.iter().enumerate() {
        if multiline == &true {
            println!("\t{}{}{},", quote, frame, quote);
        } else if f < frames.len() - 1 {
            print!("{}{}{}, ", quote, frame, quote);
        } else {
            print!("{}{}{} ", quote, frame, quote);
        }
    }

    if multiline == &true {
        println!("}}");
    } else {
        print!("}}\n");
    }
}

pub fn as_string (multiline: &bool, _quote: &char, frames: &Vec<String>) {
    for frame in frames {
        if multiline == &true {
            println!("{}", frame);
        } else {
            print!("{} ", frame);
        }
    }
}

