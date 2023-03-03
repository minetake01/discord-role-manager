use std::collections::HashMap;

use poise::serenity_prelude::{RoleId, GuildId};
use serde::{Deserialize, Serialize};

pub type GuildMap = HashMap<GuildId, RoleMap>;

pub type RoleMap = HashMap<RoleId, RoleAttrs>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAttrs {
    pub flexible: bool,
    pub edges: RoleEdges,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RoleEdges {
    pub parent: Vec<RoleId>,
    pub children: Vec<RoleId>,
}