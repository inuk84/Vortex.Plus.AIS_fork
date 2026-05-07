use serenity::all::ChannelId;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::server::fetch::fetch_user::{
    self, fetch_user_bio, fetch_user_creation, fetch_user_followers, fetch_user_following,
    fetch_user_friends, fetch_user_status, fetch_user_visits,
};

fn get_token() -> &'static str {
    "MTUwMTYxNjUzNzQ3ODYyNzQwOA.GTSVrb.zgRDnPJDq-HiY3f1SN60R47R7QxojjmWRwCN6I"
}

pub async fn send_embed(
    ctx: &Context,
    title: &str,
    desc: &str,
    content: &str,
    msg: &Message,
) -> Result<(), serenity::Error> {
    let embed = CreateEmbed::new()
        .title(title)
        .description(desc)
        .color(0x00ff00);

    let builder = CreateMessage::new().content(content).embed(embed);

    msg.channel_id.send_message(&ctx.http, builder).await?;
    Ok(())
}

async fn send_msg(ctx: &Context, channel_id: u64, content: &str) {
    let channel = ChannelId::new(channel_id);

    let _ = channel.say(&ctx.http, content).await;
}

pub async fn init() {
    use serenity::{
        async_trait,
        model::{channel::Message, gateway::Ready},
        prelude::*,
    };

    struct Handler;

    const LOG_CHANNEL: u64 = 1501636741772349601; // channel in bot database, logging

    #[async_trait]
    impl EventHandler for Handler {
        async fn ready(&self, ctx: Context, ready: Ready) {
            println!("{} is online", ready.user.name);
            //let msg = format!("Client {} Active", fetch_user::fetch_client_name());
            //send_msg(&ctx, LOG_CHANNEL, &msg).await;
        }

        async fn message(&self, ctx: Context, msg: Message) {
            if msg.author.bot {
                return;
            }

            let parts: Vec<&str> = msg.content.split_whitespace().collect();
            let client = reqwest::Client::new();

            if parts.get(0) == Some(&"help") || parts.get(0) == Some(&"commands") {
                let text = format!("
                **Commands**\nwhoisid|OR|fetch <id> : finds user info by ID\n");
                let _ = msg.channel_id.say(&ctx.http, text).await;
            }

            if parts.get(0) == Some(&"whoisid") || parts.get(0) == Some(&"fetch") {
                if let Some(arg) = parts.get(1) {
                    match fetch_user::fetch_user_name(&client, arg).await {
                        Some(username) => {
                            let usr = format!("{}", username);

                            let desc = format!(
                                    "{}\n\n**Info**\n\
                                    id: `{}`\n\
                                    Status: `{}`\n\
                                    Creation Date:\n`{}`\n\n\
                                    **Statistics**\n\
                                    Friends: `{}`\n\
                                    Followers: `{}`\n\
                                    Following: `{}`\n\n\
                                    Visits: `{}`",
                                fetch_user_bio(&client, arg).await.unwrap_or("No bio".to_string()), arg,
                                fetch_user_status(&client, arg).await.unwrap_or("ERROR 404".to_string()),
                                fetch_user_creation(&client, arg).await.unwrap_or("ERROR 404".to_string()),
                                fetch_user_friends(&client, arg).await.unwrap_or(0),
                                fetch_user_followers(&client, arg).await.unwrap_or(0),
                                fetch_user_following(&client, arg).await.unwrap_or(0),
                                fetch_user_visits(&client, arg).await.unwrap_or(0),
                            );

                            let _ = send_embed(&ctx, &usr, &desc, "", &msg).await;
                        }
                        None => {
                            let _ = msg.channel_id.say(&ctx.http, "User not found").await;
                        }
                    }
                }
            }
        }
    }

    let token = get_token();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}
