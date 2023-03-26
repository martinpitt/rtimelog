// Copyright (C) 2022 Martin Pitt <martin@piware.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process;

use chrono::prelude::*;
use rustyline::{error::ReadlineError, Editor};

use rtimelog::commands::{Command, TimeMode};
use rtimelog::store::Timelog;

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn get_input(rl: &mut Editor<()>) -> Result<String, ReadlineError> {
    match rl.readline("> ") {
        Ok(mut line) => {
            line.truncate(line.trim_end().len());
            Ok(line)
        }
        // ^C: like in a shell, abort the current input
        Err(ReadlineError::Interrupted) => Ok("".to_string()),
        // ^D: like in a shell, exit
        Err(ReadlineError::Eof) => Ok(":q".to_string()),
        Err(e) => Err(e),
    }
}

fn show_help() {
    println!(
        "
:w - switch to weekly mode
:d - switch to daily mode
:q - quit
:h - show this help
:e - open timelog.txt in $EDITOR
^r - history search (like in bash) through currently shown activities

Any other input is the description of a task that you just finished."
    );
}

fn show(timelog: &Timelog, mode: &TimeMode, rl_editor: &mut Editor<()>) {
    clear_screen();
    let today = Local::now().date_naive();
    let entries = match mode {
        TimeMode::Day => {
            println!("Work done today {}:", timelog.get_today_as_string());
            timelog.get_n_days(&today, 1)
        }
        TimeMode::Week => {
            println!("Work done this week {}:", timelog.get_this_week_as_string());
            timelog.get_n_weeks(&today, 1)
        }
    };

    let a = rtimelog::activity::Activities::new_from_entries(entries);
    println!("{a}");

    rl_editor.clear_history();
    for a in Timelog::get_history(entries) {
        rl_editor.add_history_entry(a);
    }
}

fn show_prompt(timelog: &Timelog) -> Result<(), io::Error> {
    let since_last = timelog
        .get_n_days(&Local::now().date_naive(), 1)
        .last()
        .map(|e| Local::now().naive_local().signed_duration_since(e.stop));

    let since_str = match since_last {
        None => "no entries yet today".to_string(),
        Some(d) => format!(
            "{} h {} min since last entry",
            d.num_hours(),
            d.num_minutes() % 60
        ),
    };

    println!("\n{since_str}; type command (:h for help) or entry");
    Ok(())
}

fn run_editor(fname: &PathBuf) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    if let Err(e) = process::Command::new(&editor).arg(fname).status() {
        println!("Failed to run {} on {:?}: {:?}", &editor, fname, e);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut timelog = Timelog::new_from_default_file();
    let mut running = true;
    let mut time_mode = TimeMode::Day;
    let mut readline = Editor::<()>::new()?;
    let mut do_show = true;

    while running {
        if do_show {
            show(&timelog, &time_mode, &mut readline);
        }
        do_show = true;
        show_prompt(&timelog)?;

        match Command::parse(get_input(&mut readline)?) {
            Command::Nothing => (),
            Command::Quit => running = false,
            Command::Help => {
                show_help();
                do_show = false;
            }
            Command::Edit => {
                run_editor(&timelog.filename.unwrap());
                timelog = Timelog::new_from_default_file();
            }
            Command::SwitchMode(m) => time_mode = m,
            Command::Add(a) => {
                timelog.add(a);
                timelog.save()?;
            }
            Command::Error(e) => {
                println!("Error: {}", e);
                do_show = false;
            }
        }
    }
    Ok(())
}
