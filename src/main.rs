mod commands;
mod structs;

use std::{env, sync::Mutex, ops::Deref};

use poise::serenity_prelude::{Result, GatewayIntents};
use structs::GuildMap;
use tokio::{fs, io::{AsyncWriteExt, BufWriter}};

pub struct Data {
    guild_map: Mutex<GuildMap>
}

impl Data {
    pub async fn save(&self) -> Result<()> {
        let guild_map = self.guild_map.lock().unwrap();
        
        let file = fs::File::create("db/roles.toml").await?;
        let mut writer = BufWriter::new(file);
        let content = ron::ser::to_string(&guild_map.deref()).unwrap();
        writer.write_all(content.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main(){
    dotenv::dotenv().unwrap();

    let token = if cfg!(debug_assertions) { env::var("DEBUG_DISCORD_TOKEN").unwrap() } else { env::var("DISCORD_TOKEN").unwrap() };

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MEMBERS;

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::help::help(),
            commands::role::role(),
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
        .setup(|ctx, ready, _framework| {
            println!("{} is connected! Shard ID: {}", ready.user.name, ctx.shard_id);

            Box::pin(async move {
                //Slash Commandを登録
                poise::builtins::register_globally(&ctx.http, &[
                    commands::help::help(),
                    commands::role::role(),
                    commands::group::group(),
                ]).await?;

                let file_content = fs::read_to_string("db/roles.ron").await?;
                let roles_data: GuildMap = ron::from_str(&file_content).expect("RONのパースに失敗しました。");

                Ok(Data {
                    guild_map: Mutex::new(roles_data),
                })
            })
        });

    if let Err(err) = framework.run_autosharded().await {
        println!("An error occurred while running the client: {:?}", err)
    }
}
