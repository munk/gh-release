extern crate git2;

use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let origin = match repo.find_remote("origin") {
        Ok(origin) => origin,
        Err(e) => panic!("no remote named origin: {}", e),
    };
    let url = origin.url().expect("no url for origin");
    let url: Vec<&str> = url.split(":").collect();

    let names = url.get(1).expect("no url data!");
    let names: Vec<&str> = names.split("/").collect();

    let owner = names.get(0).expect("no owner!");
    let project: Vec<&str> = names.get(1).expect("no project!").split(".").collect();
    let project = project.get(0).expect("it's borked!");
   
    let target_url = format!("/repos/{}/{}/releases", owner, project);

    println!("Hello! {}, {}, {}", owner, project, target_url)
}
