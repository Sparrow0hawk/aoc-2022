fn find_overlap(line: &str) -> Result<Vec<String>, &'static str> {
    let parts = line
        .split(",")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    Ok(parts)
}

pub fn solve_four_one(line: &str) -> Result<Vec<String>, &'static str> {
    find_overlap(line)
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
