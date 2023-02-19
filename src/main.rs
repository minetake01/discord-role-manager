mod commands;
mod structs;

use std::{env, sync::Mutex, ops::Deref};

use poise::serenity_prelude::{Result, GatewayIntents, GuildId, RoleId};
use structs::{GuildMap, RoleMap};
use tokio::{fs, io::{AsyncWriteExt, BufWriter}};

pub struct Data {
    guild_map: Mutex<GuildMap>
}

impl Data {
    pub fn get_role_map(&self, guild_id: &GuildId) -> Option<RoleMap> {
        self.guild_map.lock().unwrap().get(guild_id).cloned()
    }

    pub fn get_parent_role_ids(&self, guild_id: &GuildId, roles: &Vec<RoleId>) -> Vec<RoleId> {
        let Some(role_map) = self.get_role_map(guild_id) else { return vec![] };
        let mut role_ids: Vec<_> = roles.iter()
            .flat_map(|role_id| {
                let edges = role_map.get(role_id).and_then(|role_attrs| Some(role_attrs.edges.clone())).unwrap_or_default();
                if edges.children.is_empty() {
                    return self.get_parent_role_ids_recursive(&role_map, role_id)
                }
                vec![]
            })
            .collect();
        
        role_ids.sort();
        role_ids.dedup();
        role_ids
    }

    fn get_parent_role_ids_recursive(&self, role_map: &RoleMap, role_id: &RoleId) -> Vec<RoleId> {
        let edges = role_map.get(role_id).and_then(|role_attrs| Some(role_attrs.edges.clone())).unwrap_or_default();
        edges.parent.iter()
            .flat_map(|parent_role_id| {
                self.get_parent_role_ids_recursive(role_map, parent_role_id)
            })
            .chain(vec![role_id.clone()])
            .collect()
    }

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
        event_handler: |ctx, event, _framework, data| {
            Box::pin(async move {
                match event {
                    poise::Event::GuildMemberUpdate { old_if_available, new } => {
                        if let Some(old) = old_if_available {
                            if new.roles == old.roles { return Ok(()) }
                        }

                        let role_ids = data.get_parent_role_ids(&new.guild_id, &new.roles);

                        let add_role_ids: Vec<_> = role_ids.iter().filter(|&x| !new.roles.contains(x)).cloned().collect();
                        let remove_role_ids: Vec<_> = new.roles.iter().filter(|&x| !role_ids.contains(x)).cloned().collect();

                        let member = &mut new.clone();

                        if !add_role_ids.is_empty() { member.add_roles(&ctx.http, &add_role_ids).await?; }
                        if !remove_role_ids.is_empty() { member.remove_roles(&ctx.http, &remove_role_ids).await?; }
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
