CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS files (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    hash VARCHAR NOT NULL,
    ext VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL UNIQUE,
    nickname VARCHAR,
    email VARCHAR UNIQUE,
    password VARCHAR,
    color VARCHAR,
    file_id INT REFERENCES files(id) ON DELETE CASCADE,
    is_admin BOOLEAN NOT NULL DEFAULT 'f',
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rooms (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, 
    file_id INT NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    url VARCHAR
);

CREATE TABLE IF NOT EXISTS videos (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    subtitles_id INT REFERENCES files(id) ON DELETE SET NULL,
    file_id INT REFERENCES files(id) ON DELETE CASCADE,
    url VARCHAR,
    title VARCHAR,
    duration INTEGER,
    is_raw BOOLEAN NOT NULL DEFAULT 'f',
    is_iframe BOOLEAN NOT NULL DEFAULT 'f',
    is_live BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS roles (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    name VARCHAR NOT NULL,
    color VARCHAR,
    is_default BOOLEAN NOT NULL DEFAULT 'f',
    position INTEGER NOT NULL DEFAULT 999,

    title_update INTEGER NOT NULL DEFAULT -1,
    path_update INTEGER NOT NULL DEFAULT -1,
    public_update INTEGER NOT NULL DEFAULT -1,
    room_delete INTEGER NOT NULL DEFAULT -1,
    audit_log_read INTEGER NOT NULL DEFAULT -1,

    embed_links INTEGER NOT NULL DEFAULT -1,
    ping_everyone INTEGER NOT NULL DEFAULT -1,

    password_create INTEGER NOT NULL DEFAULT -1,
    password_update INTEGER NOT NULL DEFAULT -1,
    password_delete INTEGER NOT NULL DEFAULT -1,

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
    message_history_read INTEGER NOT NULL DEFAULT -1,
    message_timeout INTEGER NOT NULL DEFAULT -1,

    user_kick INTEGER NOT NULL DEFAULT -1,
    user_ban INTEGER NOT NULL DEFAULT -1,
    user_unban INTEGER NOT NULL DEFAULT -1,
    user_timeout INTEGER NOT NULL DEFAULT -1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS emotes (
    id UUID NOT NULL PRIMARY KEY, 
    name VARCHAR NOT NULL,
    file_id INT NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    is_global BOOLEAN NOT NULL DEFAULT 'f',
    is_deleted BOOLEAN NOT NULL DEFAULT 'f',
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Messages

-- Base channel table
CREATE TABLE IF NOT EXISTS channels (
    id UUID NOT NULL PRIMARY KEY,
    deleted_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Channel of type ROOM
CREATE TABLE IF NOT EXISTS room_channels (
    id UUID NOT NULL PRIMARY KEY,
    channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE
);
-- Channel of type DM
CREATE TABLE IF NOT EXISTS dm_channels (
    id UUID NOT NULL PRIMARY KEY,
    channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE
);

-- dm participants
CREATE TABLE IF NOT EXISTS dm_channel_users (
    id UUID NOT NULL PRIMARY KEY, 
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    dm_channel_id UUID NOT NULL REFERENCES dm_channels(id) ON DELETE CASCADE
);

-- message itself
CREATE TABLE IF NOT EXISTS messages (
    id UUID NOT NULL PRIMARY KEY, 
    channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- message mentions
CREATE TABLE IF NOT EXISTS message_mentions (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE
);

-- bans and timeouts
CREATE TABLE IF NOT EXISTS restrains (
    id UUID NOT NULL PRIMARY KEY, 
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    ip VARCHAR,
    fingerprint VARCHAR,
    channels_id UUID REFERENCES channels(id) ON DELETE CASCADE,

    is_global BOOLEAN NOT NULL DEFAULT 'f',
    is_ban BOOLEAN NOT NULL DEFAULT 'f',
    ending_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    -- 0 - add
    -- 1 - change
    -- 2 - delete
    kind SMALLSERIAL NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,

    table_name VARCHAR NOT NULL,
    changes VARCHAR NOT NULL
);
