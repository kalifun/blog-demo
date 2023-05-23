use models::{user::search_user, *};

fn main() {
    let con = &mut establish_connection();

    let uid = String::from("c5f74c66-841e-400b-b6cb-129d4a2e3c39");

    let data = search_user(con, &uid);

    for u in data {
        println!("{:?}", u);
    }
}
