mod store;
mod activity;

use std::io::{self, Write};

use store::Timelog;

enum TimeMode {
    Day,
    Week,
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn stdin_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .expect("Failed to read from stdlin");
    buffer.pop(); // eat \n
    buffer
}

fn show_help() {
    println!("
:w - switch to weekly mode
:d - switch to daily mode
:q - quit
:h - show this help

Any other input is the description of a task that you just finished.");
}

fn show_today(timelog: &Timelog) {
    println!("Work done today:");
    let a = activity::Activities::new_from_entries(timelog.get_today());
    println!("{}", a);
}

fn show_week(timelog: &Timelog) {
    println!("Work done this week:");
    let a = activity::Activities::new_from_entries(timelog.get_this_week());
    println!("{}", a);
}

fn show(timelog: &Timelog, mode: &TimeMode) {
    clear_screen();
    match mode {
        TimeMode::Day => show_today(timelog),
        TimeMode::Week => show_week(timelog),
    }
}

fn main() {
    let mut timelog = Timelog::new_from_default_file();
    let mut running = true;
    let mut time_mode = TimeMode::Day;

    show(&timelog, &time_mode);

    while running {
        print!("\ncommand (:h for help) or entry: ");
        io::stdout().flush()
            .expect("Failed to flush stdout");
        let input = stdin_line();
        match input.as_str() {
            ":q" => { running = false },
            ":h" => show_help(),
            ":d" => {
                time_mode = TimeMode::Day;
                show(&timelog, &time_mode);
            },
            ":w" => {
                time_mode = TimeMode::Week;
                show(&timelog, &time_mode);
            },
            _ => {
                timelog.add(input);
                timelog.save();
                show(&timelog, &time_mode);
            }
        }
    }
}
