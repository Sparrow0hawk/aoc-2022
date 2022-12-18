use std::collections::HashMap;
#[derive(Debug)]
pub struct Backpack {
    items: String,
    length: Option<usize>,
    shared_item: Option<char>,
}

pub trait Length {
    fn get_length(&self) -> usize;
}

pub trait SharedItem {
    fn get_shared(&self) -> char;
}

impl SharedItem for Backpack {
    fn get_shared(&self) -> char {
        // Find the duplicate letter in the pack
        // not super happy with this function as it doesn't handle error cases at all
        let mut char_count: HashMap<char, i32> = HashMap::new();

        for ch in self.items.chars() {
            char_count.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        let dup = char_count
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap();

        dup
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

    println!("{:?}", pack.shared_item);
    Ok(pack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pack() {
        let line = String::from("aaBBcc");

        let pack = Backpack::new(line).unwrap();

        assert_eq!(pack.items, line);
        assert_eq!(pack.length, Some(6));
    }
}
