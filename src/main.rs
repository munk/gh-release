extern crate git2;
extern crate reqwest;
extern crate base64;

use std::env;
use reqwest::header;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

mod git;

fn create_release(url: &str, auth: header::Basic) -> Result<reqwest::Response, Box<std::error::Error>> {
    let mut headers = header::Headers::new();
    headers.set(header::Authorization(auth));

    let mut body = HashMap::new();
    body.insert("tag_name", "v0.0.1");
    body.insert("name", "v0.0.1");
    body.insert("commitish", "master");
    body.insert("body", "This is a release");

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let res = client.post(url).json(&body).send()?;
    Ok(res)
}

fn get_data(url: &str, auth: header::Basic) -> Result<reqwest::Response, Box<std::error::Error>> {
    let mut headers = header::Headers::new();
    headers.set(header::Authorization(auth));

    let client = reqwest::Client::builder()
         .default_headers(headers)
        .build()?;
    let res = client.get(url).send()?;
    Ok(res)
}

fn read_line(mut reader: &mut BufReader<File>) -> String {
    let mut buf = String::new();
    reader.read_line(&mut buf);
    return String::from(buf.trim())
}


fn read_ghreleaseauth() -> header::Basic {
    let mut home_dir: PathBuf = env::home_dir().expect("Cannot find config file!");
    home_dir.push(".ghreleaseauth");
    let file = File::open(home_dir).expect("Unable to open file ~/.ghreleaseauth");
    let mut reader: BufReader<File> = BufReader::new(file);
    let username = read_line(&mut reader);
    let password = read_line(&mut reader);

    return header::Basic {
        username: username,
        password: Some(password),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let tag_name: Option<&String> = if args.len() > 1 {
        Some(&args[1])
    } else {
        None
    };

    println!("{:?}", tag_name);

    let (owner, project) = git::remote_details();

    let target_url = format!("https://api.github.com/repos/{}/{}/releases", owner, project);
    let auth_string = read_ghreleaseauth();
    let mut create_response = match create_release(&target_url, auth_string) {
        Ok(create_response) => create_response,
        Err(e) => panic!("Unable to create release: {}", e),
    };
    println!("Create response {:?}", create_response.text());

    let auth_string = read_ghreleaseauth();
    let mut response = match get_data(&target_url, auth_string) {
        Ok(response) => response,
        Err(e) => panic!("Unable to reach github: {}", e),
    };

    println!("Hello! {}, {}, {:?}", owner, project, response.text())
}
