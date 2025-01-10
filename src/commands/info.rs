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
use readable::byte::*;
use readable::up::*;
use serenity::all::CreateEmbedFooter;
use serenity::builder::CreateEmbed;
use sysinfo::System;
use crate::commands::{Context, Error};

use crate::{MONGO_DB, UPTIME};

/// Get information about the bot
#[poise::command(slash_command)]
pub async fn info(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let db = MONGO_DB.get().unwrap();

    let db_stats = db.run_command(doc! {"dbStats": 1}).await?;
    let storage_size = db_stats.get_i64("storageSize").unwrap(); // in bytes

    let readable_storage_size = Byte::from(storage_size);

    let uptime = UptimeFull::from(UPTIME.get().unwrap());

    let server_count = ctx.cache().guilds().len();
    let total_users = db.collection::<Collection<Document>>("users").count_documents(doc! {}).await?;
    let total_sessions = db.collection::<Collection<Document>>("session").count_documents(doc! {}).await?;
    let memory_usage = Byte::from(get_memory_usage());
    
    let thumbnail = ctx.cache().current_user().avatar_url().unwrap_or_default();
    let footer = CreateEmbedFooter::new("Made with ❤️ by Alex");
    let embed = CreateEmbed::new()
        .title("__Bot Info__")
        .description(format!("**📡 Server count**: {}\n**🔌 Registered users**: {}\n**💡 Current active sessions**: {}\n**🗃️ RAM Usage**: {}\n**📝 DB Storage Usage**: {}\n**🕐 Uptime**: {}", server_count, total_users, total_sessions, memory_usage, readable_storage_size, uptime))
        .thumbnail(thumbnail)
        .color(0x5754d0)
        .footer(footer);


    let builder = poise::reply::CreateReply::default()
        .embed(embed);

    ctx.send(builder).await?;

    Ok(())
}

fn get_memory_usage() -> u64 {
    let mut system = System::new_all();

    system.refresh_memory();

    let pid = std::process::id() as usize;

    let process = system.process(pid.into()).unwrap();
    
    process.memory()
}