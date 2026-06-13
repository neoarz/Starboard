use dotenv::dotenv;
use std::env;
use twilight_model::id::{Id, marker::WebhookMarker};
use url::Url;

pub struct ErrorWebhook {
    pub id: Id<WebhookMarker>,
    pub token: String,
}

pub struct Config {
    pub token: String,
    pub shards: u32,
    pub db_url: String,
    pub db_connections: u32,
    pub error_webhook: Option<ErrorWebhook>,
    pub development: bool,
    pub owner_ids: Vec<u64>,
    pub bot_id: u64,
}

impl Config {
    fn get_optional_env(name: &str) -> Option<String> {
        env::var(name).ok().filter(|value| !value.trim().is_empty())
    }

    fn get_required_env(name: &str) -> String {
        Self::get_optional_env(name).unwrap_or_else(|| panic!("{name} not set"))
    }

    fn database_url_from_env() -> String {
        if let Some(url) = Self::get_optional_env("SB_DATABASE_URL") {
            return url;
        }

        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "db".to_string());
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let database = Self::get_required_env("POSTGRES_DB");
        let username = Self::get_required_env("POSTGRES_USER");
        let password = Self::get_required_env("POSTGRES_PASSWORD");

        let mut url = Url::parse(&format!("postgresql://{host}:{port}/{database}"))
            .expect("Invalid PostgreSQL host, port, or database name");
        url.set_username(&username)
            .expect("Invalid PostgreSQL username");
        url.set_password(Some(&password))
            .expect("Invalid PostgreSQL password");

        url.into()
    }

    fn error_webhook_from_env() -> Option<ErrorWebhook> {
        let webhook_url = Self::get_optional_env("ERROR_WEBHOOK_URL")?;
        let url = Url::parse(&webhook_url).expect("Invalid ERROR_WEBHOOK_URL");
        let mut segments = url
            .path_segments()
            .expect("ERROR_WEBHOOK_URL must include a webhook path");

        let is_webhook_path =
            matches!(segments.next(), Some("api")) && matches!(segments.next(), Some("webhooks"));
        if !is_webhook_path {
            panic!("ERROR_WEBHOOK_URL must be a Discord webhook URL");
        }

        let id = segments
            .next()
            .expect("ERROR_WEBHOOK_URL missing webhook ID")
            .parse()
            .expect("Invalid webhook ID in ERROR_WEBHOOK_URL");
        let token = segments
            .next()
            .expect("ERROR_WEBHOOK_URL missing webhook token")
            .to_string();

        Some(ErrorWebhook {
            id: Id::new(id),
            token,
        })
    }

    pub fn from_env() -> Self {
        match dotenv() {
            Ok(_) => {}
            Err(why) => eprintln!("Failed to load .env: {why}"),
        };
        let token = Self::get_required_env("DISCORD_TOKEN");
        let db_url = Self::database_url_from_env();
        let error_webhook = Self::error_webhook_from_env();
        let development = env::var("DEVELOPMENT")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .expect("Invalid boolean for DEVELOPMENT.");
        let owner_ids = Self::get_optional_env("OWNER_IDS").map(|var| {
            var.split(',')
                .map(|item| item.trim().parse().expect("invalid owner id"))
                .collect()
        });
        let bot_id = Self::get_required_env("BOT_ID")
            .parse()
            .expect("Invalid BOT_ID");

        Config {
            token,
            shards: 1,
            db_url,
            db_connections: 10,
            error_webhook,
            development,
            owner_ids: owner_ids.unwrap_or_default(),
            bot_id,
        }
    }
}
