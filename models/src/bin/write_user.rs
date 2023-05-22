use std::io::stdin;

use models::{establish_connection, user::create_user};

fn main() {
    let connection = &mut establish_connection();

    let mut u = String::new();

    println!("what would you like your user to be?");
    stdin().read_line(&mut u).unwrap();
    let u = u.trim_end();
    let us = create_user(connection, &u);
    println!("id {}", us.id);
}
