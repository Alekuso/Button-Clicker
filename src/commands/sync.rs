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

use crate::commands::play::create_user;
use crate::commands::{Context, Error};
use mongodb::Collection;
use mongodb::bson::{Document, doc};

/// Fix your account if it appears "broken"
#[poise::command(slash_command)]
pub async fn sync(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;
    let collection: Collection<Document> = db.collection("users");

    // Checks if the user has an account
    // It creates a new account if the user doesn't have one
    let user = collection
        .find_one(doc! {"user_id": ctx.author().id.to_string()})
        .await?;

    if user.is_none() {
        create_user(ctx, collection.clone()).await?;
    }

    // Force update all infos
    collection
        .update_one(
            doc! {
                "user_id": ctx.author().id.to_string()
            },
            doc! {
                "$set": {
                    "username": ctx.author().name.clone(),
                    "user_id": ctx.author().id.to_string(),
                    "avatar_url": ctx.author().avatar_url().unwrap_or_default()
                }
            },
        )
        .await?;

    let builder = poise::reply::CreateReply::default()
        .content("Your account has been synchronized and fixed!");

    ctx.send(builder).await?;

    Ok(())
}
