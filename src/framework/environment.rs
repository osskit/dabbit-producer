use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
pub struct Environment {
    pub port: u16,
}

pub fn get_environment() -> Result<Environment, envy::Error> {
    dotenv().ok();

    envy::from_env::<Environment>()
}
