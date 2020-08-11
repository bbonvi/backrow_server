use super::DieselError;
use crate::schema::files;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: i32,
    pub hash: String,
    pub ext: String,
    pub created_at: NaiveDateTime,
}

impl File {
    pub fn by_id(file_id: Uuid, conn: &PgConnection) -> Result<File, DieselError> {
        use crate::schema::files::dsl::*;

        files
            .filter(id.eq(file_id))
            .first::<File>(conn)
            .map_err(|err| {
                error!("Couldn't query file by id {:?}: {}", file_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::files::dsl::*;

        diesel::delete(files.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove file {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "files"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewFile<'a> {
    pub hash: &'a str,
    pub ext: &'a str,
}

impl<'a> NewFile<'a> {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<File, DieselError> {
        use crate::schema::files::dsl::*;

        diesel::insert_into(files)
            .values(self)
            .get_result::<File>(conn)
            .map_err(|err| {
                error!("Couldn't create file {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
