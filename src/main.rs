extern crate git2;
extern crate reqwest;

use git2::Repository;
use git2::Remote;
use std::env;
use reqwest::header;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_url<'a>(origin: &'a Remote) -> Vec<&'a str> {
    let url = origin.url().map(|s| s.split(":"));
    let url = url.expect("No url for origin!").collect();
    return url
}

fn get_repo_details(url: Vec<&str>) -> (&str, &str) {
    let names: Vec<&str> = url.get(1).expect("no url data!").split("/").collect();
    let owner = names.get(0).expect("no owner!");
    let project: Vec<&str> = names.get(1)
        .map(|s| s.split("."))
        .map(|v| v.collect())
        .expect("no project!");
    let project = project.get(0).expect("it's borked!");

    return (owner, project);
}

fn get_data(url: &str) -> Result<reqwest::Response, Box<std::error::Error>> {
    let mut headers = header::Headers::new();
    headers.set(header::Authorization("secret".to_string()));

    let client = reqwest::Client::builder()
         .default_headers(headers)
        .build()?;
    let res = client.get(url).send()?;
    Ok(res)
}

fn read_ghreleaseauth() -> String {
    let mut home_dir: PathBuf = env::home_dir().expect("Cannot find config file!");
    home_dir.push(".ghreleaseauth");
    let file = File::open(home_dir).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let line = match reader.read_line(&mut buf) {
        Ok(line) => line,
        Err(e) => panic!("Failed to read file!")
    };
    return buf
}

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let origin = match repo.find_remote("origin") {
        Ok(origin) => origin,
        Err(e) => panic!("no remote named origin: {}", e),
    };

    let url = get_url(&origin);
    let (owner, project) = get_repo_details(url);

    let target_url = format!("https://api.github.com/repos/{}/{}/releases", owner, project);
    let response = get_data(&target_url);

    let filepath = read_ghreleaseauth();
    println!("file {:?}", filepath);

    println!("Hello! {}, {}, {:?}", owner, project, response)
}
