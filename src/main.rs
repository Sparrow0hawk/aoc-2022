use aoc_2022::{download_file, read_file};
use clap::Parser;
use std::io::BufRead;
use three::solve_three;
use two::{match_hands, pick_hands};

mod one;
mod three;
mod two;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    task: u8,
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();

    let _ = download_file(args.task);

    let mut fname = String::from("data/task_");
    fname.push_str(&args.task.to_string());

    let open_file = read_file(fname).unwrap_or_else(|err| panic!("Error opening file: {err}"));

    match args.task {
        1 => {
            let mut answer = one::solve_one().unwrap();
            println!(
                "The answer to part 1 is: {:?}",
                answer.iter().max().unwrap()
            );
            answer.sort_by(|a, b| b.cmp(a));
            println!(
                "The answer to part 2 is: {:?}",
                &answer[0..=2].iter().sum::<i32>()
            )
        }

        2 => match args.part {
            1 => {
                let open_file = read_file("data/task_2")
                    .unwrap_or_else(|err| panic!("Error opening file: {err}"));

                let score: i64 = open_file
                    .lines()
                    .into_iter()
                    .map(|line| match_hands(line.unwrap().split(" ")).unwrap())
                    .sum();

                println!("Answer for task 2 is: {:?}", score);
            }

            2 => {
                let open_file = read_file("data/task_2")
                    .unwrap_or_else(|err| panic!("Error opening file: {err}"));

                let score: i64 = open_file
                    .lines()
                    .into_iter()
                    .map(|line| pick_hands(line.unwrap().split(" ")).unwrap())
                    .sum();

                println!("Answer for task 2, part 2 is: {:?}", score);
            }

            _ => println!("Invalid part"),
        },

        3 => match args.part {
            1 => {
                let ans = open_file
                    .lines()
                    .into_iter()
                    .map(|line| solve_three(line.unwrap()).unwrap());

                println!("Answer to task 3 part 1: {:?}", ans)
            }
            2 => println!("Answer to task 3 part 2:"),
            _ => println!("Invalid part"),
        },

        _ => println!("Not sure what task you're doing!"),
    }
}
