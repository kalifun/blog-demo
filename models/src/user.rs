use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::users;

use uuid::Uuid;

#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
