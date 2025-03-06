use env_logger;
use serde::Deserialize;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage, Ready, Timestamp};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::fs;
use ureq;

struct Handler;

#[derive(Deserialize)]
struct WeatherResponse {
    name: String,
    main: WeatherMain,
    weather: Vec<WeatherDetails>,
}

#[derive(Deserialize)]
struct WeatherMain {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    humidity: u8,
}

#[derive(Deserialize)]
struct WeatherDetails {
    description: String,
}

impl WeatherResponse {
    fn fetch(city: &str, api_key: &str) -> Result<Self, ureq::Error> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=es",
            city, api_key
        );
        let response: WeatherResponse = ureq::get(&url).call()?.into_json()?;
        Ok(response)
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let config_data =
            fs::read_to_string("config.toml").expect("No se pudo leer el archivo de configuración");
        let config: Config =
            toml::from_str(&config_data).expect("No se pudo parsear el archivo de configuración");
        let api_key = &config.api.api_weather;

        if msg.content.starts_with("!clima") {
            let city = msg.content.split_whitespace().nth(1).unwrap_or("Madrid");
            match WeatherResponse::fetch(city, api_key) {
                Ok(weather) => {
                    let embed = CreateEmbed::new()
                        .title(format!("☁️ Clima en {}", weather.name))
                        .field("🌡 Temperatura", format!("{}°C", weather.main.temp), true)
                        .field("📉 Mínima", format!("{}°C", weather.main.temp_min), true)
                        .field("📈 Máxima", format!("{}°C", weather.main.temp_max), true)
                        .field("💧 Humedad", format!("{}%", weather.main.humidity), true)
                        .field("🌤 Estado", weather.weather[0].description.clone(), false)
                        .footer(CreateEmbedFooter::new("Datos de OpenWeather"))
                        .timestamp(Timestamp::now());
                    let response = CreateMessage::new().embed(embed);
                    let _ = msg.channel_id.send_message(&ctx.http, response).await;
                }
                Err(_) => {
                    let _ = msg
                        .channel_id
                        .say(&ctx.http, "⚠️ No se pudo obtener el clima.")
                        .await;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[derive(Deserialize)]
struct Config {
    api: ApiKeys,
}

#[derive(Deserialize)]
struct ApiKeys {
    api_token: String,
    api_weather: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let config_data =
        fs::read_to_string("config.toml").expect("No se pudo leer el archivo de configuración");
    let config: Config =
        toml::from_str(&config_data).expect("No se pudo parsear el archivo de configuración");

    let token = &config.api.api_token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
