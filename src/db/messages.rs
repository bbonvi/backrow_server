use super::DieselError;
use crate::schema::message_mentions;
use crate::schema::messages;

use crate::diesel::prelude::*;
use crate::diesel::*;
use std::vec::Vec;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = serde_json::to_string(self).unwrap();
        write!(f, "{}", message)
    }
}

impl Message {
    // TODO: pagination
    pub fn list_by_channel_id(
        message_channel_id: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<Message>, DieselError> {
        const LIMIT: i64 = 20;
        use crate::schema::messages::dsl::*;

        messages
            .filter(channel_id.eq(message_channel_id))
            .limit(LIMIT)
            .load::<Message>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query messages by channel_id {:?}: {}",
                    message_channel_id, err
                );
                err
            })
            .map_err(From::from)
    }
    pub fn list_by_room_id(
        room_id_query: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<Message>, DieselError> {
        use crate::schema::messages::dsl::*;
        const LIMIT: i64 = 20;

        conn.transaction(|| {
            let room_channel = super::RoomChannel::by_room_id(room_id_query, conn)?;
            let primary_channel = super::Channel::by_id(room_channel.channel_id, conn)?;

            messages
                .filter(channel_id.eq(primary_channel.id))
                .limit(LIMIT)
                .load::<Message>(conn)
                .map_err(|err| {
                    error!(
                        "Couldn't query messages by room_id {:?}: {}",
                        room_id_query, err
                    );
                    err
                })
                .map_err(From::from)
        })
    }
    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::messages::dsl::*;

        diesel::delete(messages.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove message {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<Message, DieselError> {
        use crate::schema::messages::dsl::*;

        diesel::update(messages)
            .set(self)
            .get_result::<Message>(conn)
            .map_err(|err| {
                error!("Couldn't update message {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "messages"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewMessage<'a> {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub content: &'a str,
}

impl<'a> fmt::Display for NewMessage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let new_message = serde_json::to_string(self).unwrap();
        write!(f, "{}", new_message)
    }
}

impl<'a> NewMessage<'a> {
    pub fn create_with_mentions(
        self: &'_ Self,
        mentions: Vec<crate::db::User>,
        conn: &PgConnection,
    ) -> Result<Message, DieselError> {
        use crate::schema::messages::dsl::*;

        conn.transaction(|| {
            let result_message = diesel::insert_into(messages)
                .values(self)
                .get_result::<Message>(conn)
                .map_err(|err| {
                    error!("Couldn't create message {}: {}", self, err);
                    err
                })?;

            let mut mentions_iterator = mentions.iter();
            if let Some(user) = mentions_iterator.next() {
                let new_mention = NewMessageMention {
                    user_id: user.id,
                    message_id: result_message.id,
                };

                new_mention.create(conn)?;
            }

            Ok(result_message)
        })
    }
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<Message, DieselError> {
        use crate::schema::messages::dsl::*;

        diesel::insert_into(messages)
            .values(self)
            .get_result::<Message>(conn)
            .map_err(|err| {
                error!("Couldn't create message {}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "message_mentions"]
pub struct MessageMention {
    pub id: i32,
    pub user_id: Uuid,
    pub message_id: Uuid,
}

impl MessageMention {
    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::message_mentions::dsl::*;

        diesel::delete(message_mentions.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't remove message {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, AsExpression, Insertable, Debug, Associations, Deserialize, Serialize)]
#[table_name = "message_mentions"]
// We only need camelCase for consistent debug output
#[serde(rename_all = "camelCase")]
pub struct NewMessageMention {
    pub user_id: Uuid,
    pub message_id: Uuid,
}
impl NewMessageMention {
    pub fn create(self: &'_ Self, conn: &PgConnection) -> Result<MessageMention, DieselError> {
        use crate::schema::message_mentions::dsl::*;

        diesel::insert_into(message_mentions)
            .values(self)
            .get_result::<MessageMention>(conn)
            .map_err(|err| {
                error!("Couldn't create message mention {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
