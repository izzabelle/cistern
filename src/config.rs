pub struct Config {
    pub dev_token: String,
    pub prod_token: String,
    pub command_prefix: String,
    pub bot_log_id: u64,
    pub guild_id: u64,
    pub admin_id: u64,
    pub bot_id: u64,
}

impl Config {
    pub fn load() -> Self {
        let (mut dev_token, mut prod_token, mut command_prefix): (String, String, String) =
            Default::default();
        let (mut admin_id, bot_id, mut guild_id, mut bot_log_id): (u64, u64, u64, u64) =
            (0, 0, 0, 0);

        dotenv::dotenv().ok();

        for (key, value) in std::env::vars() {
            match key.as_str() {
                "PROD_TOKEN" => prod_token = value,
                "DEV_TOKEN" => dev_token = value,
                "COMMAND_PREFIX" => command_prefix = value,
                "BOT_LOG_ID" => {
                    match value.parse::<u64>() {
                        Ok(id) => bot_log_id = id,
                        Err(e) => panic!("An error occurred: {:?}", e),
                    };
                }
                "GUILD_ID" => {
                    match value.parse::<u64>() {
                        Ok(id) => guild_id = id,
                        Err(e) => panic!("An error occurred: {:?}", e),
                    };
                }
                "ADMIN_ID" => {
                    match value.parse::<u64>() {
                        Ok(id) => admin_id = id,
                        Err(e) => panic!("An error occurred: {:?}", e),
                    };
                }
                _ => {}
            }
        }

        Config {
            dev_token,
            prod_token,
            command_prefix,
            bot_log_id,
            admin_id,
            guild_id,
            bot_id,
        }
    }
}
