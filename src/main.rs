// https://www.youtube.com/watch?v=lLWchWTUFOQ

use std::collections::HashMap;
use std::fs;

fn main() {
    // get args
    let mut arguments = std::env::args().skip(1);

    // get the keys and print the key and value
    let key = arguments.next().expect("The key was not there");
    let value = arguments.next().expect("The value was not there");
    println!("The key is '{}' and the value is '{}'", key, value);

    // concat with a marco, that can take af variable amount of arguments
    let contents = format!("{}\t{}\n", key, value);

    // write to file
    fs::write("kv.db", contents).unwrap();


    let database = Database::new().expect("database::new() crashed");
}

struct Database {
    map: HashMap<String, String>
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read file
        let contents = fs::read_to_string("kv.db")?;

        let mut map = HashMap::new();
        for line in contents.lines() {
            // parse the string
            let mut chunks = line.splitn(2, '\t');
            let key= chunks.next().expect("Key not found");
            let value= chunks.next().expect("Value not found");

            // populate our map (used to_owned to make a copy of the value,
            // since it was a pointer('&str'), and therefore does not own the data)
            map.insert(key.to_owned(), value.to_owned());
        }

        // return a new database with the map in it
        Ok(Database {
            map: map
        })
    }
}
