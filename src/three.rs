use std::collections::HashMap;
#[derive(Debug)]
pub struct Backpack {
    items: String,
    length: Option<usize>,
    shared_item: Option<HashMap<char, i32>>,
}

pub trait Length {
    fn get_length(&self) -> usize;
}

pub trait SharedItem {
    fn get_shared(&self) -> HashMap<char, i32>;
}

impl SharedItem for Backpack {
    fn get_shared(&self) -> HashMap<char, i32> {
        let mut char_count: HashMap<char, i32> = HashMap::new();

        for ch in self.items.chars() {
            char_count.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        char_count
    }
}

impl Length for Backpack {
    fn get_length(&self) -> usize {
        self.items.chars().count()
    }
}

impl Backpack {
    fn new(s: String) -> Result<Backpack, &'static str> {
        let mut pack = Backpack {
            items: s,
            length: None,
            shared_item: None,
        };

        pack.length = Some(pack.get_length());

        pack.shared_item = Some(pack.get_shared());

        Ok(pack)
    }
}

pub fn solve_three(line: String) -> Result<Backpack, &'static str> {
    let pack = Backpack::new(line)?;

    println!("{:?}", pack.items);
    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pack() {
        let line = String::from("aaBBcc");

        let pack = Backpack::new(line).unwrap();

        assert_eq!(pack.items, String::from("aaBBcc"));
        assert_eq!(pack.length, Some(6));
    }
}
