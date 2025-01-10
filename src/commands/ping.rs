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

use serenity::builder::CreateEmbed;
use crate::commands::{Context, Error};

/// Measure the latency of the bot.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error>  {
    let embed = CreateEmbed::new();


    let builder = poise::reply::CreateReply::default()
                .embed(embed);

    let now = chrono::Utc::now().timestamp_millis();
    let msg = ctx.send(builder).await?;

    let timestamp = msg.clone().into_message().await?.timestamp.timestamp_millis();

    // Shard's ping is the best way to measure Discord's latency.
    // However, its value is 0 when the client has just started.
    // We can avoid this by calculating the latency while sending a message instead of the shard
    // if the actual ping hasn't been calculated yet.
    let latency = if ctx.ping().await.as_millis() == 0 {
        // timestamp will always be higher than now because it'll be got after the message is sent.
        // so we subtract now from timestamp to get the latency while sending a message.
        // It's a good enough way to measure Discord's latency.
        (timestamp - now) as u128
    } else {
        ctx.ping().await.as_millis()
    };
    
    let embed = CreateEmbed::new()
        .title("Pong!")
        .description(format!("Latency: {}ms", latency))
        .color(0x5754d0);

    let builder = poise::reply::CreateReply::default().embed(embed);

    msg.edit(ctx, builder).await?;

    Ok(())
}