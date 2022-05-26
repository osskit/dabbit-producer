use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Environment {
    pub port: u16,
}

pub fn get_environment() -> Result<Environment, envy::Error> {
    envy::from_env::<Environment>()
}
