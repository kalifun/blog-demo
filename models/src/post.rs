use std::time::SystemTime;

use crate::tag::Tag;
use crate::user::User;
use diesel::prelude::*;

use crate::schema::posts;

use uuid::Uuid;

#[derive(Queryable, Associations)]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub desc: Option<String>,
    pub body: String,
    pub user_id: Uuid,
    pub tag_id: Option<Uuid>,
    pub state: i16,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
