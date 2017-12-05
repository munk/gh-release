extern crate git2;
extern crate reqwest;

use git2::Repository;
use git2::Remote;
use std::io::Read;
use reqwest::header;
use reqwest::Client;

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

    println!("Hello! {}, {}, {:?}", owner, project, response)
}
