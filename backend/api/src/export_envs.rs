use std::error::Error;
use std::fmt::Debug;


#[derive(Debug)]
pub struct Envs {
    pub database_url: String
}
pub fn get_envs() -> Result<Envs, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_string = dotenvy::var("DATABASE_URL");
    match db_string {
        Result::Ok(str) => Ok(Envs { database_url: str }),
        Result::Err(err) => Err(Box::new(err))
    }
}
