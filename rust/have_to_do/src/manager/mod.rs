use bincode::SizeLimit;
use bincode::serde::{serialize_into, deserialize_from};
use std::collections::HashMap;

use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io;

include!(concat!(env!("OUT_DIR"), "/manage_serde_types.rs"));

impl CrateName {
    fn new(get_name: String) -> CrateName {
        CrateName {
            name: Some(get_name),
            done: false,
        }
    }
}
impl Manager {
    pub fn new() -> Manager {
        println!("welcome to crate manager");
        println!("creat the new file to manage");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("fail to get the file name");
        File::create(Path::new(&file_name.trim())).expect("fail to make the file");
        Manager {
            list: HashMap::new(),
            achivement: 0,
            obtain_file: file_name.trim().to_string(),
            number_of_crate: 0,
        }
    }
    pub fn open(manage_file: &'static str) -> Manager {
        let mut file = OpenOptions::new()
            .read(true)
            .open(Path::new("./").join(manage_file))
            .expect("fail to open this file maybe wrong type");
        deserialize_from(&mut file, SizeLimit::Infinite).unwrap()
    }
    pub fn insert(&mut self, gname: &str) {
        if !self.list.values().any(|a| *a.name.as_ref().unwrap() == gname) {
            self.number_of_crate += 1;
            self.list.insert(self.number_of_crate, CrateName::new(gname.to_string()));
        } else {
            println!("there is the same crate here");
        }
    }
    pub fn list(&self) {
        println!("here is the crate you have to do");
        for (key, crate_name) in self.list.iter() {
            println!("{}. {}", key, crate_name.name.as_ref().unwrap());
        }
        println!("have fun");
    }
}
impl Drop for Manager {
    fn drop(&mut self) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(Path::new("./").join(&self.obtain_file))
            .expect("fail to open the file to serialize");
        serialize_into(&mut file, &self, SizeLimit::Infinite).expect("fail to serialize the data");
    }
}
