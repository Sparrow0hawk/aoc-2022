#[derive(Debug, PartialEq, Eq)]
pub struct Crate {
    name: char,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Stack {
    index: Vec<i32>,
    crates: Vec<Option<Crate>>,
}

impl Stack {
    fn new(line: &str) -> Stack {
        let mut elements: Vec<Option<Crate>> = Vec::new();
        let mut indices = Vec::new();

        let mut index = 1;

        for chunk in line.as_bytes().chunks(4) {
            let chunk_str = std::str::from_utf8(chunk).unwrap();

            let element = chunk_str.chars().nth(1).unwrap();

            if element.is_alphabetic() {
                elements.push(Some(Crate::new(element)));
            } else {
                elements.push(None);
            }

            indices.push(index);

            index += 1;
        }

        Stack {
            crates: elements,
            index: indices,
        }
    }
}

impl Crate {
    fn new(name: char) -> Crate {
        Crate { name }
    }
}

pub fn solve_five_one(line: &str) -> () {
    let stack = Stack::new(line);

    println!("{:?}", stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_index() {
        let test: &str = "[a]     [b] [c]";

        let res = Stack::new(test);

        assert_eq!(res.index, vec!(1, 2, 3, 4))
    }

    #[test]
    fn test_new_stack_crates() {
        let test: &str = "[a]     [b] [c]";

        let res = Stack::new(test);

        assert_eq!(
            res.crates,
            vec![
                Some(Crate::new('a')),
                None,
                Some(Crate::new('b')),
                Some(Crate::new('c'))
            ]
        )
    }
}
