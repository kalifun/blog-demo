use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::tags;

use uuid::Uuid;

#[derive(Queryable)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}
