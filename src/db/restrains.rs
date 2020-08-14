use super::DieselError;
use crate::schema::restrains;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Restrain {
    pub id: i64,
    pub user_id: i64,
    pub ip: Option<String>,
    /// Probably won't be used
    pub fingerprint: Option<String>,
    pub channel_id: Option<i64>,
    pub is_global: bool,
    /// `is_ban` indicates whether restrain is ban or timeout
    pub is_ban: bool,
    pub ending_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl Restrain {
    pub fn by_id(restrain_id: i64, conn: &PgConnection) -> Result<Restrain, DieselError> {
        use crate::schema::restrains::dsl::*;

        restrains
            .filter(id.eq(restrain_id))
            .first::<Restrain>(conn)
            .map_err(|err| {
                error!("Couldn't query restrain by id {:?}: {}", restrain_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn by_user_id(user_id_query: i64, conn: &PgConnection) -> Result<Restrain, DieselError> {
        use crate::schema::restrains::dsl::*;

        restrains
            .filter(user_id.eq(user_id_query))
            .first::<Restrain>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query restrain by user_id {:?}: {}",
                    user_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn list_by_channel_id(
        channel_id_query: i64,
        conn: &PgConnection,
    ) -> Result<Vec<Restrain>, DieselError> {
        use crate::schema::restrains::dsl::*;

        // TODO: pagination
        const LIMIT: i64 = 20;

        restrains
            .filter(user_id.eq(channel_id_query))
            .limit(LIMIT)
            .load::<Restrain>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query restrains by channel_id {:?}: {}",
                    channel_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::restrains::dsl::*;

        diesel::delete(restrains.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove restrain {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(&self, conn: &PgConnection) -> Result<Restrain, DieselError> {
        use crate::schema::restrains::dsl::*;

        diesel::update(restrains)
            .set(self)
            .get_result::<Restrain>(conn)
            .map_err(|err| {
                error!("Couldn't update restrain {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(Insertable, AsChangeset, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "restrains"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewRestrain {
    pub user_id: i64,
    pub ip: Option<String>,
    /// Probably won't be used
    pub fingerprint: Option<String>,
    pub channel_id: Option<i64>,
    pub is_global: bool,
    /// `is_ban` indicates whether restrain is ban or timeout
    pub is_ban: bool,
    pub ending_at: Option<NaiveDateTime>,
}

impl NewRestrain {
    pub fn create(&self, conn: &PgConnection) -> Result<Restrain, DieselError> {
        use crate::schema::restrains::dsl::*;

        diesel::insert_into(restrains)
            .values(self)
            .get_result::<Restrain>(conn)
            .map_err(|err| {
                error!("Couldn't create restrain {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
