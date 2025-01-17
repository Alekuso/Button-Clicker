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

use crate::Handler;
use serenity::all::{Context, EventHandler, Guild, Interaction, Ready, UnavailableGuild};
use serenity::async_trait;

mod guild_event;
mod interaction;
mod ready;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        self.guild_create(ctx, guild, is_new).await;
    }

    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {
        self.guild_delete(ctx, incomplete, full).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        self.ready(ctx, ready).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        self.interaction_create(ctx, interaction).await;
    }
}
