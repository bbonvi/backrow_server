use super::DieselError;
use crate::schema::subtitles;
use crate::schema::videos;

use crate::diesel::prelude::*;
use crate::diesel::*;
use std::vec::Vec;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub room_id: String,

    #[serde(skip_serializing)]
    pub subtitles_id: Option<String>,

    #[serde(skip_serializing)]
    pub file_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    pub is_raw: bool,
    pub is_iframe: bool,
    pub is_live: bool,
    pub created_at: NaiveDateTime,
}

impl Video {
    pub fn list_by_room_id(
        room_id_query: String,
        conn: &PgConnection,
    ) -> Result<Vec<Video>, DieselError> {
        use crate::schema::videos::dsl::*;

        videos
            .filter(room_id.eq(room_id_query.clone()))
            .load::<Video>(conn)
            .map_err(|err| {
                error!(
                    "Couldn't query videos by room id {:?}: {}",
                    room_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn by_id(video_id: String, conn: &PgConnection) -> Result<Video, DieselError> {
        use crate::schema::videos::dsl::*;

        videos
            .filter(id.eq(video_id.clone()))
            .first::<Video>(conn)
            .map_err(|err| {
                error!("Couldn't query video by id {:?}: {}", video_id, err);
                err
            })
            .map_err(From::from)
    }

    pub fn delete_all_by_room_id(
        room_id_query: String,
        conn: &PgConnection,
    ) -> Result<usize, DieselError> {
        use crate::schema::videos::dsl::*;

        let target = videos.filter(room_id.eq(room_id_query.clone()));

        diesel::delete(target)
            .execute(conn)
            .map_err(|err| {
                error!(
                    "Couldn't delete videos by room_id {:?}: {}",
                    room_id_query, err
                );
                err
            })
            .map_err(From::from)
    }

    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::videos::dsl::*;

        diesel::delete(videos.filter(id.eq(self.id.to_owned())))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't delete video {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(Insertable, AsChangeset, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "videos"]
#[serde(rename_all = "camelCase")]
pub struct NewVideo {
    pub room_id: String,
    pub subtitles_id: Option<String>,
    pub file_id: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub duration: Option<i32>,
    pub is_raw: bool,
    pub is_iframe: bool,
    pub is_live: bool,
}

impl NewVideo {
    /// Create multiple NewVideos
    pub fn bulk_create(
        new_videos: Vec<NewVideo>,
        conn: &PgConnection,
    ) -> Result<Vec<Video>, DieselError> {
        // Use transction for performance reasons.
        conn.transaction(|| {
            let mut result: Vec<Video> = Vec::new();
            for new_video in new_videos {
                let created_video = new_video.create(conn)?;
                result.push(created_video);
            }

            Ok(result)
        })
    }
    pub fn create(&self, conn: &PgConnection) -> Result<Video, DieselError> {
        use crate::schema::videos::dsl::*;

        diesel::insert_into(videos)
            .values(self)
            .get_result::<Video>(conn)
            .map_err(|err| {
                error!("Couldn't create video {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(AsChangeset, Associations, Queryable, Debug, Identifiable, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "subtitles"]
pub struct Subtitles {
    pub id: String,
    pub file_id: String,
    pub url: Option<String>,
}

impl Subtitles {
    pub fn by_id(subtitles_id: String, conn: &PgConnection) -> Result<Subtitles, DieselError> {
        use crate::schema::subtitles::dsl::*;

        subtitles
            .filter(id.eq(subtitles_id.clone()))
            .first::<Subtitles>(conn)
            .map_err(|err| {
                error!("Couldn't query subtitles by id {:?}: {}", subtitles_id, err);
                err
            })
            .map_err(From::from)
    }
    pub fn delete(&self, conn: &PgConnection) -> Result<usize, DieselError> {
        use crate::schema::subtitles::dsl::*;
        let query_id = self.id.to_owned();

        diesel::delete(subtitles.filter(id.eq(query_id)))
            .execute(conn)
            .map_err(|err| {
                error!("Couldn't delete subtitles {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}

#[derive(Insertable, AsChangeset, AsExpression, Debug, Associations, Deserialize, Serialize)]
#[table_name = "subtitles"]
#[serde(rename_all = "camelCase")]
pub struct NewSubtitles {
    pub file_id: String,
    pub url: Option<String>,
}

impl NewSubtitles {
    pub fn create(&self, conn: &PgConnection) -> Result<Subtitles, DieselError> {
        use crate::schema::subtitles::dsl::*;

        diesel::insert_into(subtitles)
            .values(self)
            .get_result::<Subtitles>(conn)
            .map_err(|err| {
                error!("Couldn't create subtitles {:?}: {}", self, err);
                err
            })
            .map_err(From::from)
    }
}
