#[cfg(not(feature = "deterministic"))]
use crate::version_data::VersionData;

use image::{load_from_memory_with_format, ImageFormat, RgbaImage};
use rust_embed::RustEmbed;
use serenity::framework::standard::CommandResult;

#[derive(RustEmbed)]
#[folder = "$RESOURCE_PATH"]
struct Assets;

#[cfg(not(feature = "deterministic"))]
pub fn version() -> CommandResult<VersionData> {
    let ver = &Assets::get("version.json").ok_or("version.json not embedded for some reason")?;

    let version = serde_json::from_slice::<VersionData>(&ver)?;

    Ok(version)
}

pub fn six_choice_background() -> CommandResult<RgbaImage> {
    let res = Assets::get("avatars.png").unwrap();
    let dynimg = load_from_memory_with_format(&*res, ImageFormat::Png)?;
    Ok(dynimg.to_rgba8())
}
