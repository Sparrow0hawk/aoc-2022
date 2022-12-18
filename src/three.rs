use std::collections::HashMap;

static ALPHABET_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

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
fn count_letter(s: char) -> Option<usize> {
    for (idx, lt) in ALPHABET_LOWER.iter().enumerate() {
        if s.eq_ignore_ascii_case(lt) {
            return Some(if s.is_uppercase() {
                idx + 1 + 26
            } else {
                idx + 1
            });
        }
    }
    None
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

        assert_eq!(pack.items, String::from("aaBBcc"));
        assert_eq!(pack.length, Some(6));
    }

    #[test]
    fn test_count_letter_lower() {
        let lt_a: char = 'a';

        assert_eq!(count_letter(lt_a).unwrap(), 1);
    }
    #[test]
    fn test_count_letter_upper() {
        let lt_Z: char = 'Z';

        assert_eq!(count_letter(lt_Z).unwrap(), 52);
    }
}
