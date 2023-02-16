use poise::serenity_prelude::{Permissions, GuildId, RoleId};
use serde::{Serialize, Deserialize, de::Visitor};

//GuildIdのDeserialize実装はU64Visitorで文字列から直接デシリアライズできないため、VisitorでDeserializeを独自実装する。
#[derive(Hash, PartialEq, Eq, Debug, Serialize)]
pub struct WrappedGuildId (u64);

struct GuildIdVisitor;

impl<'de> Visitor<'de> for GuildIdVisitor {
    type Value = WrappedGuildId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(WrappedGuildId(v.parse().unwrap()))
    }
}

impl<'de> Deserialize<'de> for WrappedGuildId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_identifier(GuildIdVisitor)
    }
}

impl From<GuildId> for WrappedGuildId {
    fn from(value: GuildId) -> Self {
        Self ( value.0 )
    }
}


//GuildIdと同じく、RoleIdのDeserialize実装はU64Visitorで文字列から直接デシリアライズできないため、VisitorでDeserializeを独自実装する。
#[derive(Hash, PartialEq, Eq, Debug, Serialize)]
pub struct WrappedRoleId (u64);

struct RoleIdVisitor;

impl<'de> Visitor<'de> for RoleIdVisitor {
    type Value = WrappedRoleId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(WrappedRoleId(v.parse().unwrap()))
    }
}

impl<'de> Deserialize<'de> for WrappedRoleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_identifier(RoleIdVisitor)
    }
}

impl From<RoleId> for WrappedRoleId {
    fn from(value: RoleId) -> Self {
        Self ( value.0 )
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WrappedPermissions (u64);

impl From<Permissions> for WrappedPermissions {
    fn from(value: Permissions) -> Self {
        Self ( value.bits() )
    }
}