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

pub mod about;
pub mod help;
pub mod info;
pub mod leaderboard;
pub mod ping;
pub mod play;
pub mod profile;
pub mod sync;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub db: mongodb::Database,
    pub uptime: std::time::Instant,
}

/*
    The database schema is as follows:
        user_id: String,
        username: String,
        avatar_url: String,
        counter: i64

    user_id uses a string because Discord stores its user IDs as a snowflake, which is a 64-bit integer.
    However, that integer is unsigned, which mongodb doesn't support.
    In addition, making it a String will make it sure that there won't be any problem.

    username is used to search for a user when doing a /profile command, it's also used for the leaderboard.
    there might be an issue if a user changes their username, and doesn't update their score, since it won't be synced in the database anymore.
    This can cause issues with the /profile not working as expected,
    but that's a minor issue which might be fixed in the future. (TODO)

    counter uses an i64 because its limit are big enough to not have to worry about it.
    In the old versions of Button Clicker (v1 & v2), the counter was a string, which allowed to store theoretically an infinite number.
    But it was recently found that after 5 years, it didn't really matter because the counter hardly exceeded multiple thousands.
    An i32 was ignored because it doesn't cost much to use 64 bits instead + it's more future-proof as future version of this bot,
    might have new features like a multiplier that could make the counter exceed the limit of an i32. (probably not, but we never know)
*/
