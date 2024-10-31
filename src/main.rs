//Thanks AHQ for this code

use std::{collections::HashMap, fs};

use serde_json::to_string;
use sqlite::Connection;

fn main() {
    let db = Connection::open("export.db").unwrap();

    let mut final_raw: HashMap<u64, HashMap<String, String>> = HashMap::new();

    db.iterate("SELECT * FROM chromebooks", |data| {
        println!("START");
        let mut id = 0u64;
        let mut values: HashMap<String, String> = HashMap::new();

        data.iter().for_each(|(k, v)| {
            let v = v.as_ref();
            assert!(v.is_some());

            if k == &"id" {
                id = v.unwrap().parse().expect("id must be u64");
            } else {
                values.insert(k.to_string(), v.unwrap().to_string());
            }
        });
        println!("END");

        final_raw.insert(id, values);
        true
    }).unwrap();

    let data = to_string(&final_raw).unwrap();

    fs::write("data.json", data).unwrap();
}