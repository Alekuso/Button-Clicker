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

use crate::cli::*;
use crate::Handler;
use serenity::all::{ActivityData, OnlineStatus, Ready};
use serenity::prelude::*;
use tracing::info;

impl Handler {
    pub async fn ready(&self, ctx: Context, ready: Ready) {
        let instant = std::time::Instant::now();

        if let Some(shard) = ready.shard {
            info!(
                "Connected on shard {}/{} on {} server{}",
                shard.id.0 + 1,
                shard.total,
                ready.guilds.len(),
                if ready.guilds.len() == 1 { "" } else { "s" }
            );

            #[cfg(debug_assertions)]
            ctx.set_presence(
                Some(ActivityData::playing("DEBUG BUILD")),
                OnlineStatus::DoNotDisturb,
            );

            #[cfg(not(debug_assertions))]
            ctx.set_presence(
                Some(ActivityData::playing("/help to get started")),
                OnlineStatus::Online,
            );

            if shard.id.0 == 0 {
                tokio::task::spawn(async move {
                    loop {
                        live_cli(&ctx).await;
                    }
                });
            }
        }

        info!("Event ready | Time: {:?}", instant.elapsed());
    }
}
