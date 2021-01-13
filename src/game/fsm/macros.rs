#[macro_export]
macro_rules! for_all_blocks {
    ($matcher:expr, $n:ident, $e:expr) => {
        match $matcher {
            Wrapper::ABlock($n) => Some($e),
            Wrapper::BBlock($n) => Some($e),
            Wrapper::CBlock($n) => Some($e),
            Wrapper::DBlock($n) => Some($e),
            Wrapper::EBlock($n) => Some($e),
            Wrapper::FBlock($n) => Some($e),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! impl_wrap {
    ($name:ident) => {
        impl Wrap for GameMachine<$name> {
            fn wrap(self) -> Wrapper {
                Wrapper::$name(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_common_state_boilerplate {
    () => {
        pub(super) fn set_king_substitution_status(&mut self, kss: SubstitutionStatus) {
            self.state.king_substitution_status = kss;
        }

        pub(super) fn king_has_substituted(&self) -> bool {
            matches!(
                self.state.king_substitution_status,
                SubstitutionStatus::Has | SubstitutionStatus::CurrentlyIs
            )
        }
    };
}

#[macro_export]
macro_rules! impl_timeblock {
    ($name:ident) => {
        impl TimeBlock for $name {
            fn day(&self) -> u8 {
                self.day
            }

            fn players(&self) -> &BTreeMap<UserId, Player> {
                &self.players
            }

            fn players_mut(&mut self) -> &mut BTreeMap<UserId, Player> {
                &mut self.players
            }
        }
    };
}

pub mod state {
    pub use crate::{for_all_blocks, impl_common_state_boilerplate, impl_timeblock, impl_wrap};
}

#[macro_export]
macro_rules! expect_game {
    ($ctx:ident, $func: literal) => {{
        let game = $ctx
            .data
            .read()
            .await
            .get::<crate::commands::game::GameContainer>()
            .cloned();

        if let Some(game) = game {
            game
        } else {
            warn!(
                "{} woke up but no game is running. (Game likely ended)",
                $func
            );
            return;
        }
    }};
}

pub mod tasks {
    pub use crate::expect_game;
}
