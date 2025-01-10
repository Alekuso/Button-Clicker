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

use std::time::Duration;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use serenity::builder::CreateEmbed;
use futures::stream::TryStreamExt;
use serenity::all::{CreateEmbedFooter, CreateInteractionResponse, EditMessage};
use tracing::info;
use crate::commands::{Context, Error};

use crate::MONGO_DB;

enum Filter {
    ScoreAsc,
    ScoreDesc
}

/// View the global leaderboard
#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let db = MONGO_DB.get().unwrap();
    let collection: Collection<Document> = db.collection("users");

    let users = get_users(collection.clone(), Filter::ScoreDesc).await?;

    let embed = make_embed(ctx, users, Filter::ScoreDesc).await;

    let builder = poise::reply::CreateReply::default()
        .embed(embed);
        // .components(vec![
        //     poise::serenity_prelude::CreateActionRow::Buttons(
        //         vec![
        //            poise::serenity_prelude::CreateButton::new("asc").label("⬆️").style(poise::serenity_prelude::ButtonStyle::Secondary),
        //            poise::serenity_prelude::CreateButton::new("desc").label("⬇️").style(poise::serenity_prelude::ButtonStyle::Secondary),
        //         ])
        // ]);
    
    let msg = ctx.send(builder).await?.into_message().await?;


    loop {
        let interaction = msg.await_component_interactions(ctx).timeout(Duration::from_secs(600)).await;

        if interaction.is_none() {
            break;
        }
        let interaction = interaction.unwrap();
        
        let interaction_time = std::time::Instant::now();

        // Handle which button has been pressed
        let embed = match interaction.data.custom_id.as_str() {
            "asc" => {
                let users = get_users(collection.clone(), Filter::ScoreAsc).await?;
                make_embed(ctx, users, Filter::ScoreAsc).await
            },
            "desc" => {
                let users = get_users(collection.clone(), Filter::ScoreDesc).await?;
                make_embed(ctx, users, Filter::ScoreDesc).await
            },
            _ => {
                continue;
            }
        };

        let mut new_msg = interaction.message.clone();
        new_msg.edit(ctx, EditMessage::new().embed(
            embed
        )).await?;

        interaction.create_response(ctx, CreateInteractionResponse::Acknowledge).await?;

        info!("Leaderboard interaction | Time: {:?}", interaction_time.elapsed());
        
    }
    
    Ok(())
}

async fn get_users(collection: Collection<Document>, filter: Filter) -> Result<Vec<Document>, Error> {
    let cursor = match filter {
        Filter::ScoreAsc => collection.find(doc! {}).sort(doc! { "counter": 1 }).skip(0).limit(10).await?,
        Filter::ScoreDesc => collection.find(doc! {}).sort(doc! { "counter": -1 }).skip(0).limit(10).await?
    };

    let users: Vec<Document> = cursor.try_collect().await?;

    Ok(users)
}

async fn author_place(ctx: &Context<'_>) -> Result<(i64, usize), Error> {
    let db = MONGO_DB.get().unwrap();
    let collection: Collection<Document> = db.collection("users");
    
    let user = collection.find_one(doc! {"user_id": ctx.author().id.to_string()}).await?;
    if user.is_none() {
        return Ok((-1, 0));
    }
    let user_counter = user.unwrap().get_i64("counter").unwrap();
    let cursor = collection.find(doc! {}).sort(doc! { "counter": -1 }).await?;
    let users: Vec<Document> = cursor.try_collect().await?;
    let user_position = users.iter().position(|x| x.get_i64("counter").unwrap() == user_counter).unwrap();

    Ok((user_position as i64 + 1, users.len()))
}

async fn make_embed(ctx: Context<'_>, users: Vec<Document>, filter: Filter) -> CreateEmbed {
    
    // This is to be rearranged later
    let _filter: String = String::from(match filter {
        Filter::ScoreAsc => "",
        Filter::ScoreDesc => ""
    });
    
    let footer = match author_place(&ctx).await {
        Ok((place, _)) => {
            if place == -1 {
                "You are not on the leaderboard.".to_string()
            } else {
                format!("You are #{} on the leaderboard.", place)
            }
        },
        Err(_) => "You are not on the leaderboard.".to_string()
    };
    
    let footer = CreateEmbedFooter::new(footer);

    // Number 1 spot avatar url
    let thumbnail = match users[0].get_str("avatar_url") {
        Ok(url) => url.to_string(),
        Err(_) => ctx.author().default_avatar_url()
    };
    
    let mut embed = CreateEmbed::new()
        .title("__Leaderboard__")
        .color(0x5754d0)
        .footer(footer)
        .thumbnail(thumbnail);
    
    let mut users_str: String = String::new();
    
    // This part should be rewritten later, it's a bit messy
    for (i, user) in users.iter().enumerate() {
        let username = user.get_str("username").unwrap();
        let counter = user.get_i64("counter").unwrap();
        
        let medal = match i {
            0 => "🥇 ",
            1 => "🥈 ",
            2 => "🥉 ",
            _ => ""
        };
        
        users_str.push_str(format!("{}{}: **{}**", medal, username, counter).as_str());
        
        if i != users.len() - 1 {
            users_str.push('\n');
        }
    };

    embed = embed.description(users_str);

    embed
}