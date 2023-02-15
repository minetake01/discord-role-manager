use poise::serenity_prelude::Error;

type Context<'a> = poise::Context<'a, (), Error>;

/// Display help
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[description_localized("ja", "ヘルプを表示するコマンド")]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    Ok(())
}