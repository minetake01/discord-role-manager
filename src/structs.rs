use std::collections::HashMap;

use poise::serenity_prelude::RoleId;
use serde::{Deserialize, Serialize};

use crate::serializer::{WrappedPermissions, WrappedGuildId, WrappedRoleId};

pub type RolesData = HashMap<WrappedGuildId, HashMap<WrappedRoleId, RoleData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleData {
    pub flexible: bool,
    pub permissions: WrappedPermissions,
    pub parent: Vec<RoleId>,
}