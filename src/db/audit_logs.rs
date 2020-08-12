use super::DieselError;
use crate::schema::audit_logs;

use crate::diesel::prelude::*;
use crate::diesel::*;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;
use diesel::sql_types::*;
use std::io::Write;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[sql_type = "SmallInt"]
pub enum AuditLogKind {
    Add = 0,
    Change = 1,
    Delete = 2,
}

impl From<i16> for AuditLogKind {
    fn from(value: i16) -> AuditLogKind {
        match value {
            0 => AuditLogKind::Add,
            1 => AuditLogKind::Change,
            2 => AuditLogKind::Delete,
            _ => AuditLogKind::Add,
        }
    }
}

impl<ST, DB> FromSql<ST, DB> for AuditLogKind
where
    i16: FromSql<ST, DB>,
    DB: Backend,
{
    fn from_sql(value: Option<&<DB as Backend>::RawValue>) -> deserialize::Result<Self> {
        <i16 as FromSql<ST, DB>>::from_sql(value).map(AuditLogKind::from)
    }
}

impl<DB> ToSql<SmallInt, DB> for AuditLogKind
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        <i16 as ToSql<SmallInt, DB>>::to_sql(&(*self as i16), out)
    }
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuditLog {
    pub id: i32,
    pub kind: AuditLogKind,
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub table_name: String,
    pub changes: String,
    pub created_at: NaiveDateTime,
}

impl AuditLog {
    pub fn by_id(audit_log_id: i32, conn: &PgConnection) -> Result<AuditLog, DieselError> {
        use crate::schema::audit_logs::dsl::*;

        audit_logs
            .filter(id.eq(audit_log_id))
            .first::<AuditLog>(conn)
            .map_err(|err| {
                error!("Couldn't query audit_log by id {:?}: {}", audit_log_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn list_by_room_id(
        room_id_query: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<AuditLog>, DieselError> {
        use crate::schema::audit_logs::dsl::*;

        // TODO: pagination
        const LIMIT: i64 = 20;

        audit_logs
            .filter(room_id.eq(room_id_query))
            .limit(LIMIT)
            .load::<AuditLog>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query audit_logs by room_id {:?}: {}",
                    room_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn list_by_user_id(
        user_id_query: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<AuditLog>, DieselError> {
        use crate::schema::audit_logs::dsl::*;

        // TODO: pagination
        const LIMIT: i64 = 20;

        audit_logs
            .filter(user_id.eq(user_id_query))
            .limit(LIMIT)
            .load::<AuditLog>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query audit_logs by user_id {:?}: {}",
                    user_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::audit_logs::dsl::*;

        diesel::delete(audit_logs.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove audit log {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }

    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<AuditLog, DieselError> {
        use crate::schema::audit_logs::dsl::*;

        diesel::update(audit_logs)
            .set(self)
            .get_result::<AuditLog>(conn)
            .map_err(|err| {
                error!("Couldn't update audit log {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}


#[derive(Insertable, AsChangeset, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "audit_logs"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewAuditLog<'a> {
    pub kind: AuditLogKind,
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub table_name: &'a str,
    pub changes: &'a str,
    pub created_at: NaiveDateTime,
}

impl<'a> NewAuditLog<'a> {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<AuditLog, DieselError> {
        use crate::schema::audit_logs::dsl::*;

        diesel::insert_into(audit_logs)
            .values(self)
            .get_result::<AuditLog>(conn)
            .map_err(|err| {
                error!("Couldn't create auditlog {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
