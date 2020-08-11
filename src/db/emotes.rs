use super::DieselError;
use crate::schema::emotes;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    pub id: Uuid,
    pub name: String,

    #[serde(skip_serializing)]
    pub image_id: Uuid,
    pub room_id: Uuid,

    #[serde(skip_serializing)]
    pub is_global: bool,

    #[serde(skip_serializing)]
    pub is_deleted: bool,

    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

#[derive(AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "emotes"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewEmote<'a> {
    pub name: &'a str,
    pub image_id: Uuid,
    pub room_id: Uuid,
    pub is_global: bool,
    pub is_deleted: bool,
}

impl<'a> NewEmote<'a> {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<Emote, DieselError> {
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

impl Emote {
    // TODO: paginations
    pub fn list_by_room_id(
        room_id_query: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<Emote>, DieselError> {
        use crate::schema::emotes::dsl::*;

        emotes
            .filter(room_id.eq(room_id_query))
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
    pub fn by_id(emote_id: Uuid, conn: &PgConnection) -> Result<Emote, DieselError> {
        use crate::schema::emotes::dsl::*;

        emotes
            .filter(id.eq(emote_id))
            .first::<Emote>(conn)
            .map_err(|err| {
                error!("Couldn't query emote by id {:?}: {}", emote_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::emotes::dsl::*;

        diesel::delete(emotes.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove emote {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<Emote, DieselError> {
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
