// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::Shared;
use crate::types::{entities::PublicUser, Snowflake};
use crate::types::utils::serde::*;

#[derive(Debug, Deserialize, Default, Serialize, Clone)]
/// Represents a participating user in a guild.
///
/// # Reference
/// See <https://discord-userdoccers.vercel.app/resources/guild#guild-member-object>
pub struct GuildMember {
    pub user: Option<Shared<PublicUser>>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    #[serde(with = "ts_seconds_str")]
    pub joined_at: DateTime<Utc>,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: Option<i32>,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    #[serde(with = "ts_seconds_option_str")]
    pub communication_disabled_until: Option<DateTime<Utc>>,
}
