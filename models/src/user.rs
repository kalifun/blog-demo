use core::panic;
use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::users;

use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}

pub fn create_user(conn: &mut PgConnection, user_name: &str) -> User {
    let uid = Uuid::new_v4();
    let now = SystemTime::now();
    let user = User {
        id: uid,
        name: user_name.to_string(),
        create_time: now,
        update_time: now,
    };
    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Error create failed")
}

pub fn delete_user(con: &mut PgConnection, uid: &str) {
    use crate::schema::users::dsl::*;
    let id_data: Uuid;
    match Uuid::parse_str(uid) {
        Ok(i) => id_data = i,
        Err(e) => panic!("{}", e),
    }
    diesel::delete(users.filter(id.eq(id_data)))
        .execute(con)
        .expect("Error delete failed");
}

pub fn modify_user(con: &mut PgConnection, uid: &str, user_name: &str) {
    use crate::schema::users::dsl::*;
    let id_data: Uuid;
    match Uuid::parse_str(uid) {
        Ok(i) => id_data = i,
        Err(e) => panic!("{}", e),
    }

    diesel::update(users)
        .filter(id.eq(id_data))
        .set(name.eq(user_name))
        .execute(con)
        .expect("Error modify user failed");
}

pub fn search_user(con: &mut PgConnection, uid: &str) -> Vec<User> {
    use crate::schema::users::dsl::*;
    let id_data: Uuid;
    match Uuid::parse_str(uid) {
        Ok(i) => id_data = i,
        Err(e) => panic!("{}", e),
    }

    let users_data = users
        .filter(id.eq(id_data))
        .load::<User>(con)
        .expect("Error load users");
    users_data
}
