use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::tags;

use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}

pub fn create_tag(conn: &mut PgConnection, tag_name: &str) -> Tag {
    let now = SystemTime::now();
    let uid = Uuid::new_v4();
    let tag_data = Tag {
        id: uid,
        name: tag_name.to_string(),
        create_time: now,
        update_time: now,
    };
    diesel::insert_into(tags::table)
        .values(&tag_data)
        .get_result(conn)
        .expect("Error create tag failed")
}
