/*
 *  Copyright (C) 2025 Alex Olemans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, version 3 of the License.
 *
 *  This program is distributed WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
 *
 *  This software may be subject to the AGPLv3 license if it is used as a service over a network,
 *  as defined by the AGPLv3 license.
 */

use crate::commands::{Context, Error};
use mongodb::Collection;
use mongodb::bson::{Document, doc};
use serenity::all::{CreateEmbedFooter, CreateInteractionResponse, EditMessage};
use serenity::builder::CreateEmbed;
use std::time::Duration;
use tracing::info;

/// Create a play session
#[poise::command(slash_command)]
pub async fn play(ctx: Context<'_>) -> Result<(), Error> {
    let time = std::time::Instant::now();
    let db = &ctx.data().db;

    // Checks if the user has an account
    // It creates a new account if the user doesn't have one
    let collection_user: Collection<Document> = db.collection("users");
    let user = collection_user
        .find_one(doc! {"user_id": ctx.author().id.to_string()})
        .await?;
    let mut counter: i64;
    if user.is_none() {
        create_user(ctx, collection_user.clone()).await?;
        counter = 0;
    } else {
        counter = user.clone().unwrap().get_i64("counter").unwrap();

        // Update the username in the database if it's different
        if user.clone().unwrap().get_str("username").unwrap() != ctx.author().name {
            collection_user
                .update_one(
                    doc! {
                        "user_id": ctx.author().id.to_string()
                    },
                    doc! {
                        "$set": {
                            "username": ctx.author().name.clone()
                        }
                    },
                )
                .await?;
            info!("Updated username for {}", ctx.author().id);
        }

        // Update the avatar url in the database if it's different
        if user.unwrap().get_str("avatar_url").unwrap()
            != ctx.author().avatar_url().unwrap_or_default()
        {
            collection_user
                .update_one(
                    doc! {
                        "user_id": ctx.author().id.to_string()
                    },
                    doc! {
                        "$set": {
                            "avatar_url": ctx.author().avatar_url().unwrap_or_default()
                        }
                    },
                )
                .await?;
            info!("Updated avatar url for {}", ctx.author().id);
        }
    }

    info!("Creating a new session for {}", ctx.author().id);

    let embed = make_embed(ctx, counter);

    let builder = poise::reply::CreateReply::default()
        .embed(embed)
        .components(vec![poise::serenity_prelude::CreateActionRow::Buttons(
            vec![
                poise::serenity_prelude::CreateButton::new("click")
                    .label("🔘")
                    .style(poise::serenity_prelude::ButtonStyle::Primary),
                poise::serenity_prelude::CreateButton::new("delete")
                    .label("✖️")
                    .style(poise::serenity_prelude::ButtonStyle::Danger),
            ],
        )]);

    let msg = ctx.send(builder).await?.into_message().await?;

    loop {
        let interaction = msg
            .await_component_interactions(ctx)
            .timeout(Duration::from_secs(3600))
            .await;

        match interaction {
            Some(interaction) => {
                // Completely ignore if the interaction doesn't come from the message author
                if interaction.user.id != ctx.author().id {
                    // Next time avoid Discord to send an "Interaction failed" message
                    interaction
                        .create_response(ctx, CreateInteractionResponse::Acknowledge)
                        .await?;
                    continue;
                }

                let interaction_time = std::time::Instant::now();

                // Delete the message if the user clicks on the stop session button
                if interaction.data.custom_id.as_str() != "click" {
                    msg.delete(ctx).await?;
                    break;
                }

                increase_counter(ctx, collection_user.clone(), &mut counter).await?;

                let mut new_msg = interaction.message.clone();
                new_msg
                    .edit(ctx, EditMessage::new().embed(make_embed(ctx, counter)))
                    .await?;

                interaction
                    .create_response(ctx, CreateInteractionResponse::Acknowledge)
                    .await?;

                info!(
                    "Increase Counter for {} | Time: {:?}",
                    ctx.author().id.to_string(),
                    interaction_time.elapsed()
                );
            }

            None => {
                msg.delete(ctx).await?;
                break;
            }
        }
    }

    info!(
        "Terminating session for {} | Time {:?}",
        ctx.author().id,
        time.elapsed()
    );
    Ok(())
}

fn make_embed(ctx: Context<'_>, counter: i64) -> CreateEmbed {
    let thumbnail = match ctx.author().avatar_url() {
        Some(url) => url,
        None => ctx.author().default_avatar_url(),
    };
    let footer = CreateEmbedFooter::new("Click the button to increase your score!");
    CreateEmbed::new()
        .title(format!("__{}'s session__", ctx.author().name))
        .description(format!("Current Score: **{}**", counter))
        .color(0x5754d0)
        .thumbnail(thumbnail)
        .footer(footer)
}

pub async fn create_user(ctx: Context<'_>, collection: Collection<Document>) -> Result<(), Error> {
    collection
        .insert_one(doc! {
            "user_id": ctx.author().id.to_string(),
            "username": ctx.author().name.clone(),
            "avatar_url": ctx.author().avatar_url().unwrap_or_default(),
            "counter": 0i64
        })
        .await?;

    Ok(())
}

async fn increase_counter(
    ctx: Context<'_>,
    collection: Collection<Document>,
    counter: &mut i64,
) -> Result<(), Error> {
    collection
        .update_one(
            doc! {
                "user_id": ctx.author().id.to_string()
            },
            doc! {
                "$inc": {
                    "counter": 1
                }
            },
        )
        .await?;

    // Update the counter, fetches directly from the DB as it might be updated by another session
    *counter = fetch_score(ctx, collection).await?;

    Ok(())
}

async fn fetch_score(ctx: Context<'_>, collection: Collection<Document>) -> Result<i64, Error> {
    let user = collection
        .find_one(doc! {"user_id": ctx.author().id.to_string()})
        .await?;
    if user.is_none() {
        return Ok(0);
    }
    let user_counter = user.unwrap().get_i64("counter").unwrap();
    Ok(user_counter)
}
