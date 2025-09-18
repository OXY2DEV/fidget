use crossterm::{
    cursor::{self, Hide, MoveDown, MoveToColumn, MoveUp, Show}, 
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType}
};
use std::{collections::HashMap, io::stdout, time::Duration};

mod term;
mod args;
mod json;
mod export;

/// Spinn CLI
struct Spinn {
    interval: u32,
    pick: String,
    frame: usize,
    export: Option<export::SpinnExport>,
    multi_line: Option<bool>,
    quote: char,

    items: HashMap<String, Vec<String>>,
}

impl Spinn {
    fn help (&self) {
        println!("");
        println!(
            "{} {}",
            term::color(35) + env!("CARGO_PKG_NAME"),
            term::bold() + &term::color(33) + "v" + env!("CARGO_PKG_VERSION") + &term::reset(),
        );
        println!(
            "{}Spinners for the terminal!{}",
            term::bold() + &term::color(2),
            term::reset(),
        );

        println!("");
        println!(
            "{}Usage:{} {}spinn-rs {}<args>",
            term::underlined() + &term::color(32),
            term::reset(),

            term::bold() + &term::color(34),
            term::reset() + &term::color(31),
        );

        println!("");
        println!(
            "{}Arguments:{}",
            term::underlined() + &term::color(32),
            term::reset(),
        );
        println!("");

        let arg_col_size = 20;
        let mut args = Vec::new();

        args.push(
            (
                format!(
                    "{}--export={}<as>{}",
                    term::color(33),
                    term::color(36),
                    term::reset() + &" ".repeat(arg_col_size - 13),
                ),
                format!(
                    "Export format. See {}Expprt options{}.",
                    term::underlined() + &term::color(32),
                    term::reset() + &term::color(97),
                )
            )
        );
        args.push(
            (
                format!(
                    "{}--interval={}<ms>{}",
                    term::color(33),
                    term::color(36),
                    term::reset() + &" ".repeat(arg_col_size - 15),
                ),
                format!(
                    "Interval between each frame in {}miliseconds{}.",
                    term::color(33),
                    term::reset() + &term::color(97),
                )
            )
        );
        args.push(
            (
                format!(
                    "{}--multiline={}<bool>{}",
                    term::color(33),
                    term::color(36),
                    term::reset() + &" ".repeat(arg_col_size - 18),
                ),
                format!(
                    "Whether to export the output in {}multiple lines{}.",
                    term::color(33),
                    term::reset() + &term::color(97),
                )
            )
        );
        args.push(
            (
                format!(
                    "{}--quote={}<char>{}",
                    term::color(33),
                    term::color(36),
                    term::reset() + &" ".repeat(arg_col_size - 14),
                ),
                format!(
                    "Text to use for {}quoting{} strings when exporting.",
                    term::color(33),
                    term::reset() + &term::color(97),
                )
            )
        );
        args.push(
            (
                format!(
                    "{}--source={}<path>{}",
                    term::color(33),
                    term::color(36),
                    term::reset() + &" ".repeat(arg_col_size - 15),
                ),
                format!(
                    "Path to a {}JSON{} file containing spinners.",
                    term::color(33),
                    term::reset() + &term::color(97),
                )
            )
        );

        for (k, v) in args {
            println!("  {}    {}{}", k, term::color(97), v);
        }

        println!("");
        println!(
            "{}Spiners:{}",
            term::underlined() + &term::color(32),
            term::reset(),
        );
        println!("");

        for (name, frames) in &self.items {
            let max = frames.len() as f32;

            let _mid: f32 = max / 2.0;
            let mid = _mid.floor() as usize;

            let as_text = format!("{}", frames[mid]);

            println!(
                "  {}{:<arg_col_size$}    {}{}",

                term::color(33),
                name,

                term::color(97),
                as_text
            );
        }

        println!("");
        println!(
            "{}Export options:{}",
            term::underlined() + &term::color(32),
            term::reset(),
        );
        println!("");

        let mut export_format = Vec::new();
        export_format.push(
            ( "array", "{ \"a\", \"b\", \"c\" }" )
        );
        export_format.push(
            ( "list", "[ \"a\", \"b\", \"c\" ]" )
        );
        export_format.push(
            ( "string", "a b c" )
        );

        for (k, v) in export_format {
            println!("  {}{:<arg_col_size$}    {}{}", term::color(33), k, term::color(97), v);
        }

    }

