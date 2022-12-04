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
            let answer = solve_one().unwrap();
            println!("Max calories in the file is {}", answer)
        }
        _ => println!("Not sure what task you're doing!"),
    }
}
