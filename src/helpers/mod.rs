pub mod choose_target;
pub mod confirm_murder;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
