use models::{tag::search_tag, *};

fn main() {
    let con = &mut establish_connection();

    let uid = String::from("3128fd9f-3446-414b-b4c7-88c22b711608");

    let data = search_tag(con, &uid);

    for u in data {
        println!("{:?}", u);
    }
}
