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

const TEXT: &str = r#"*Button Clicker* bot is a simple bot about gaining a score by pressing a button.
There's not much to it currently but some features will come as a shop to exchange a "score" to a point multiplier, etc...
Feel free to check up the dev's social if there's an issue with the bot!"#;

/// Links to the bot's dev
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let thumbnail = ctx.cache().current_user().avatar_url().unwrap_or_default();
    let embed = CreateEmbed::new()
        .title("__About__")
        .description(TEXT)
        .field(
            "__Discord__",
            "alexou *(will not accept friend requests)*",
            true,
        )
        .field("__Twitter__", "[@Alekuso_](https://x.com/Alekuso_)", true)
        .field("__Github__", "[Alekuso](https://github.com/Alekuso)", false)
        .field(
            "__Bluesky__",
            "[alex.tanukii.dev](https://bsky.app/profile/alex.tanukii.dev)",
            true,
        )
        .field("__Website__", "[tanukii.dev](https://tanukii.dev)", true)
        .field(
            "__Alex's community Server__",
            "[Alex's Den](https://discord.gg/HWQXZJGCAM)",
            false,
        )
        .field(
            "__Bot Invite__",
            "**[Invite me !](https://discord.com/oauth2/authorize?client_id=774018602549772289)**",
            false,
        )
        .field(
            "__Github Repository__",
            "[Github Repository](https://github.com/Alekuso/Button-Clicker-Bot)",
            false,
        )
        .thumbnail(thumbnail)
        .color(0x5754d0);

    let builder = poise::reply::CreateReply::default().embed(embed);

    ctx.send(builder).await?;

    Ok(())
}
