#[derive(sqlx::Type, PartialEq, Debug)]
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

#[derive(sqlx::Type, PartialEq, Debug)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
    King,
    Price,
    TheDouble,
    Knight,
    Sorcerer,
    Revolutionary,
}
