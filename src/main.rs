use github_actions::greet;
use std::io;

fn main() {
    // read user's name
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // use lib.rs module to greet the user
    println!("{}", greet(&name));
}
