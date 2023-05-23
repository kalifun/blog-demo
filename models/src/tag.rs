use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::tags;

use uuid::Uuid;

#[derive(Queryable, Insertable, Debug)]
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

pub fn delete_tag(con: &mut PgConnection, uid: &str) {
    use crate::schema::tags::dsl::*;
    let id_data: Uuid;
    match Uuid::parse_str(uid) {
        Ok(i) => id_data = i,
        Err(e) => panic!("{}", e),
    }
    diesel::delete(tags.filter(id.eq(id_data)))
        .execute(con)
        .expect("Error delete failed");
}

pub fn modify_tag(con: &mut PgConnection, uid: &str, tag_name: &str) {
    use crate::schema::tags::dsl::*;
    let id_data: Uuid;
    match Uuid::parse_str(uid) {
        Ok(i) => id_data = i,
        Err(e) => panic!("{}", e),
    }

    diesel::update(tags)
        .filter(id.eq(id_data))
        .set(name.eq(tag_name))
        .execute(con)
        .expect("Error modify tag failed");
}

pub fn search_tag(con: &mut PgConnection, uid: &str) -> Vec<Tag> {
    use crate::schema::tags::dsl::*;
    let id_data: Uuid;
    match Uuid::parse_str(uid) {
        Ok(i) => id_data = i,
        Err(e) => panic!("{}", e),
    }

    let tags_data = tags
        .filter(id.eq(id_data))
        .load::<Tag>(con)
        .expect("Error load tags");
    tags_data
}
