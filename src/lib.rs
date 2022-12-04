use anyhow;
use reqwest;
use reqwest::blocking::Client;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn download_file(task: u8) -> Result<(), anyhow::Error> {
    let base_url = "https://adventofcode.com/2022/day/$TASK/input";

    let updated_url = base_url.replace("$TASK", &task.to_string());

    println!("{:?} is updated url", updated_url);

    let data_dir = Path::new("data");

    if data_dir.exists() {
        println!("{:?} already exists", data_dir.to_str().unwrap());
    } else {
        let _ = fs::create_dir(data_dir)?;
    }

    let mut dest = {
        let mut fname = "task_".to_string();

        fname.push_str(&task.to_string());

        let fname = data_dir.join(fname);

        fs::File::create(fname)?
    };

    let env_var = env::var("COOKIE")?;

    let client = Client::new();

    let content = client
        .get(updated_url)
        .header("Cookie", "session=$SESSION".replace("$SESSION", &env_var))
        .send()?;

    io::copy(&mut content.text()?.as_bytes(), &mut dest)?;

    Ok(())
}

pub fn solve_one() -> Result<i32, anyhow::Error> {
    let mut max_cal: i32 = 0;

    let mut elf_cal = 0;

    if let Ok(lines) = read_lines("data/task_1") {
        for line in lines {
            let line = line.unwrap();

            if line.is_empty() {
                if elf_cal > max_cal {
                    max_cal = elf_cal;
                }
                elf_cal = 0;
            } else {
                let line_val = line.parse::<i32>().unwrap();

                elf_cal = elf_cal + line_val
            }
        }
    }

    Ok(max_cal)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