    fn export (&self) {
        if self.export.is_none() {
            return;
        }

        let _as = self.export.as_ref().unwrap_or_else(|| &export::SpinnExport::List);
        let _ml = self.multi_line.as_ref().unwrap_or_else(|| &true);

        export::export(_as, &self.quote, _ml, &self.items[&self.pick]);
    }

    fn clear_output (&self) {
        execute!(
            stdout(),

            MoveUp(6),
            Clear(ClearType::FromCursorDown),
        ).ok();
    }

    fn _layouts (&self) -> Vec<String> {
        let mut keys: Vec<String> = vec![];

        for key in self.items.keys() {
            keys.push(key.into());
        }

        keys.sort();
        keys
    }

    fn next_loader (&mut self) {
        let keys: Vec<String> = self._layouts();
        let current_index = keys.iter().position(|s| s == &self.pick).expect("");

        let _c_index = (current_index + 1) as usize;

        if _c_index < keys.len() {
            self.pick = keys[current_index + 1].to_owned();
        } else {
            self.pick = keys[0].to_owned();
        }
    }

    fn prev_loader (&mut self) {
        let keys: Vec<String> = self._layouts();
        let current_index = keys.iter().position(|s| s == &self.pick).expect("");

        let _c_index = (current_index + 1) as usize;

        if _c_index > 1 {
            self.pick = keys[current_index - 1].to_owned();
        } else {
            self.pick = keys[keys.len() - 1].to_owned();
        }
    }

    fn _show_indicator (&self) -> (bool, bool) {
        let keys: Vec<String> = self._layouts();

        let mut at_start = false;
        let mut at_end = false;

        let mut _current_index = 1;

        for key in keys.iter() {
            if key == &self.pick {
                if _current_index == 1 {
                    at_start = true;
                } else if _current_index as usize == keys.len() {
                    at_end = true;
                }

                break;
            }

            _current_index += 1;
        }

        (at_start, at_end)
    }

