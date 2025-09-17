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

/// Fidget CLI
struct Fidget {
    interval: u32,
    pick: String,
    frame: usize,
    export: Option<export::FidgetExport>,
    multi_line: Option<bool>,
    quote: char,

    items: HashMap<String, Vec<String>>,
}

impl Fidget {
    fn help (&self) {
        println!("");
        println!(
            "{}{}Fidget{}{}: {}Fidget spinners for the terminal!",
            term::bold(),
            term::fg("#a6e3a1"),

            term::reset(),
            term::fg("#9399b2"),

            term::reset(),
        );

        //|fS "chunk: Arguments"

        println!("");
        println!("Usage,");
        println!("");
        println!(
            "   {}fidget {}<command> {}<name> {}...{}",
            term::fg("#89b4fa"),
            term::fg("#eba0ac"),
            term::fg("#f5c2e7"),
            term::fg("#9399b2"),

            term::reset(),
        );
        println!("");
        println!(
            "   {}{}<command>    {}Either {}show{} or {}output{}.",
            term::bold(),
            term::fg("#eba0ac"),

            term::reset(),

            term::fg("#cba6f7"),
            term::reset(),

            term::fg("#cba6f7"),
            term::reset(),
        );
        println!(
            "   {}{}<name>       {}Fidget name",
            term::bold(),
            term::fg("#f5c2e7"),
            term::reset(),
        );
        println!(
            "   {}{}...          {}Options",
            term::bold(),
            term::fg("#9399b2"),
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
            term::fg("#eba0ac"),
            term::reset(),
        );

        //|fE
    }

    fn export (&self) {
        if self.export.is_none() {
            return;
        }

        let _as = self.export.as_ref().unwrap_or_else(|| &export::FidgetExport::List);
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

            term::fg("#cba6f7"),
            term::fg("#9399b2"),
            term::fg("#89b4fa") + &term::bold(),

            &self.pick,

            term::reset(),
            if !at_end    { term::fg("#cba6f7") } else { term::fg("#9399b2") },
            if !at_start  { term::fg("#cba6f7") } else { term::fg("#9399b2") },
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!("");
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!(
            "{}{:^loader_size$}{}",

            term::reset() + &term::fg("#9399b2"),
            current,
            term::reset(),
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!("");
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!(
            "{:<stat_pad$}{}󰄉 {}Interval: {}ms{} • {}󰕟 {}Current: {}{:0frame_size$}{} • {}󰕬 {}Frames: {}",

            "",

            term::fg("#89b4fa") + &term::bold(),
            term::fg("#9399b2"),

            term::fg("#fab387") + &self.interval.to_string() + &term::fg("#f9e2af"),
            term::fg("#9399b2"),

            term::fg("#f5c2e7") + &term::bold(),
            term::fg("#9399b2"),

            term::fg("#fab387"),
            self.frame + 1,
            term::fg("#9399b2"),

            term::fg("#a6e3a1") + &term::bold(),
            term::fg("#9399b2"),

            term::fg("#cba6f7") + &max.to_string() + &term::reset(),
        );
        execute!(stdout(), MoveDown(1), MoveToColumn(0)).ok();
        print!(
            "{:<keymap_pad$}{}󰌏  {}l{}: {}Delay+, {}h{}: {}Delay-, {}j{}: {}Next, {}k{}; {}Previous, {}q{}: {}Quit",

            "",
            term::fg("#89b4fa"),


            term::fg("#fab387") + &term::bold(),

            term::reset() + &term::fg("#9399b2"),
            term::fg("#a6e3a1"),


            term::fg("#fab387") + &term::bold(),

            term::reset() + &term::fg("#9399b2"),
            term::fg("#a6e3a1"),


            term::fg("#fab387") + &term::bold(),

            term::reset() + &term::fg("#9399b2"),
            term::fg("#a6e3a1"),


            term::fg("#fab387") + &term::bold(),

            term::reset() + &term::fg("#9399b2"),
            term::fg("#a6e3a1"),


            term::fg("#fab387") + &term::bold(),

            term::reset() + &term::fg("#9399b2"),
            term::fg("#a6e3a1"),
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
    let fidgets = json::read_config(config.source);

    let mut fd = Fidget {
        interval: match config.interval {
            Some(v) => v,
            None => 100,
        },

        frame: 0,
        pick: match config.pick {
            Some(v) => v,
            None => "default".to_owned(),
        },
        items: fidgets,

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
        term::fg("#9399b2"),
    );

    fd.export();
    Ok(())
}
