use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
