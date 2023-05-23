use models::{user::delete_user, *};

fn main() {
    let conn = &mut establish_connection();

    let uid = String::from("28F53D6C-74C3-DEFC-DECE-1428EC7268CE");

    delete_user(conn, &uid)
}
