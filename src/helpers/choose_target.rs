use super::Error;
use crate::data::{Cdn, ReqwestClient};
use image::{
    self, imageops, load_from_memory_with_format, png::PngEncoder, ColorType, ImageFormat,
};
use libwebp_image::webp_load_from_memory;
use serenity::{builder::CreateEmbed, http::AttachmentType, model::id::UserId, prelude::*};
use tracing::{error, info, warn};

type Image = image::RgbaImage;

/// This function takes UserIds and generates an Embed containing
/// * the avatars of the users indicated by those UserIds, placed one after each other
/// * emojis indicating with what reaction you need to reply to select a specific player
/// * flavour text in the embed title
pub async fn build_embed_for_target_choice(
    ctx: &Context,
    players: &[UserId],
    embed_title: &str,
) -> Result<CreateEmbed, Error> {
    info!("Fetching avatars...");
    let avatars = fetch_avatars(ctx, players).await?;

    let merged_avatars = tokio::task::spawn_blocking(move || -> Result<AttachmentType, Error> {
        info!("Merging avatars...");
        let merged_avatars = merge_avatars(avatars)?;
        let merged_avatars_png = encode_to_png(merged_avatars)?;
        Ok(AttachmentType::Bytes {
            data: merged_avatars_png.into(),
            filename: "avatars.png".into(),
        })
    })
    .await??;

    let cdn = {
        info!("build_embed_for_target_choice: trying to lock data");
        let data = ctx.data.read().await;
        info!("build_embed_for_target_choice: Data locked");
        *data.get::<Cdn>().expect("Where's my CDN")
    };

    let msg = cdn
        .send_message(ctx, |m| m.add_file(merged_avatars))
        .await?;

    let mut embed = CreateEmbed::default();
    embed.title(embed_title);
    embed.image(msg.attachments[0].url.clone());

    Ok(embed)
}

async fn fetch_avatars(ctx: &Context, players: &[UserId]) -> Result<Vec<Image>, Error> {
    let data = ctx.data.read().await;
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

        let image = match image_format {
            // `image` chokes on webp's with alpha in them, so we `libwebp-image` in order to workaround that.
            // This also gives us colour support, which `image` doesn't have
            // Pretty cool overall, sadly that's a few more deps though
            ImageFormat::WebP => webp_load_from_memory(&raw_image)?,
            ImageFormat::Png => load_from_memory_with_format(&raw_image, ImageFormat::Png)?,
            _ => unreachable!("This image format variable should only ever be Webp or Png"),
        };

        let image = image.into_rgba8();
        avatars.push(image);
    }

    Ok(avatars)
}

/// This function creates an image big enough to contain the first 6 images in the vector,
/// putting them one after each other after they got resized to be 512x512
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

    let mut image = crate::resources::six_choice_background()?;

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
