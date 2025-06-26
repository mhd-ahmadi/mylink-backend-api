use sqlx::{Pool, Postgres};
use crate::base::configuration::Configuration;


#[derive(Clone,Debug)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub configuration: Configuration,
}
