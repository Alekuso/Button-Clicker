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

/// Help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Show help about a specific command"]
    command: Option<String>
) -> Result<(), Error> {

    let config = poise::builtins::HelpConfiguration::default();

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}