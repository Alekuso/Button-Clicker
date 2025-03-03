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

use crate::{CONFIG, Handler};
use serenity::all::{
    ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage, Guild, UnavailableGuild,
};
use serenity::prelude::*;
use tracing::{debug, info};

impl Handler {
    pub async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        let instant = std::time::Instant::now();

        if is_new.is_some() && CONFIG.get().unwrap().log_channel_id.is_some() {
            if !is_new.unwrap() {
                return;
            }

            info!(
                "Guild {} joined || total {}",
                guild.name,
                ctx.cache.guilds().len()
            );
            // A guild can have no icon, so we need to check if it exists.
            // If it doesn't exist, we use 0.png which is Discord's default icon.
            let thumbnail_link = guild
                .icon_url()
                .unwrap_or_else(|| String::from("https://cdn.discordapp.com/embed/avatars/0.png"));

            let embed = CreateEmbed::new()
                .description("**Joined a new guild**")
                .fields(vec![
                    ("__Name__", guild.name, true),
                    ("__ID__", guild.id.to_string(), true),
                    ("__Members__", guild.member_count.to_string(), true),
                    (
                        "__Owner__",
                        guild.owner_id.to_user(&ctx).await.unwrap().tag(),
                        true,
                    ),
                ])
                .thumbnail(thumbnail_link)
                .footer(CreateEmbedFooter::new(format!(
                    "Now in {} guilds",
                    ctx.cache.guilds().len()
                )));

            let builder = CreateMessage::new().embed(embed);

            let channel = ChannelId::from(CONFIG.get().unwrap().log_channel_id.unwrap());
            let message = channel.send_message(&ctx.http, builder).await;
            debug!("Sent message: {:?}", message);
        }

        info!("Event guild_create | Time: {:?}", instant.elapsed());
    }

    pub async fn guild_delete(
        &self,
        ctx: Context,
        incomplete: UnavailableGuild,
        full: Option<Guild>,
    ) {
        let instant = std::time::Instant::now();

        // Discord can sometimes mess up and "crash" a guild.
        // This still send a guild_delete event.
        // Which technically isn't a bot getting kicked from a guild.
        if incomplete.unavailable {
            return;
        }

        if let Some(guild) = full {
            // If the log channel is not set, we don't want to send a message.
            if CONFIG.get().unwrap().log_channel_id.is_none() {
                return;
            }

            info!(
                "Guild {} left || total {}",
                guild.name,
                ctx.cache.guilds().len()
            );

            // A guild can have no icon, so we need to check if it exists.
            // If it doesn't exist, we use 0.png which is Discord's default icon.
            let thumbnail_link = guild
                .icon_url()
                .unwrap_or_else(|| String::from("https://cdn.discordapp.com/embed/avatars/0.png"));

            let embed = CreateEmbed::new()
                .description("**Left a guild**")
                .fields(vec![
                    ("__Name__", guild.name, true),
                    ("__ID__", guild.id.to_string(), true),
                    ("__Members__", guild.member_count.to_string(), true),
                    (
                        "__Owner__",
                        guild.owner_id.to_user(&ctx).await.unwrap().name,
                        true,
                    ),
                ])
                .thumbnail(thumbnail_link)
                .footer(CreateEmbedFooter::new(format!(
                    "Now in {} guilds",
                    ctx.cache.guilds().len()
                )));

            let builder = CreateMessage::new().embed(embed);

            let channel = ChannelId::from(CONFIG.get().unwrap().log_channel_id.unwrap());
            let message = channel.send_message(&ctx.http, builder).await;
            debug!("Sent message: {:?}", message);
        }

        info!("Event guild_delete | Time: {:?}", instant.elapsed());
    }
}
