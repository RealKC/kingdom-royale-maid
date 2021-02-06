-- Add migration script here

DROP TABLE running_games;
DROP TABLE secret_meetings;
DROP TYPE game_state;

CREATE TYPE game_state AS ENUM (
    'notstarted',
    'ablock',
    'bblock',
    'cblock',
    'dblock',
    'eblock',
    'fblock',
    'gameended'
);

CREATE TABLE IF NOT EXISTS public.running_games (
    guild_id bigint PRIMARY KEY,
    players bigint ARRAY[6] NOT NULL,
    gstate game_state NOT NULL,
    day int NOT NULL
)

TABLESPACE pg_default;

CREATE TABLE IF NOT EXISTS public.secret_meetings (
    id bigserial PRIMARY KEY,
    guild_id bigint NOT NULL,
    host bigint NOT NULL,
    guest bigint NOT NULL,
    channel bigint NOT NULL,
    day int NOT NULL
)
