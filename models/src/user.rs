use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::users;

use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, PartialEq)]
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
