use std::time::SystemTime;

use crate::tag::Tag;
use crate::user::User;
use diesel::prelude::*;

use crate::schema::posts;

#[derive(Queryable, Associations)]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub body: String,
    pub user_id: String,
    pub tag_id: String,
    pub state: i32,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
