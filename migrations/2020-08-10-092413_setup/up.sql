CREATE TABLE files (
    id BIGSERIAL NOT NULL PRIMARY KEY, 
    hash VARCHAR NOT NULL,
    ext VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)

CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL,
    nickname VARCHAR,
    email VARCHAR,
    color VARCHAR NOT NULL DEFAULT '#eeeeee',
    image_id BIGSERIAL REFERENCES files(id) ON DELETE CASCADE,
    is_admin BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)

CREATE TABLE rooms (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR NOT NULL,
    path VARCHAR NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT 't',
    is_deleted BOOLEAN NOT NULL DEFAULT 'f',
    password VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
)

CREATE TABLE roles (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id UUID REFERENCES rooms(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT 'f',
    position SMALLSERIAL NULL DEFAULT 999,

    title_update SMALLINT DEFAULT -1,
    path_update SMALLINT DEFAULT -1,
    public_update INTEGER DEFAULT -1,
    room_delete INTEGER DEFAULT -1,

    embed_links INTEGER DEFAULT -1,
    audit_log_read SMALLINT DEFAULT -1,

    password_create SMALLINT DEFAULT -1,
    password_update SMALLINT DEFAULT -1,
    password_delete SMALLINT DEFAULT -1,

    emote_create SMALLINT DEFAULT -1,
    emote_update SMALLINT DEFAULT -1,
    emote_delete SMALLINT DEFAULT -1,
    emote_view SMALLINT DEFAULT -1,

    role_create SMALLINT DEFAULT -1,
    role_delete SMALLINT DEFAULT -1,
    role_update SMALLINT DEFAULT -1,
    role_view SMALLINT DEFAULT -1,

    video_create SMALLINT DEFAULT -1,
    video_delete SMALLINT DEFAULT -1,
    video_watch SMALLINT DEFAULT -1,
    video_move SMALLINT DEFAULT -1,
    video_iframe SMALLINT DEFAULT -1,
    video_raw SMALLINT DEFAULT -1,

    player_pause SMALLINT DEFAULT -1,
    player_resume SMALLINT DEFAULT -1,
    player_rewind SMALLINT DEFAULT -1,
    subtitles_file SMALLINT DEFAULT -1,
    subtitles_embed SMALLINT DEFAULT -1,

    message_create SMALLINT DEFAULT -1,
    message_read SMALLINT DEFAULT -1,
    message_history_read SMALLINT DEFAULT -1,
    message_timeout INTEGER DEFAULT -1,

    user_kick SMALLINT DEFAULT -1,
    user_ban SMALLINT DEFAULT -1,
    user_unban SMALLINT DEFAULT -1,
    user_timeout SMALLINT DEFAULT -1
)

-- "Everyone" - 1002
-- "Anonymous" - 1001
-- "Stranger" - 1000
-- "DJ" - 2
-- "Mod" - 1
-- "Owner" - 0

CREATE TABLE emotes (
    id UUID NOT NULL PRIMARY KEY, 
    name VARCHAR,
    image_id BIGSERIAL REFERENCES files(id) ON DELETE CASCADE,
    room_id UUID REFERENCES rooms(id) ON DELETE CASCADE,
    is_global BOOLEAN NOT NULL DEFAULT 'f',
    is_deleted BOOLEAN NOT NULL DEFAULT 'f',
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)


-- Messages

-- Base channel table
CREATE TABLE channels (
    id UUID NOT NULL PRIMARY KEY,
    deleted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)

-- Channel of type ROOM
CREATE TABLE room_channels (
    id UUID NOT NULL PRIMARY KEY,
    channels_id UUID REFERENCES channels(id) ON DELETE CASCADE,
    room_id UUID REFERENCES rooms(id) ON DELETE CASCADE,
)
-- Channel of type DM
CREATE TABLE dm_channels (
    id UUID NOT NULL PRIMARY KEY,
    channels_id UUID REFERENCES channels(id) ON DELETE CASCADE,
)

-- dm participants
CREATE TABLE dm_channel_users (
    id UUID NOT NULL PRIMARY KEY, 
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    dm_channel_id UUID REFERENCES dm_channels(id) ON DELETE CASCADE,
)

-- message itself
CREATE TABLE messages (
    id UUID NOT NULL PRIMARY KEY, 
    channel_id UUID REFERENCES channels(id) ON DELETE CASCADE,
    content VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)

CREATE TABLE restrictions (
    id UUID NOT NULL PRIMARY KEY, 

    ip VARCHAR,
    fingerprint VARCHAR,
    channels_id UUID REFERENCES channels(id) ON DELETE CASCADE,

    is_global BOOLEAN NOT NULL DEFAULT 'f',
    is_ban BOOLEAN NOT NULL DEFAULT 'f',
    ending_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)

CREATE TABLE audit_logs (
    id BIGSERIAL NOT NULL PRIMARY KEY, 
    -- 0 - add
    -- 1 - change
    -- 2 - delete
    kind SMALLSERIAL NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID REFERENCES rooms(id) ON DELETE CASCADE,

    table_name VARCHAR NOT NULL,
    changes VARCHAR NOT NULL
)
