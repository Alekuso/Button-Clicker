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

use mongodb::bson::{doc, Document};
use mongodb::Collection;
use serenity::builder::CreateEmbed;
use crate::commands::{Context, Error};

use crate::MONGO_DB;

/// View the profile of yourself or a user
#[poise::command(slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "(Optional) The username to view the profile of"]
    username: Option<String>
) -> Result<(), Error> {
    let db = MONGO_DB.get().unwrap();
    let collection: Collection<Document> = db.collection("users");

    let search_self: bool;
    let username = match username {
        Some(username) => {
            search_self = false;
            username
        
        },
        None => {
            search_self = true;
            ctx.author().name.clone()
        }
    };
    
    // If the user isn't using a username, then it'll search itself with the author's ID
    // This can avoid some bug like searching for a user with a username that doesn't exist
    // since they might have changed username right before and the bot hasn't updated yet.
    // This avoids their account to look "broken" too!
    let user = 
        if search_self {
            collection.find_one(doc! {
                "user_id": ctx.author().id.to_string()
            }).await?
        } else {
            collection.find_one(doc! {
                "username": username
            }).await?
        };
    
    
    if user.is_none() {
        return Err("User not found".into());
    }
    
    let user = user.unwrap();

    let embed = make_embed(ctx, user);

    let builder = poise::reply::CreateReply::default()
        .embed(embed);

    ctx.send(builder).await?;

    Ok(())
}

fn make_embed(ctx: Context<'_>, user: Document) -> CreateEmbed {
    
    // As for now, there will only be a thumbnail if the user views their own profile.
    let search_user_name = user.get_str("username").unwrap();
    let thumbnail = if search_user_name == ctx.author().name {
        ctx.author().avatar_url().unwrap_or_default()
    } else {
        user.get_str("avatar_url").unwrap().to_string()
    };

    CreateEmbed::new()
        .title(format!("__{}'s Profile__", search_user_name))
        .description(format!("Current score: **{}**", user.get_i64("counter").unwrap()))
        .color(0x5754d0)
        .thumbnail(thumbnail)
}