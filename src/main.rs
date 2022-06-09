mod store;

fn main() {
    let contents = store::read_log();
    println!("{}", contents);
}
