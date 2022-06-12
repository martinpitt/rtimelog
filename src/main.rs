mod activity;
mod store;

use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::process;

use chrono::prelude::*;
use rustyline::{error::ReadlineError, Editor};

use store::Timelog;

enum TimeMode {
    Day,
    Week,
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn get_input(rl: &mut Editor<()>) -> Result<String, ReadlineError> {
    match rl.readline("> ") {
        Ok(mut line) => {
            line.truncate(line.trim_end().len());
            Ok(line)
        }
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => Ok(":q".to_string()),
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

Any other input is the description of a task that you just finished."
    );
}

fn show(timelog: &Timelog, mode: &TimeMode) {
    clear_screen();
    let entries = match mode {
        TimeMode::Day => {
            println!("Work done today:");
            timelog.get_today()
        }
        TimeMode::Week => {
            println!("Work done this week:");
            timelog.get_this_week()
        }
    };

    let a = activity::Activities::new_from_entries(entries);
    println!("{}", a);
}

fn show_prompt(timelog: &Timelog) -> Result<(), io::Error> {
    let since_last = timelog
        .get_today()
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

    println!("\n{}; type command (:h for help) or entry", since_str);
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
    let mut readline = Editor::<()>::new();

    show(&timelog, &time_mode);

    while running {
        show_prompt(&timelog)?;

        let input = get_input(&mut readline)?;
        match input.as_str() {
            ":q" => running = false,
            ":h" => show_help(),
            ":e" => {
                run_editor(&timelog.filename.unwrap());
                timelog = Timelog::new_from_default_file();
                show(&timelog, &time_mode);
            }
            ":d" => {
                time_mode = TimeMode::Day;
                show(&timelog, &time_mode);
            }
            ":w" => {
                time_mode = TimeMode::Week;
                show(&timelog, &time_mode);
            }
            "" => (),
            _ => {
                timelog.add(input);
                timelog.save()?;
                show(&timelog, &time_mode);
            }
        }
    }
    Ok(())
}
