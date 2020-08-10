table! {
    audit_logs (id) {
        id -> Int4,
        kind -> Int2,
        user_id -> Nullable<Uuid>,
        room_id -> Nullable<Uuid>,
        table_name -> Varchar,
        changes -> Varchar,
    }
}

table! {
    channels (id) {
        id -> Uuid,
        deleted_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    dm_channels (id) {
        id -> Uuid,
        channels_id -> Nullable<Uuid>,
    }
}

table! {
    dm_channel_users (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        dm_channel_id -> Nullable<Uuid>,
    }
}

table! {
    emotes (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        image_id -> Nullable<Int4>,
        room_id -> Nullable<Uuid>,
        is_global -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    files (id) {
        id -> Int4,
        hash -> Varchar,
        ext -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    message_mentions (id) {
        id -> Int4,
        user_id -> Nullable<Uuid>,
        messages_id -> Nullable<Uuid>,
    }
}

table! {
    messages (id) {
        id -> Uuid,
        channel_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        content -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    restrains (id) {
        id -> Uuid,
        user_id -> Uuid,
        ip -> Nullable<Varchar>,
        fingerprint -> Nullable<Varchar>,
        channels_id -> Nullable<Uuid>,
        is_global -> Bool,
        is_ban -> Bool,
        ending_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Uuid,
        room_id -> Uuid,
        name -> Varchar,
        is_default -> Bool,
        position -> Int4,
        title_update -> Int4,
        path_update -> Int4,
        public_update -> Int4,
        room_delete -> Int4,
        audit_log_read -> Int4,
        embed_links -> Int4,
        ping_everyone -> Int4,
        password_create -> Int4,
        password_update -> Int4,
        password_delete -> Int4,
        emote_create -> Int4,
        emote_update -> Int4,
        emote_delete -> Int4,
        emote_view -> Int4,
        role_create -> Int4,
        role_delete -> Int4,
        role_update -> Int4,
        role_view -> Int4,
        video_create -> Int4,
        video_delete -> Int4,
        video_watch -> Int4,
        video_move -> Int4,
        video_iframe -> Int4,
        video_raw -> Int4,
        player_pause -> Int4,
        player_resume -> Int4,
        player_rewind -> Int4,
        subtitles_file -> Int4,
        subtitles_embed -> Int4,
        message_create -> Int4,
        message_read -> Int4,
        message_history_read -> Int4,
        message_timeout -> Int4,
        user_kick -> Int4,
        user_ban -> Int4,
        user_unban -> Int4,
        user_timeout -> Int4,
    }
}

table! {
    room_channels (id) {
        id -> Uuid,
        channels_id -> Nullable<Uuid>,
        room_id -> Nullable<Uuid>,
    }
}

table! {
    rooms (id) {
        id -> Uuid,
        title -> Varchar,
        path -> Varchar,
        is_public -> Bool,
        is_deleted -> Bool,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_login -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        nickname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        color -> Varchar,
        image_id -> Nullable<Int4>,
        is_admin -> Bool,
        last_login -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    videos (id) {
        id -> Int4,
        room_id -> Nullable<Uuid>,
        subtitles_id -> Nullable<Int4>,
        url -> Varchar,
        title -> Nullable<Varchar>,
        duration -> Int4,
        is_raw -> Bool,
        is_iframe -> Bool,
        is_live -> Bool,
        created_at -> Timestamp,
    }
}

joinable!(audit_logs -> rooms (room_id));
joinable!(audit_logs -> users (user_id));
joinable!(dm_channel_users -> dm_channels (dm_channel_id));
joinable!(dm_channel_users -> users (user_id));
joinable!(dm_channels -> channels (channels_id));
joinable!(emotes -> files (image_id));
joinable!(emotes -> rooms (room_id));
joinable!(message_mentions -> messages (messages_id));
joinable!(message_mentions -> users (user_id));
joinable!(messages -> channels (channel_id));
joinable!(messages -> users (user_id));
joinable!(restrains -> channels (channels_id));
joinable!(restrains -> users (user_id));
joinable!(roles -> rooms (room_id));
joinable!(room_channels -> channels (channels_id));
joinable!(room_channels -> rooms (room_id));
joinable!(users -> files (image_id));
joinable!(videos -> files (subtitles_id));
joinable!(videos -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    audit_logs,
    channels,
    dm_channels,
    dm_channel_users,
    emotes,
    files,
    message_mentions,
    messages,
    restrains,
    roles,
    room_channels,
    rooms,
    users,
    videos,
);
