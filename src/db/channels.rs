use super::DieselError;
use crate::schema::channels;
use crate::schema::dm_channel_users;
use crate::schema::dm_channels;
use crate::schema::room_channels;
use std::vec::Vec;

use crate::diesel::prelude::*;
use crate::diesel::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// Right now, there is two types of channels: DM and Room.
// DM channels have a potentially infinite number of users, but usually 2.
// Room channels don't have users, even though theu have messages, sent by user,
// but they do have reference to a room.
//
// Both types of channels reference primary Channel table, because
// this is where Messages point to.

/// Primary Channel model
#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: Uuid,
    #[serde(skip_serializing)]
    pub deleted_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
}

impl Channel {
    pub fn by_id(channel_id: Uuid, conn: &PgConnection) -> Result<Channel, DieselError> {
        use crate::schema::channels::dsl::*;

        channels
            .filter(id.eq(channel_id))
            .first::<Channel>(conn)
            .map(|channel| {
                debug!(
                    "Channel {:?} has been queried by id {:?}",
                    channel, channel_id
                );
                channel
            })
            .map_err(|err| {
                error!("Couldn't query channel by id {:?}: {}", channel_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::channels::dsl::*;

        diesel::delete(channels.filter(id.eq(self.id)))
            .execute(conn)
            .map(|size| {
                debug!("Channel {:?} has been removed", self);
                size
            })
            .map_err(|err| {
                error!("Couldn't remove channel {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<Channel, DieselError> {
        use crate::schema::channels::dsl::*;

        diesel::update(channels)
            .set(self)
            .get_result::<Channel>(conn)
            .map(|channel| {
                debug!("Channel has been updated: {:?}", channel);
                channel
            })
            .map_err(|err| {
                error!("Couldn't update channel {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "channels"]
pub struct NewChannel;

impl NewChannel {
    pub fn create(conn: &PgConnection) -> Result<Channel, DieselError> {
        use crate::schema::channels::dsl::*;

        diesel::insert_into(channels)
            .default_values()
            .get_result::<Channel>(conn)
            .map(|channel| {
                debug!("Channel has been created: {:?}", channel);
                channel
            })
            .map_err(|err| {
                error!("Couldn't create channel: {}", err);
                err
            })
            .map_err(From::from)
    }
}

/// DM Channel
#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "dm_channels"]
pub struct DmChannel {
    pub id: Uuid,
    pub channel_id: Uuid,
}

impl DmChannel {
    // TODO: implement
}

#[derive(AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "dm_channels"]
pub struct NewDmChannel;
impl NewDmChannel {
    // TODO: implement
}

/// Room Messages Channel.
#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "room_channels"]
pub struct RoomChannel {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub room_id: Uuid,
}

// TODO: pagination
impl RoomChannel {
    pub fn by_id(room_channel_id: Uuid, conn: &PgConnection) -> Result<RoomChannel, DieselError> {
        use crate::schema::room_channels::dsl::*;

        room_channels
            .filter(id.eq(room_channel_id))
            .first::<RoomChannel>(conn)
            .map(|channel| {
                debug!(
                    "Room channel {:?} has been queried by id {:?}",
                    channel, room_channel_id
                );
                channel
            })
            .map_err(|err| {
                error!(
                    "Couldn't query room channel by id {:?}: {}",
                    room_channel_id, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn delete(self: &'_ Self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::room_channels::dsl::*;

        diesel::delete(room_channels.filter(id.eq(self.id)))
            .execute(conn)
            .map(|size| {
                debug!("Room channel {:?} has been removed", self);
                size
            })
            .map_err(|err| {
                error!("Couldn't remove room channel {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
    pub fn update(self: &'_ Self, conn: &PgConnection) -> Result<RoomChannel, DieselError> {
        use crate::schema::room_channels::dsl::*;

        diesel::update(room_channels)
            .set(self)
            .get_result::<RoomChannel>(conn)
            .map(|channel| {
                debug!("Room channel has been updated: {:?}", channel);
                channel
            })
            .map_err(|err| {
                error!("Couldn't update room channel {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, Insertable, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "room_channels"]
pub struct NewRoomChannel {
    pub channel_id: Option<Uuid>,
    pub room_id: Uuid,
}

impl NewRoomChannel {
    pub fn create(mut self, conn: &PgConnection) -> Result<RoomChannel, DieselError> {
        use crate::schema::room_channels::dsl::*;
        conn.transaction(|| {
            let primary_channel = NewChannel::create(conn)?;
            self.channel_id = Some(primary_channel.id);

            diesel::insert_into(room_channels)
                .values(self)
                .get_result::<RoomChannel>(conn)
                .map(|channel| {
                    debug!("Room channel has been created: {:?}", channel);
                    channel
                })
                .map_err(|err| {
                    error!("Couldn't create room channel: {}", err);
                    err
                })
                .map_err(From::from)
        })
    }
}

/// Participant of DM
#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[table_name = "dm_channel_users"]
pub struct DmChannelUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub dm_channel_id: Uuid,
}

impl DmChannelUser {
    // TODO: implement
}

#[derive(AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "dm_channel_users"]
pub struct NewDmChannelUser;

impl NewDmChannelUser {
    // TODO: implement
}
