#[derive(sqlx::Type, PartialEq)]
#[sqlx(type_name = "game_state")]
#[sqlx(rename_all = "lowercase")]
pub enum GameState {
    ABlock,
    BBlock,
    CBlock,
    DBlock,
    EBlock,
    FBlock,
    GameEnded,
}

pub struct Meeting {
    pub id: i64,
    pub guild_id: i64,
    pub host: i64,
    pub guest: i64,
    pub channel: i64,
    pub day: i32,
}

pub struct RunningGame {
    pub guild_id: i64,
    pub players: Vec<i64>,
    pub gstate: GameState,
    pub day: i32,
}
