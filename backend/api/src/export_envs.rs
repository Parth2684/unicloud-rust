use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Envs {
    pub database_url: String,
}
pub fn get_envs() -> Result<Envs, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let database_url = dotenvy::var("DATABASE_URL")?;
    Ok(Envs { database_url })
}
