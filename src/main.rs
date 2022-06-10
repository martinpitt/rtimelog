mod store;
mod activity;

use std::io::{self, Write};

use store::Timelog;

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
:q - quit
:h - show this help

Any other input is the description of a task that you just finished.");
}

fn show_today(timelog: &Timelog) {
    println!("Work done today:");
    let a = activity::Activities::new_from_entries(timelog.get_today());
    println!("{}", a);
}

fn main() {
    let mut timelog = Timelog::new_from_default_file();
    let mut running = true;

    clear_screen();
    show_today(&timelog);

    while running {
        print!("\ncommand (:h for help) or entry: ");
        io::stdout().flush()
            .expect("Failed to flush stdout");
        let input = stdin_line();
        match input.as_str() {
            ":q" => { running = false },
            ":h" => show_help(),
            _ => {
                timelog.add(input);
                timelog.save();
                clear_screen();
                show_today(&timelog);
            }
        }
    }
}
