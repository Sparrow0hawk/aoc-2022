use std::{collections::HashMap, hash::Hash};
#[derive(Debug)]
pub struct Backpack {
    items: String,
    length: Option<u8>,
    shared_item: Option<HashMap<char, i32>>,
}

pub trait Length {
    fn get_length(&self) -> Option<u8>;
}

pub trait SharedItem {
    fn get_shared(&self) -> HashMap<char, i32>;
}

impl SharedItem for Backpack {
    fn get_shared(&self) -> HashMap<char, i32> {
        let mut char_count = HashMap::new();

        self.items
            .chars()
            .into_iter()
            .map(|c| char_count.entry(c).and_modify(|c| *c += 1).or_insert(1));

        char_count
    }
}

impl Length for Backpack {
    fn get_length(&self) -> Option<u8> {
        Some(self.items.chars().count())
    }
}

impl Backpack {
    fn new(s: String) -> Result<Backpack, &'static str> {
        let mut pack = Backpack {
            items: s,
            length: None,
            shared_item: None,
        };

        pack.length = pack.get_length();

        pack.shared_item = pack.get_shared();

        Ok(pack)
    }
}

pub fn solve_three(line: String) -> Result<Backpack, &'static str> {
    let pack = Backpack::new(line)?;

    Ok(pack)
}
