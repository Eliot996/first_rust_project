use std::fmt::format;

fn main() {
    let mut arguments = std::env::args().skip(1);

    let key = arguments.next().expect("The key was not there");
    let value = arguments.next().expect("The value was not there");
    println!("The key is '{}' and the value is '{}'", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents);
}
