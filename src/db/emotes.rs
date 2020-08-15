use super::DieselError;
use crate::schema::emotes;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "emotes"]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    pub id: String,
    pub name: String,

    #[serde(skip_serializing)]
    pub file_id: String,
    pub room_id: String,

    #[serde(skip_serializing)]
    pub is_global: bool,

    #[serde(skip_serializing)]
    pub is_deleted: bool,

    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl Emote {
    // TODO: paginations
    pub fn list_by_room_id(
        room_id_query: String,
        conn: &PgConnection,
    ) -> Result<Vec<Emote>, DieselError> {
        use crate::schema::emotes::dsl::*;

        emotes
            .filter(room_id.eq(room_id_query.clone()))
            .load::<Emote>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query emote by room id {:?}: {}",
                    room_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn by_id(emote_id: String, conn: &PgConnection) -> Result<Emote, DieselError> {
        use crate::schema::emotes::dsl::*;

        emotes
            .filter(id.eq(emote_id.clone()))
            .first::<Emote>(conn)
            .map_err(|err| {
                error!("Couldn't query emote by id {:?}: {}", emote_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::emotes::dsl::*;

        diesel::delete(emotes.filter(id.eq(self.id.to_owned())))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove emote {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<Emote, DieselError> {
        use crate::schema::emotes::dsl::*;

        diesel::update(emotes)
            .set(self)
            .get_result::<Emote>(conn)
            .map_err(|err| {
                error!("Couldn't update emote {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(Insertable, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "emotes"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewEmote<'a> {
    pub name: &'a str,
    pub file_id: String,
    pub room_id: String,
    pub is_global: bool,
    pub is_deleted: bool,
}

impl<'a> NewEmote<'a> {
    pub fn create(&self, conn: &PgConnection) -> Result<Emote, DieselError> {
        use crate::schema::emotes::dsl::*;

        diesel::insert_into(emotes)
            .values(self)
            .get_result::<Emote>(conn)
            .map_err(|err| {
                error!("Couldn't create emote {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
