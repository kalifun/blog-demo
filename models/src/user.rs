use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
