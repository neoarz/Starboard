use dotenv::dotenv;
use std::env;
use url::Url;

pub struct Config {
    pub token: String,
    pub patreon_token: Option<String>,
    pub sentry: Option<String>,
    pub shards: u32,
    pub db_url: String,
    pub db_connections: u32,
    pub error_channel: Option<u64>,
    pub development: bool,
    pub owner_ids: Vec<u64>,
    pub bot_id: u64,
    pub main_guild: Option<u64>,
    pub patron_role: Option<u64>,
    pub supporter_role: Option<u64>,
    pub proxy: Option<String>,
}

impl Config {
    fn get_optional_env(name: &str) -> Option<String> {
        env::var(name).ok().filter(|value| !value.trim().is_empty())
    }

    fn get_required_env(name: &str) -> String {
        Self::get_optional_env(name).unwrap_or_else(|| panic!("{name} not set"))
    }

    fn get_optional_id(name: &str) -> Option<u64> {
        Self::get_optional_env(name).map(|value| value.parse().expect("Invalid ID"))
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

    pub fn from_env() -> Self {
        match dotenv() {
            Ok(_) => {}
            Err(why) => eprintln!("Failed to load .env: {why}"),
        };
        let token = Self::get_required_env("DISCORD_TOKEN");
        let patreon_token = Self::get_optional_env("PATREON_TOKEN");
        let sentry = Self::get_optional_env("SENTRY_URL");
        let shards = env::var("SHARDS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap();
        let db_url = Self::database_url_from_env();
        let db_connections = Self::get_optional_env("DB_MAX_DB_CONNECTIONS")
            .map(|v| v.parse().unwrap())
            .unwrap_or(10);
        let error_channel = Self::get_optional_id("ERROR_CHANNEL_ID");
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

        let main_guild = Self::get_optional_id("MAIN_GUILD");
        let patron_role = Self::get_optional_id("PATRON_ROLE");
        let supporter_role = Self::get_optional_id("SUPPORTER_ROLE");

        let proxy = Self::get_optional_env("PROXY");

        Config {
            token,
            patreon_token,
            sentry,
            shards,
            db_url,
            db_connections,
            error_channel,
            development,
            owner_ids: owner_ids.unwrap_or_default(),
            bot_id,
            main_guild,
            patron_role,
            supporter_role,
            proxy,
        }
    }
}
