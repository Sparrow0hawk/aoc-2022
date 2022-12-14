use aoc_2022::read_lines;

pub fn solve_one() -> Result<Vec<i32>, anyhow::Error> {
    let mut cal_vec: Vec<i32> = Vec::new();
    let mut elf_cal = 0;

    if let Ok(lines) = read_lines("data/task_1") {
        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                cal_vec.push(elf_cal);
                elf_cal = 0;
            } else {
                let line_val = line.parse::<i32>().unwrap();

                elf_cal = elf_cal + line_val
            }
        }
    }

    Ok(cal_vec)
}
