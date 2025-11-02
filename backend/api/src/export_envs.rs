use once_cell::sync::Lazy;

pub struct Envs {
    pub database_url: String,
}


pub static ENVS: Lazy<Envs> = Lazy::new(||{
    dotenvy::dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL not found in the environment");
    Envs { database_url }
});
