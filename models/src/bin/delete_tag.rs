use models::{tag::delete_tag, *};

fn main() {
    let conn = &mut establish_connection();

    let uid = String::from("3128fd9f-3446-414b-b4c7-88c22b711608");

    delete_tag(conn, &uid)
}
