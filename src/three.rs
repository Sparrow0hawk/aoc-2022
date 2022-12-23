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
        let dup = create_char_count(&self.items)
            .unwrap()
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap();

        dup
    }
}

pub fn create_char_count(s: &str) -> Result<HashMap<char, i32>, &'static str> {
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        char_count.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }
    Ok(char_count)
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

    fn get_priority(&self) -> Result<usize, &'static str> {
        let (comp1, comp2) = self.items.split_at(self.length.unwrap() / 2);

        let out = comp1
            .to_string()
            .chars()
            .find_map(|item| comp2.to_string().chars().find(|&s_item| s_item == item));

        let priority = count_letter(out.unwrap())?;

        Ok(priority)
    }
}

fn count_letter(s: char) -> Result<usize, &'static str> {
    for (idx, lt) in ALPHABET_LOWER.iter().enumerate() {
        if s.eq_ignore_ascii_case(lt) {
            return Ok(if s.is_uppercase() {
                idx + 1 + 26
            } else {
                idx + 1
            });
        }
    }
    Err("Couldn't get the index of the letter passed")
}

pub fn solve_three(line: String) -> Result<usize, &'static str> {
    let pack = Backpack::new(line)?;

    // debug print line
    //println!("{:?}", pack.get_priority().unwrap());
    pack.get_priority()
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
    #[test]
    fn test_create_char_count() {
        let foo = "aabC";

        let result = create_char_count(foo).unwrap();

        assert_eq!(result, HashMap::from([('a', 2), ('b', 1), ('C', 1)]))
    }
}
