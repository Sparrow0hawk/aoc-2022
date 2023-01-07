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

    fn find_overlap(&self) -> bool {
        // implementation inspired by https://github.com/aaronblohowiak/advent-of-code-2022/blob/main/four/src/main.rs
        // RangeInclusive works nicely here
        let elf1 = std::ops::RangeInclusive::new(self.elf1[0], self.elf1[1]);
        let elf2 = std::ops::RangeInclusive::new(self.elf2[0], self.elf2[1]);

        if elf1.start() <= elf2.start() && elf1.end() >= elf2.end()
            || elf2.start() <= elf1.start() && elf2.end() >= elf1.end()
        {
            // println!(
            //     "Fully contained overlap! {:?}-{:?}, {:?}-{:?}",
            //     self.elf1[0], self.elf1[1], self.elf2[0], self.elf2[1]
            // );
            return true;
        } else if elf1.contains(&elf2.start())
            || elf1.contains(&elf2.end())
            || elf2.contains(&elf1.start())
            || elf2.contains(elf1.end())
        {
            // println!(
            //     "Overlap! {:?}-{:?}, {:?}-{:?}",
            //     self.elf1[0], self.elf1[1], self.elf2[0], self.elf2[1]
            // );
            return false;
        } else {
            return false;
        }
    }
}

pub fn solve_four_one(line: &str) -> Result<bool, &'static str> {
    let pair = ElfPair::new(line)?;

    Ok(pair.find_overlap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_overlap_no() {
        let input = "1-5,6-9";

        let test_elf = ElfPair::new(input).unwrap();

        let res = test_elf.find_overlap();

        assert_eq!(res, false)
    }

    #[test]
    fn test_find_overlap_partial() {
        let input = "2-5,1-4";

        let test_elf = ElfPair::new(input).unwrap();

        let res = test_elf.find_overlap();

        assert_eq!(res, false)
    }

    #[test]
    fn test_find_overlap_full() {
        let input = "14-88,3-99";

        let test_elf = ElfPair::new(input).unwrap();

        let res = test_elf.find_overlap();

        assert_eq!(res, true)
    }
}