    fn next_frame (&mut self) {
        let frames = &self.items[&self.pick];
        let max = frames.len();

        let current = &frames[self.frame];
        let terminal_w: u16 = match terminal::size() {
            Ok(w) => w.0,
            Err(_) => 80
        };
        let name_size = (terminal_w - (3 + 5 + 7)) as usize;
        let (at_start, at_end) = self._show_indicator();
        let loader_size = terminal_w as usize;

        let _frame_count: String = frames.len().to_string();
        let frame_size = _frame_count.chars().count();

        let stat = format!(
            "󰄉 Interval: {}ms • 󰕟 Current: {:0frame_size$} • 󰕬 Frames: {}",

            self.interval,
            self.frame + 1,
            max
        );
        let stat_len = stat.chars().count() as u16;
        let _stat_pad = ((terminal_w - stat_len) / 2) as f32;
        let stat_pad = _stat_pad.floor() as usize;

        let keymap_len = "󰌏  l: Delay+, h: Delay-, j: Next, k; Previous, q: Quit".chars().count() as u16;
        let _keymap_pad = ((terminal_w - keymap_len) / 2) as f32;
        let keymap_pad = _keymap_pad.floor() as usize;

        execute!(stdout(), MoveToColumn(0)).ok();
        print!(
            "{} 󰢵 {}Style: {}{:<name_size$}{} {}󰸽 {}󰹁 ",

            term::color(32),
            term::color(97),
            term::color(34) + &term::bold(),

            &self.pick,

            term::reset(),
            if !at_end    { term::color(32) } else { term::color(97) },
            if !at_start  { term::color(32) } else { term::color(97) },
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!("");
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!(
            "{}{:^loader_size$}{}",

            term::reset() + &term::color(97),
            current,
            term::reset(),
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!("");
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!(
            "{:<stat_pad$}{}󰄉 {}Interval: {}ms{} • {}󰕟 {}Current: {}{:0frame_size$}{} • {}󰕬 {}Frames: {}",

            "",

            term::color(34) + &term::bold(),
            term::color(97),

            term::color(36) + &self.interval.to_string() + &term::color(33),
            term::color(97),

            term::color(35) + &term::bold(),
            term::color(97),

            term::color(36),
            self.frame + 1,
            term::color(97),

            term::color(32) + &term::bold(),
            term::color(97),

            term::color(37) + &max.to_string() + &term::reset(),
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!(
            "{:<keymap_pad$}{}󰌏  {}l{}: {}Delay+, {}h{}: {}Delay-, {}j{}: {}Next, {}k{}; {}Previous, {}q{}: {}Quit",

            "",
            term::color(34),


            term::color(36) + &term::bold(),

            term::reset() + &term::color(97),
            term::color(32),


            term::color(36) + &term::bold(),

            term::reset() + &term::color(97),
            term::color(32),


            term::color(36) + &term::bold(),

            term::reset() + &term::color(97),
            term::color(32),


            term::color(36) + &term::bold(),

            term::reset() + &term::color(97),
            term::color(32),


            term::color(36) + &term::bold(),

            term::reset() + &term::color(97),
            term::color(32),
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();

        if self.frame + 1 < max {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    fn show (&mut self) {
        println!("");
        execute!(stdout(), cursor::SavePosition, Hide).ok();

        let mut interval: u64 = self.interval as u64;

        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");

        self.clear_output();
        self.next_frame();

        loop {
            //|fS "chunk: Main loop"

            if event::poll(Duration::from_millis(interval)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.code == KeyCode::Char('d') && key.modifiers == KeyModifiers::CONTROL {
                        execute!(stdout(), Show).ok();
                        self.clear_output();
                        return;
                    } else if key.code == KeyCode::Char('q') {
                        execute!(stdout(), Show).ok();
                        self.clear_output();
                        return;
                    } else if key.code == KeyCode::Char('h') && interval > 100 {
                        self.interval -= 50;
                        interval -= 50;

                        self.clear_output();
                        self.next_frame();
                    } else if key.code == KeyCode::Char('j') {
                        self.next_loader();
                        self.clear_output();

                        self.frame = 0_usize;
                        self.next_frame();
                    } else if key.code == KeyCode::Char('k') {
                        self.prev_loader();
                        self.clear_output();

                        self.frame = 0_usize;
                        self.next_frame();
                    } else if key.code == KeyCode::Char('l') && interval < 1000 {
                        self.interval += 50;
                        interval += 50;

                        self.clear_output();
                        self.next_frame();
                    } else {
                        self.clear_output();
                        self.next_frame();
                    }
                }
            } else {
                self.clear_output();
                self.next_frame();
            }

            //|fE
        }
    }
}

fn main() -> std::io::Result<()> {
    let config = args::get_config();
    let spinners = json::read_config(config.source);

    let mut fd = Spinn {
        interval: match config.interval {
            Some(v) => v,
            None => 100,
        },

        frame: 0,
        pick: match config.pick {
            Some(v) => v,
            None => "default".to_owned(),
        },
        items: spinners,

        export: config.export_as,
        multi_line: config.multi_line,
        quote: match config.quote {
            Some(v) => v,
            None => '"',
        }
    };

    if config.show_help != None {
        fd.help();
    } else {
        terminal::enable_raw_mode()?;
        fd.show();
        terminal::disable_raw_mode()?;
    }

    print!(
        "{}",
        term::color(97),
    );

    fd.export();
    Ok(())
}
