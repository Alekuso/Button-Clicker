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

use serenity::all::{ActivityData, OnlineStatus};
use serenity::prelude::*;
use tracing::{info, warn};

macro_rules! scan {
    ($x:ty) => {{
        use std::io;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().parse::<$x>().expect("Invalid input")
    }};
}

// This is still under WIP. It's very far from being the final wanted version
// it works okay for now
pub async fn live_cli(ctx: &Context) {
    let command = scan!(String);
    let command = command.split(" ").collect::<Vec<_>>();

    match command[0] {
        "status:" => {
            let status = match command[1] {
                "online" => OnlineStatus::Online,
                "idle" => OnlineStatus::Idle,
                "dnd" => OnlineStatus::DoNotDisturb,
                "invisible" => OnlineStatus::Invisible,
                _ => {
                    warn!("Invalid status.");
                    return;
                }
            };
            ctx.set_presence(None, status);
        }

        "activity:" => {
            let activity = command[1..].join(" ");
            ctx.set_activity(Some(ActivityData::playing(activity)));
        }

        _ => {
            warn!("Unknown command.");
        }
    }

    info!("Command executed.");
}
