use std::io::stdin;

use models::{establish_connection, tag::create_tag};

fn main() {
    let connection = &mut establish_connection();

    let mut tag = String::new();

    println!("what would you like your tag to be?");
    stdin().read_line(&mut tag).unwrap();
    let tag = tag.trim_end();
    let t = create_tag(connection, &tag);
    println!("id {}", t.id);
}
