use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub body: String,
    pub user_id: i32,
    pub tag_id: i32,
    pub state: i32,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
