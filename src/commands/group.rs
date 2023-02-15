use poise::serenity_prelude::{Error, Role};

type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(
    slash_command,
    hide_in_help,
    subcommands("new", "create", "add", "remove", "modify")
)]
pub async fn group(_: Context<'_>) -> Result<(), Error> { Ok(()) }

/// Create a new role group, including child roles.
#[poise::command(
    slash_command,
    description_localized("ja", "子ロールを含め、ロールグループを新規作成する。"),
)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "Name of the role group to create"]
    #[description_localized("ja", "作成するロールグループの名前")]
    name: String,
    #[description = "Whether to create in flexible mode"]
    #[description_localized("ja", "フレキシブルモードで作成するかどうか")]
    flexible: Option<bool>,
) -> Result<(), Error> {
    Ok(())
}

/// Create a role group and include existing roles.
#[poise::command(
    slash_command,
    description_localized("ja", "ロールグループを作成し、既存のロールを含める。"),
)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Name of the role group to create"]
    #[description_localized("ja", "作成するロールグループの名前")]
    name: String,
    #[description = "Whether to create in flexible mode"]
    #[description_localized("ja", "フレキシブルモードで作成するかどうか")]
    flexible: Option<bool>,
) -> Result<(), Error> {
    Ok(())
}

/// Add roles to the role group.
#[poise::command(
    slash_command,
    description_localized("ja", "ロールグループに子ロールを追加する。"),
)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Name of child role to be created"]
    #[description_localized("ja", "作成する子ロールの名前")]
    name: String,
    #[description = "Role group to be added"]
    #[description_localized("ja", "追加するロールグループ")]
    parent_role: Role,
) -> Result<(), Error> {
    Ok(())
}

/// Delete a role from a role group.
#[poise::command(
    slash_command,
    description_localized("ja", "ロールグループからロールを削除する。"),
)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Roles to be deleted"]
    #[description_localized("ja", "削除するロール")]
    child_role: Role,
    #[description = "Role group to delete role"]
    #[description_localized("ja", "ロールを削除するロールグループ")]
    parent_role: Role,
) -> Result<(), Error> {
    Ok(())
}

/// Modify role group settings.
#[poise::command(
    slash_command,
    description_localized("ja", "ロールグループの設定を変更する。"),
)]
pub async fn modify(
    ctx: Context<'_>,
    #[description = "Role group to modify settings"]
    #[description_localized("ja", "設定を変更するロールグループ")]
    parent_role: Role,
    #[description = "Whether to change to flexible mode"]
    #[description_localized("ja", "フレキシブルモードに変更するかどうか")]
    flexible: Option<bool>,
) -> Result<(), Error> {
    Ok(())
}