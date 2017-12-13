extern crate git2;

use git2::Repository;
use git2::Remote;

pub fn remote_details() -> (String, String) {
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
    return (String::from(owner), String::from(project))
}

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
