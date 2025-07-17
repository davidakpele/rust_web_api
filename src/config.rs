use dotenvy::dotenv;  

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok(); 
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        Config { database_url }
    }
}
