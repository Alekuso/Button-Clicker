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

use serenity::all::Interaction;
use serenity::prelude::*;
use tracing::info;
use crate::Handler;

impl Handler {

    pub async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        let instant = std::time::Instant::now();
        let command_name;
        
        if let Interaction::Command(command) = interaction {
            command_name = command.clone().data.name;
            
        } else {
            command_name = "Unknown".to_string();
        }
        
        if command_name != "Unknown" {
            info!("Event interaction_create ({}) | Time: {:?}", command_name, instant.elapsed());
        }
    }
}