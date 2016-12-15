extern crate regex;
extern crate crypto;
#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

fn md5(s: String) -> String {
    use crypto::digest::Digest;
    let mut sh = crypto::md5::Md5::new();
    sh.input_str(&s);
    sh.result_str()
}

fn get_triple(s: &str) -> Option<char> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(000|111|222|333|444|555|666|777|888|999|000|aaa|bbb|ccc|ddd|eee|fff)").unwrap();
    }

    match RE.captures(s) {
        Some(caps) => caps.at(1).unwrap().chars().next(),
        None => None
    }
}

fn has_quintuple(digit: char, s: &str) -> bool {
    let re = format!("{0}{0}{0}{0}{0}", digit);
    regex::is_match(&re , s).unwrap_or(false)
}

#[derive(Debug)]
pub struct HashStore {
    store: HashMap<usize, String>,
    salt: String,
}

impl HashStore {
    pub fn new(salt: &str) -> HashStore {
        HashStore {
            store: HashMap::new(),
            salt: salt.to_owned(),
        }
    }

    pub fn get<'a>(&'a mut self, index: usize) -> &'a str {
        let ref salt = self.salt;
        let entry = self.store.entry(index).or_insert_with(|| {
            md5(format!("{}{}", salt, index))
        });
        entry
    }
    
    pub fn get_stretch<'a>(&'a mut self, index: usize) -> &'a str {
        let ref salt = self.salt;
        let entry = self.store.entry(index).or_insert_with(||{
            let mut hash = md5(format!("{}{}", salt, index));
            let mut iter = 0;
            while iter < 2016 {
                hash = md5(hash);
                iter += 1;
            }
            hash
        });
        entry
    }
}

fn main() {
    let mut store = HashStore::new("ahsbgdzn");
    let mut num_found = 0;
    let mut index = 1;
    while num_found < 64 {
        match get_triple(store.get(index)) {
            Some(digit) => {
                for j in (index + 1)..(index + 1001) {
                    if has_quintuple(digit, store.get(j)) {
                        num_found += 1;
                        break;
                    }
                }
                index += 1;
            },
            None => { index += 1; },
        }
    }
    println!("Part 1 answer = {}", index - 1);

    store = HashStore::new("ahsbgdzn");
    num_found = 0;
    index = 1;
    while num_found < 64 {
        match get_triple(store.get_stretch(index)) {
            Some(digit) => {
                for j in (index + 1)..(index + 1001) {
                    if has_quintuple(digit, store.get_stretch(j)) {
                        num_found += 1;
                        break;
                    }
                }
                index += 1;
            },
            None => { index += 1; },
        }
    }
    println!("Part 2 answer = {}", index-1);

    
}
