mod store;
mod activity;

fn main() {
    let timelog = store::Timelog::new_from_default_file();
    println!("chronological all:");
    for entry in timelog.get_all() {
        println!("{}", entry);
    }

    println!("\ngrouped today:");
    let a = activity::Activities::new_from_entries(timelog.get_today());
    println!("{}", a);
}
