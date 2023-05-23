use models::{tag::modify_tag, *};

fn main() {
    let con = &mut establish_connection();

    let uid = String::from("3128fd9f-3446-414b-b4c7-88c22b711608");

    let tag_name = String::from("test_tag");
    modify_tag(con, &uid, &tag_name);
}
