use std::collections::BTreeMap;

use super::Error;
use crate::{
    data::{Cdn, ReqwestClient},
    game::Player,
};
use image::{
    self, imageops, imageops::colorops, load_from_memory_with_format, png::PngEncoder, ColorType,
    DynamicImage, ImageFormat,
};
use libwebp_image::webp_load_from_memory;
use serenity::{builder::CreateEmbed, http::AttachmentType, model::id::UserId, prelude::*};
use tracing::{error, info, warn};

type Image = image::RgbaImage;

/// Trait that exists to make !testk's existence easier on me
pub trait Players: Sync {
    fn players(&self) -> Vec<Player>;
    fn player_ids(&self) -> Vec<UserId>;
}

/// Impl for the type used to store players by TimeBlocks
impl Players for BTreeMap<UserId, Player> {
    fn players(&self) -> Vec<Player> {
        let mut players = vec![];
        for player in self.values() {
            players.push(player.clone());
        }
        players
    }

    fn player_ids(&self) -> Vec<UserId> {
        let mut ids = vec![];
        for id in self.keys() {
            ids.push(*id);
        }
        ids
    }
}

/// This function takes UserIds and generates an Embed containing
/// * the avatars of the users indicated by those UserIds, placed one after each other
/// * emojis indicating with what reaction you need to reply to select a specific player
/// * flavour text in the embed title
pub async fn build_embed_for_target_choice(
    ctx: &Context,
    players: &dyn Players,
    embed_title: &str,
) -> Result<CreateEmbed, Error> {
    info!("Fetching avatars...");
    let avatars = fetch_avatars(ctx, &players.player_ids()).await?;

    let alivenesses = {
        let mut a = vec![];
        for player in &players.players() {
            a.push(player.is_alive());
        }
        a
    };

    let merged_avatars = tokio::task::spawn_blocking(move || -> Result<AttachmentType, Error> {
        info!("Grayscaling avatars...");
        let avatars = grayscale_dead_players(avatars, alivenesses);
        info!("Merging avatars...");
        let background_image = make_background_image()?;
        let merged_avatars = merge_avatars(avatars, background_image)?;
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

fn grayscale_dead_players(mut avatars: Vec<Image>, alivenesses: Vec<bool>) -> Vec<Image> {
    for (avatar, alive) in avatars.iter_mut().zip(alivenesses.iter()) {
        if !alive {
            *avatar = DynamicImage::ImageLuma8(colorops::grayscale(avatar)).to_rgba8();
        }
    }

    avatars
}

const IMAGE_WIDTH: u32 = 512;

fn make_background_image() -> Result<Image, Error> {
    let mut res = DynamicImage::new_rgba8(6 * IMAGE_WIDTH, 764).to_rgba8();
    let mut offset = 0;

    for i in 1..7 {
        let img = crate::resources::number_reactions(i)?;
        for (x, y, pixel) in img.enumerate_pixels() {
            res.put_pixel(offset + x, y, *pixel);
        }
        offset += IMAGE_WIDTH;
    }

    Ok(res)
}

/// This function creates an image big enough to contain the first 6 images in the vector,
/// putting them one after each other after they got resized to be 512x512
fn merge_avatars(mut avatars: Vec<Image>, mut image: Image) -> Result<Image, Error> {
    avatars.truncate(6);

    let mut resized_avatars = Vec::with_capacity(6);

    for ava in &avatars {
        resized_avatars.push(imageops::resize(
            ava,
            IMAGE_WIDTH,
            IMAGE_WIDTH,
            imageops::Nearest,
        ));
    }

    warn!(" w:{} h:{}", image.width(), image.height());

    let mut offset = 0;

    for ava in &resized_avatars {
        for (x, y, pixel) in ava.enumerate_pixels() {
            image.put_pixel(x + offset, y, *pixel);
        }
        offset += IMAGE_WIDTH;
    }

    Ok(image)
}

fn encode_to_png(img: Image) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![];
    let encoder = PngEncoder::new(&mut buffer);

    encoder.encode(img.as_raw(), img.width(), img.height(), ColorType::Rgba8)?;

    Ok(buffer)
}
