use aoc_2022::{download_file, solve_one};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    task: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.task {
        println!("This is an arg: {}", args.task)
    }

    let _ = download_file(args.task);

    match args.task {
        1 => {
            let mut answer = solve_one().unwrap();
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
        _ => println!("Not sure what task you're doing!"),
    }
}
