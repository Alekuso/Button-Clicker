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
use serenity::builder::CreateEmbed;

/// Measure the latency of the bot.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await.as_millis();
    let embed = CreateEmbed::new()
        .title("Pong!")
        .description(format!("Latency: {}ms", latency))
        .color(0x5754d0);

    let builder = poise::reply::CreateReply::default()
        .embed(embed)
        .ephemeral(true);

    ctx.send(builder).await?;

    Ok(())
}
