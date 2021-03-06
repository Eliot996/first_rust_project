// part 2: https://www.youtube.com/watch?v=lLWchWTUFOQ

use std::collections::HashMap;
use std::fs;

fn main() {
    // get args
    let mut arguments = std::env::args().skip(1);

    // get the keys and print the key and value
    let key = arguments.next().expect("The key was not there");
    let value = arguments.next().expect("The value was not there");
    println!("The key is '{}' and the value is '{}'", key, value);


    let mut database = Database::new().expect("database db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
    flush: bool
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
        Ok(Database { map, flush: true })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = false;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()>{
    let mut contents = String::new();

    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }

    std::fs::write("kv.db", contents) // no semicolon since it is to be returned
}
