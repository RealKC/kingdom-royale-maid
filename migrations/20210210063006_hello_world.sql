-- Add migration script here
DROP TABLE running_games;
DROP TABLE secret_meetings;
DROP TYPE game_state;

CREATE TABLE public.whitelisted_guilds (
    guild_id bigint PRIMARY KEY
);

COMMENT ON TABLE public.whitelisted_guilds IS 'List of guilds in which games may be started';

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
    gstate game_state NOT NULL,
    day int NOT NULL
);

CREATE TYPE role AS ENUM (
    'king',
    'prince',
    'the_double',
    'knight',
    'sorcerer',
    'revolutionary'
);

CREATE TABLE IF NOT EXISTS public.players (
    game_id bigint NOT NULL REFERENCES public.running_games(guild_id),
    user_id bigint NOT NULL,
    channel_id bigint NOT NULL,
    alive boolean NOT NULL,
    prole role NOT NULL,
    PRIMARY KEY (game_id, user_id)
);

CREATE TABLE IF NOT EXISTS public.items (
    id serial PRIMARY KEY,
    game_id bigint NOT NULL,
    user_id bigint NOT NULL,
    count int NOT NULL,
    name varchar(64) NOT NULL,
    FOREIGN KEY (game_id, user_id) REFERENCES public.players (game_id, user_id)
);

CREATE TABLE IF NOT EXISTS public.notes (
    id serial PRIMARY KEY,
    game_id bigint NOT NULL,
    user_id bigint NOT NULL,
    when_ varchar(64) NOT NULL,
    text_ varchar(512) NOT NULL,
    is_ripped boolean NOT NULL,
    FOREIGN KEY (game_id, user_id) REFERENCES public.players (game_id, user_id)
);
