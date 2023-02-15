use poise::serenity_prelude::{Error, Role};

type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(
    slash_command,
    hide_in_help,
    subcommands("list", "members")
)]
pub async fn role(_: Context<'_>) -> Result<(), Error> { Ok(()) }

/// Display a list of roles.
#[poise::command(
    slash_command,
    description_localized("ja", "ロールのリストを表示"),
)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Parent role to display the list"]
    #[description_localized("ja", "リストを表示する親ロール")]
    parent_role: Option<Role>,
) -> Result<(), Error> {
    Ok(())
}

/// Displays a list of members who have been granted specific roles.
#[poise::command(
    slash_command,
    description_localized("ja", "特定のロールが付与されているメンバーのリストを表示"),
)]
pub async fn members(
    ctx: Context<'_>,
    #[description = "Roles to display"]
    #[description_localized("ja", "ロールを表示")]
    role: Role,
) -> Result<(), Error> {
    Ok(())
}