CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE sequence IF NOT EXISTS global_id_sequence;

CREATE OR REPLACE FUNCTION id_generator(OUT result VARCHAR) AS $$
DECLARE
    our_epoch bigint := 1314220021721;
    seq_id bigint;
    now_millis bigint;
    r bigint;
    shard_id int := 1;
BEGIN
    SELECT CAST(nextval('global_id_sequence') % 1024 as BIGINT) INTO seq_id;

    SELECT FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;
    r := (now_millis - our_epoch) << 23;
    r := r | (shard_id << 10);
    r := r | (seq_id);
    result := CAST (r as VARCHAR);
END;
$$ LANGUAGE PLPGSQL;

select id_generator();


CREATE TABLE IF NOT EXISTS files (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    hash VARCHAR NOT NULL,
    ext VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    discord_id VARCHAR UNIQUE,
    username VARCHAR NOT NULL UNIQUE,
    nickname VARCHAR,
    email VARCHAR UNIQUE,
    password VARCHAR,
    color VARCHAR,
    file_id VARCHAR REFERENCES files(id) ON DELETE CASCADE,
    is_admin BOOLEAN NOT NULL DEFAULT 'f',
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rooms (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    title VARCHAR NOT NULL,
    path VARCHAR NOT NULL UNIQUE,
    is_public BOOLEAN NOT NULL DEFAULT 't',
    is_deleted BOOLEAN NOT NULL DEFAULT 'f',
    password VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP,
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS subtitles (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    file_id VARCHAR NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    url VARCHAR
);

CREATE TABLE IF NOT EXISTS videos (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    room_id VARCHAR NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    subtitles_id VARCHAR REFERENCES files(id) ON DELETE SET NULL,
    file_id VARCHAR REFERENCES files(id) ON DELETE CASCADE,
    url VARCHAR,
    title VARCHAR,
    duration INTEGER,
    is_raw BOOLEAN NOT NULL DEFAULT 'f',
    is_iframe BOOLEAN NOT NULL DEFAULT 'f',
    is_live BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS roles (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),

    room_id VARCHAR NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    color VARCHAR,
    is_default BOOLEAN NOT NULL DEFAULT 'f',
    position INTEGER NOT NULL DEFAULT 999,

    title_update INTEGER NOT NULL DEFAULT -1,
    path_update INTEGER NOT NULL DEFAULT -1,
    public_update INTEGER NOT NULL DEFAULT -1,
    room_delete INTEGER NOT NULL DEFAULT -1,
    room_view INTEGER NOT NULL DEFAULT -1,
    audit_log_read INTEGER NOT NULL DEFAULT -1,

    embed_links INTEGER NOT NULL DEFAULT -1,
    ping_everyone INTEGER NOT NULL DEFAULT -1,

    password_create INTEGER NOT NULL DEFAULT -1,
    password_update INTEGER NOT NULL DEFAULT -1,
    password_delete INTEGER NOT NULL DEFAULT -1,
    password_bypass INTEGER NOT NULL DEFAULT -1,

    emote_create INTEGER NOT NULL DEFAULT -1,
    emote_update INTEGER NOT NULL DEFAULT -1,
    emote_delete INTEGER NOT NULL DEFAULT -1,
    emote_view INTEGER NOT NULL DEFAULT -1,

    role_create INTEGER NOT NULL DEFAULT -1,
    role_delete INTEGER NOT NULL DEFAULT -1,
    role_update INTEGER NOT NULL DEFAULT -1,
    role_view INTEGER NOT NULL DEFAULT -1,

    video_create INTEGER NOT NULL DEFAULT -1,
    video_delete INTEGER NOT NULL DEFAULT -1,
    video_watch INTEGER NOT NULL DEFAULT -1,
    video_move INTEGER NOT NULL DEFAULT -1,
    video_iframe INTEGER NOT NULL DEFAULT -1,
    video_raw INTEGER NOT NULL DEFAULT -1,

    player_pause INTEGER NOT NULL DEFAULT -1,
    player_resume INTEGER NOT NULL DEFAULT -1,
    player_rewind INTEGER NOT NULL DEFAULT -1,
    subtitles_file INTEGER NOT NULL DEFAULT -1,
    subtitles_embed INTEGER NOT NULL DEFAULT -1,

    message_create INTEGER NOT NULL DEFAULT -1,
    message_read INTEGER NOT NULL DEFAULT -1,
    message_delete INTEGER NOT NULL DEFAULT -1,
    message_history_read INTEGER NOT NULL DEFAULT -1,
    message_timeout INTEGER NOT NULL DEFAULT -1,

    user_kick INTEGER NOT NULL DEFAULT -1,
    user_ban INTEGER NOT NULL DEFAULT -1,
    user_unban INTEGER NOT NULL DEFAULT -1,
    user_timeout INTEGER NOT NULL DEFAULT -1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_roles (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    role_id VARCHAR NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS emotes (
    id VARCHAR NOT NULL PRIMARY KEY, 
    name VARCHAR NOT NULL,
    file_id VARCHAR NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    room_id VARCHAR NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    is_global BOOLEAN NOT NULL DEFAULT 'f',
    is_deleted BOOLEAN NOT NULL DEFAULT 'f',
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Messages

-- Base channel table
CREATE TABLE IF NOT EXISTS channels (
    id VARCHAR NOT NULL PRIMARY KEY,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Channel of type ROOM
CREATE TABLE IF NOT EXISTS room_channels (
    id VARCHAR NOT NULL PRIMARY KEY,
    channel_id VARCHAR NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    room_id VARCHAR NOT NULL REFERENCES rooms(id) ON DELETE CASCADE
);
-- Channel of type DM
CREATE TABLE IF NOT EXISTS dm_channels (
    id VARCHAR NOT NULL PRIMARY KEY,
    channel_id VARCHAR NOT NULL REFERENCES channels(id) ON DELETE CASCADE
);

-- dm participants
CREATE TABLE IF NOT EXISTS dm_channel_users (
    id VARCHAR NOT NULL PRIMARY KEY, 
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    dm_channel_id VARCHAR NOT NULL REFERENCES dm_channels(id) ON DELETE CASCADE
);

-- message itself
CREATE TABLE IF NOT EXISTS messages (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    channel_id VARCHAR NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- message mentions
CREATE TABLE IF NOT EXISTS message_mentions (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message_id VARCHAR NOT NULL REFERENCES messages(id) ON DELETE CASCADE
);

-- bans and timeouts
CREATE TABLE IF NOT EXISTS restrains (
    id VARCHAR NOT NULL PRIMARY KEY, 
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    ip VARCHAR,
    fingerprint VARCHAR,
    channel_id VARCHAR REFERENCES channels(id) ON DELETE CASCADE,

    is_global BOOLEAN NOT NULL DEFAULT 'f',
    is_ban BOOLEAN NOT NULL DEFAULT 'f',
    ending_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id VARCHAR NOT NULL PRIMARY KEY DEFAULT id_generator(),
    -- 0 - add
    -- 1 - change
    -- 2 - delete
    kind SMALLSERIAL NOT NULL,
    user_id VARCHAR NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id VARCHAR NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,

    table_name VARCHAR NOT NULL,
    changes VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
