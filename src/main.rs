use env_logger;
use serde::Deserialize;
use serenity::all::{
    CreateAttachment, CreateEmbed, CreateEmbedFooter, CreateMessage, Ready, Timestamp,
};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::fs;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }

        if msg.content == "!prueba" {
            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");

                    return;
                }
            };

            // The message builder allows for creating a message by mentioning users dynamically,
            // pushing "safe" versions of content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the 'prueba' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }

        if msg.content == "!hello" {
            // The create message builder allows you to easily create embeds and messages using a
            // builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, an image, three fields, and a footer.
            let footer = CreateEmbedFooter::new("This is a footer");
            let embed = CreateEmbed::new()
                .title("This is a title")
                .description("This is a description")
                .image("attachment://ferris_eyes.png")
                .fields(vec![
                    ("This is the first field", "This is a field body", true),
                    ("This is the second field", "Both fields are inline", true),
                ])
                .field(
                    "This is the third field",
                    "This is not an inline field",
                    false,
                )
                .footer(footer)
                // Add a timestamp for the current time
                // This also accepts a rfc3339 Timestamp
                .timestamp(Timestamp::now());
            let builder = CreateMessage::new()
                .content("Hello, World!")
                .embed(embed)
                .add_file(CreateAttachment::path("./ferris_eyes.png").await.unwrap());
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }

        if msg.content == "!help" {
            let builder = CreateMessage::new().content("HELP");
            let dm = msg.author.dm(&ctx.http, builder).await;

            if let Err(why) = dm {
                println!("Error when direct message user {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[derive(Deserialize)]
struct Config {
    api_token: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // Leer el archivo de configuración
    let config_data =
        fs::read_to_string("config.toml").expect("No se pudo leer el archivo de configuración");

    // Parsear el archivo TOML
    let config: Config =
        toml::from_str(&config_data).expect("No se pudo parsear el archivo de configuración");
    // Login with a bot token from the environment
    let token = config.api_token;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
