#[derive(Debug)]
pub struct ElfPair {
    elf1: Vec<i64>,
    elf2: Vec<i64>,
}

impl ElfPair {
    pub fn new(line: &str) -> Result<ElfPair, &'static str> {
        let mut split_line = line.split(",");

        let elf1: Vec<i64> = match split_line.next() {
            Some(line) => line
                .to_string()
                .split("-")
                .map(|i| i.to_string().parse::<i64>().unwrap())
                .collect(),
            _ => return Err("Could no parse line!"),
        };

        let elf2: Vec<i64> = match split_line.next() {
            Some(line) => line
                .to_string()
                .split("-")
                .map(|i| i.to_string().parse::<i64>().unwrap())
                .collect(),
            _ => return Err("Could no parse line!"),
        };

        let pair = ElfPair { elf1, elf2 };

        Ok(pair)
    }
}

// fn find_overlap(line: &str) -> Result<Vec<Vec<i64>>, &'static str> {
//     Ok(parts)
// }

pub fn solve_four_one(line: &str) -> Result<ElfPair, &'static str> {
    let pair = ElfPair::new(line)?;

    Ok(pair)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_overlap_no() {
        let input = "1-5,6-9";

        let res = find_overlap(input).unwrap();

        assert_eq!(res, 0)
    }

    #[test]
    fn test_find_overlap_yes() {
        let input = "2-4,1-4";

        let res = find_overlap(input).unwrap();

        assert_eq!(res, 3)
    }
}
