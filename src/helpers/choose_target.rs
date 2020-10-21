use super::Error;
use crate::data::{Cdn, ReqwestClient};
use crate::game::RoleName;
use image::{
    self, imageops, load_from_memory_with_format, png::PngEncoder, ColorType, ImageFormat,
};
use serenity::http::AttachmentType;
use serenity::{builder::CreateEmbed, model::id::UserId, prelude::*};
use tracing::{error, warn};

type Image = image::RgbaImage;

mod res {
    use super::{Error, Image};
    use image::{load_from_memory_with_format, ImageFormat};
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "res/"]
    struct ResFolder;

    pub fn get_background() -> Result<Image, Error> {
        let res = ResFolder::get("avatars.png").unwrap();
        let dynimg = load_from_memory_with_format(&*res, ImageFormat::Png)?;
        Ok(dynimg.to_rgba())
    }
}

/// This is the function used to generate the embed for the King or Revolutionary role action
pub async fn build_embed_for_target_choice(
    ctx: &Context,
    players: &Vec<UserId>,
    role: RoleName,
) -> Result<CreateEmbed, Error> {
    assert!(![RoleName::Sorcerer, RoleName::Knight].contains(&role));

    let avatars = fetch_avatars(ctx, players).await?;

    let merged_avatars = merge_avatars(avatars)?;
    let merged_avatars_png = encode_to_png(merged_avatars)?;
    let foo = AttachmentType::Bytes {
        data: merged_avatars_png.into(),
        filename: "avatars.png".into(),
    };

    let data = ctx.data.read().await;
    let cdn = data.get::<Cdn>().expect("Where's my CDN");

    let msg = cdn.send_message(ctx, |m| m.add_file(foo)).await?;

    let mut embed = CreateEmbed::default();
    embed.title(if role.is_king_like() {
        "Please select a target for 「 Murder 」"
    } else {
        "Please select a target for 「 Assassination 」"
    });
    embed.image(msg.attachments[0].url.clone());

    Ok(embed)
}

async fn fetch_avatars(ctx: &Context, players: &Vec<UserId>) -> Result<Vec<Image>, Error> {
    let data = ctx.data.write().await;
    let reqwest = data.get::<ReqwestClient>().unwrap_or_else(|| {
        error!("Reqwest client wasn't in ctx.data for some reason");
        panic!();
    });

    let mut avatars = Vec::with_capacity(6);

    for player in players {
        let user = player.to_user(ctx).await?;

        let (image_url, image_format) = match user.static_avatar_url() {
            Some(webp_url) => (webp_url, ImageFormat::WebP),
            None => (user.default_avatar_url(), ImageFormat::Png),
        };

        let image_request = reqwest.get(&image_url).build()?;
        let response = reqwest.execute(image_request).await?;
        let raw_image = response.bytes().await?;

        let image = load_from_memory_with_format(&raw_image, image_format)?;

        let image = image.into_rgba();
        avatars.push(image);
    }

    Ok(avatars)
}

fn merge_avatars(mut avatars: Vec<Image>) -> Result<Image, Error> {
    avatars.truncate(6);

    const IMAGE_LEN: u32 = 512;

    let mut resized_avatars = Vec::with_capacity(6);

    for ava in &avatars {
        resized_avatars.push(imageops::resize(
            ava,
            IMAGE_LEN,
            IMAGE_LEN,
            imageops::Nearest,
        ));
    }

    let mut image = res::get_background()?;

    warn!(" w:{} h:{}", image.width(), image.height());

    let mut offset = 0;

    for ava in &resized_avatars {
        for (x, y, pixel) in ava.enumerate_pixels() {
            image.put_pixel(x + offset, y, *pixel);
        }
        offset += IMAGE_LEN;
    }

    Ok(image)
}

fn encode_to_png(img: Image) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![];
    let encoder = PngEncoder::new(&mut buffer);

    encoder.encode(img.as_raw(), img.width(), img.height(), ColorType::Rgba8)?;

    Ok(buffer)
}
