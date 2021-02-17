-- Add migration script here
DROP TABLE whitelisted_guilds;
DROP TABLE items;
DROP TABLE notes;
DROP TABLE players;
DROP TABLE running_games;
DROP TABLE lobbies;

CREATE TABLE public.whitelisted_guilds (
    guild_id bigint PRIMARY KEY
);

COMMENT ON TABLE public.whitelisted_guilds IS
    'List of guilds in which games may be started';

CREATE TABLE IF NOT EXISTS public.lobbies (
    guild_id bigint PRIMARY KEY,
    players bigint[6],
    meeting_room bigint NOT NULL,
    announcement_channel bigint NOT NULL,
    host bigint NOT NULL,
    player_role bigint NOT NULL,
    delete_rooms_category_on_game_end boolean NOT NULL
);

COMMENT ON TABLE public.lobbies IS
    'Games that have been created but have yet to start';

-- <https://stackoverflow.com/a/48356064>
-- Web archive: <https://web.archive.org/web/20210210164315/https://stackoverflow.com/questions/7624919/check-if-a-user-defined-type-already-exists-in-postgresql/48356064>
DO $$ BEGIN
    PERFORM 'game_state'::regtype;
    EXCEPTION
        WHEN undefined_object THEN
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
END $$;

DO $$ BEGIN
    PERFORM 'king_substitution_status'::regtype;
    EXCEPTION
        WHEN undefined_object THEN
            CREATE TYPE king_substitution_status AS ENUM (
                'hasnot',
                'currentlyis',
                'has'
            );
END $$;

CREATE TABLE IF NOT EXISTS public.running_games (
    guild_id bigint PRIMARY KEY,
    gstate game_state NOT NULL,
    meeting_room bigint NOT NULL,
    announcement_channel bigint NOT NULL,
    host bigint NOT NULL,
    player_role bigint NOT NULL,
    kss king_substitution_status NOT NULL,
    day int NOT NULL,
    delete_rooms_category_on_game_end boolean NOT NULL
);

DO $$ BEGIN
    PERFORM 'player_role'::regtype;
    EXCEPTION
        WHEN undefined_object THEN
            CREATE TYPE player_role AS ENUM (
                'king',
                'prince',
                'the_double',
                'knight',
                'sorcerer',
                'revolutionary'
            );
END $$;

CREATE TABLE IF NOT EXISTS public.players (
    game_id bigint NOT NULL REFERENCES public.running_games(guild_id),
    user_id bigint NOT NULL,
    channel_id bigint NOT NULL,
    alive boolean NOT NULL,
    prole player_role NOT NULL,
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

CREATE TABLE IF NOT EXISTS public.secret_meetings (
    id serial PRIMARY KEY,
    game_id bigint NOT NULL,
    host bigint NOT NULL,
    visitor bigint NOT NULL,
    channel_id bigint NOT NULL,
    day int NOT NULL
);
