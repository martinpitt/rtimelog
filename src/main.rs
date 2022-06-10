mod store;

fn main() {
    let timelog = store::Timelog::new_from_default_file();
    for entry in timelog.get_all() {
        println!("{}", entry);
    }
}
