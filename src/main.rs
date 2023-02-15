mod commands;

use std::env;

use poise::serenity_prelude::*;

#[tokio::main]
async fn main(){
    dotenv::dotenv().unwrap();

    let token = if cfg!(debug_assertions) { env::var("DEBUG_DISCORD_TOKEN").unwrap() } else { env::var("DISCORD_TOKEN").unwrap() };

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MEMBERS;

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::help::help(),
            commands::group::group()
        ],
        event_handler: |ctx, event, _framework, _data| {
            Box::pin(async move {
                match event {
                    poise::Event::GuildMemberUpdate { old_if_available, new } => {
                        Ok(())
                    },
                    _ => {
                        println!("{}", event.name());
                        Ok(())
                    }
                }
            })
        },
        on_error: |err| {
            Box::pin(async {
                match err {
                    poise::FrameworkError::Command { ctx, .. } => {
                        println!(
                            "In on_error: {:?}",
                            ctx.invocation_data::<&str>().await.as_deref()
                        );
                    }
                    err => poise::builtins::on_error(err).await.unwrap(),
                }
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(token)
        .intents(intents)
        .options(options)
        .setup(|ctx, ready, _framework| Box::pin(async move {
            println!("{} is connected! Shard ID: {}", ready.user.name, ctx.shard_id);

            poise::builtins::register_globally(&ctx.http, &[
                commands::help::help(),
                commands::role::role(),
                commands::group::group(),
            ]).await?;
            Ok(())
        }));

    if let Err(err) = framework.run_autosharded().await {
        println!("An error occurred while running the client: {:?}", err)
    }
}
