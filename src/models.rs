use diesel::prelude::*;
use diesel_async::{pooled_connection::bb8, AsyncPgConnection, AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use crate::{schema::*, Error};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Character {
    pub name: String,
    pub owner: i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::stats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Stat {
    pub char_name: String,
    pub char_owner: i64,
    pub name: String,
    pub value: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub active_char: String,
}

type Pool = bb8::Pool<AsyncPgConnection>;

impl Character {
    pub async fn create_character_with_stats(pool: &Pool, character: Character, stats: Vec<(String, i32)>) -> Result<(), Error> {
        let mut conn = pool.get().await?;

        conn.transaction::<_, diesel::result::Error, _>(|conn| async move {
            diesel::insert_into(characters::table)
                .values(&character)
                .execute(conn)
                .await?;

            diesel::insert_into(stats::table)
                .values(stats
                    .into_iter()
                    .map(|(name, value)| Stat { char_name: name.clone(), char_owner: character.owner, name, value })
                    .collect::<Vec<Stat>>()
                )
                .execute(conn)
                .await?;
            
            Ok(())
        }.scope_boxed()).await?;

        Ok(())
    }
}