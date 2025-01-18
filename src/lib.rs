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

mod commands;
mod handler;

use crate::commands::Data;
use mongodb::error::Error;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use mongodb::Database;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use serenity::prelude::*;
use std::fs;
use std::time::Instant;
use tokio::sync::OnceCell;
use tracing::{error, info};

pub struct Handler;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    token: String,                   // Bot token
    pub log_channel_id: Option<u64>, // (optional) Channel ID for logging
    mongodb_uri: String,
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn run() -> Result<Client, serenity::Error> {
    info!("Starting Client");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS;

    let _ = CONFIG.get_or_init(|| async { get_config() }).await;

    let token = &CONFIG.get().unwrap().token;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help::help(),
                commands::play::play(),
                commands::profile::profile(),
                commands::leaderboard::leaderboard(),
                commands::ping::ping(),
                commands::sync::sync(),
                commands::info::info(),
                commands::about::about(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Slash commands registered");

                let uri = &CONFIG.get().unwrap().mongodb_uri;
                let mongo_client = create_mongo_client(uri)
                    .await
                    .expect("Failed to connect to MongoDB");

                Ok(Data {
                    db: mongo_client,
                    uptime: Instant::now(),
                })
            })
        })
        .build();

    // Initialize the client
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    client.start_autosharded().await?;

    error!("This part of the code shouldn't be reached. If you see this, something went wrong. and the bot has probably crashed.");
    Ok(client)
}

async fn create_mongo_client(secret: &str) -> Result<Database, Error> {
    let now = Instant::now();
    info!("Connecting to MongoDB");
    let mut client_options = ClientOptions::parse(secret).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = mongodb::Client::with_options(client_options)?;
    let db = client.database("button_clicker");

    info!("Connected to MongoDB | Time: {:?}", now.elapsed());
    Ok(db)
}

fn get_config() -> Config {
    // The RON file is better in this case because it's one of the few serde impl that actually supports
    // the u64 format without having to do parsing and conversion and stuff.
    // + it's "native" to Rust's type of syntax.
    match fs::read_to_string("config.ron") {
        Ok(config) => {
            let config = ron::from_str(&config).unwrap();
            info!("Loaded config from file");
            config
        }
        Err(_) => {
            error!("Config file not found. Creating a default config file.");

            // Create a default config
            let config = Config::default();

            // Write the default config to a file
            let config = ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();

            fs::write("config.ron", config).unwrap();

            // Exit
            std::process::exit(1);
        }
    }
}
