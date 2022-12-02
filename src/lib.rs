use anyhow;
use reqwest;
use std::fs;
use std::env;
use std::io;
use std::{path::Path};
use reqwest::blocking::{Client};

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

    let mut dest =  {
        let mut fname = "task_".to_string();

        fname.push_str(&task.to_string());

        let fname = data_dir.join(fname);

        fs::File::create(fname)?
    };

    let env_var = env::var("COOKIE")?;

    let client = Client::new();

    let content = client.get(updated_url)
        .header("Cookie","session=$SESSION".replace("$SESSION", &env_var))
        .send()?;

    io::copy(&mut content.text()?.as_bytes(), &mut dest)?;

    Ok(())   
}

