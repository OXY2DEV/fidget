use crossterm::{
    cursor::{Hide, MoveToColumn, MoveUp, Show}, 
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType}
};
use std::{io::{stdout, Write}, time::Duration};

mod term;

/// Show the help message.
fn print_help () {
    println!("");
    println!(
        "{}{}Fidget{}{}: {}Fidget spinners for the terminal!",
        term::bold(),
        term::term_hex("#a6e3a1"),

        term::reset(),
        term::term_hex("#9399b2"),

        term::reset(),
    );

    //|fS "chunk: Arguments"

    println!("");
    println!("Usage,");
    println!("");
    println!(
        "   {}fidget {}<command> {}<name> {}...{}",
        term::term_hex("#89b4fa"),
        term::term_hex("#eba0ac"),
        term::term_hex("#f5c2e7"),
        term::term_hex("#9399b2"),

        term::reset(),
    );
    println!("");
    println!(
        "   {}{}<command>    {}Either {}show{} or {}output{}.",
        term::bold(),
        term::term_hex("#eba0ac"),

        term::reset(),

        term::term_hex("#cba6f7"),
        term::reset(),

        term::term_hex("#cba6f7"),
        term::reset(),
    );
    println!(
        "   {}{}<name>       {}Fidget name",
        term::bold(),
        term::term_hex("#f5c2e7"),
        term::reset(),
    );
    println!(
        "   {}{}...          {}Options",
        term::bold(),
        term::term_hex("#9399b2"),
        term::reset(),
    );

    //|fE

    //|fS "chunk: Fidget names"

    println!("");
    println!("Fidget names,");
    println!("");

    println!(
        "   {}{}default      {}Basic spinner",
        term::bold(),
        term::term_hex("#eba0ac"),
        term::reset(),
    );

    //|fE
}

//////////////////////////////////////////////////////////////////////////////

fn draw (name: &str) {
    fn clear () {
        execute!(
            stdout(),

            MoveUp(2),
            Clear(ClearType::FromCursorDown)
        ).ok();
    }

    fn draw (name: &str, states: &Vec<&str>, mut index: u32, update_delay: u32) -> u32 {
        let mut _index = (index + 1) as usize;

        if _index >= states.len() {
            index = 0;
            _index = 0_usize;
        } else {
            index += 1;
        }

        execute!(stdout(),MoveToColumn(0)).ok();
        println!(
            "{}Style: {}{}{}{}{}    Frame: {}{}{}{}{}    Total: {}{}{}{}{}    Update: {}{}{}ms{}",

            term::term_hex("#9399b2"),

            term::term_hex("#a6e3a1"),
            term::bold(),
            &name,
            term::reset(),
            term::term_hex("#9399b2"),

            term::term_hex("#fab387"),
            term::bold(),
            &index,
            term::reset(),
            term::term_hex("#9399b2"),

            term::term_hex("#89b4fa"),
            term::bold(),
            states.len(),
            term::reset(),
            term::term_hex("#9399b2"),

            term::term_hex("#89dceb"),
            update_delay,
            term::bold(),
            term::reset(),
        );

        execute!(stdout(), MoveToColumn(0)).ok();
        println!(
            "{}╰──╴{}{}{}",

            term::term_hex("#9399b2"),
            term::term_hex("#cba6f7"),
            states[_index],
            term::reset(),
        );

        index
    }

    let states = match name {
        "default" => vec![ "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", " " ],
        "loader" => vec![ "[----]", "[=---]", "[==--]", "[===-]", "[====]" ],
        _ => vec![]
    };

    if states.len() == 0 {
        return;
    }

    execute!(stdout(), Hide).ok();
    println!("");

    let mut update_delay = 100;
    let mut frame = draw(name, &states, 0, update_delay);

    loop {
        if event::poll(Duration::from_millis(update_delay as u64)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('d') && key.modifiers == KeyModifiers::CONTROL {
                    execute!(stdout(), Show).ok();
                    return;
                } else if key.code == KeyCode::Char('q') {
                    execute!(stdout(), Show).ok();
                    return;
                } else if key.code == KeyCode::Char('h') && update_delay > 100 {
                    update_delay -= 50;

                    clear();
                    frame = draw(name, &states, frame, update_delay);
                } else if key.code == KeyCode::Char('l') && update_delay < 1000 {
                    update_delay += 50;

                    clear();
                    frame = draw(name, &states, frame, update_delay);
                } else {
                    clear();
                    frame = draw(name, &states, frame, update_delay);
                }
            }
        } else {
            clear();
            frame = draw(name, &states, frame, update_delay);
        }
    }
}

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut args = std::env::args();
    let fidget_name: String = match args.nth(1) {
        Some(n) => n,
        None => String::from("")
    };

    if fidget_name == "" {
        print_help();
    } else {
        draw(&fidget_name);
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
