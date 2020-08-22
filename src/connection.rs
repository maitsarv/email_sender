//! Database-related functions
use crate::config::{CONFIG};
use diesel::{mysql::MysqlConnection, pg::PgConnection, sqlite::SqliteConnection, Connection, ConnectionResult};

#[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseType {
    #[cfg(feature = "mysql")]
    Mysql,
    #[cfg(feature = "postgres")]
    Postgres,
    #[cfg(feature = "sqlite")]
    Sqlite,
}


#[cfg(feature = "mysql")]
pub type ConnectionType = MysqlConnection;

#[cfg(feature = "postgres")]
pub type ConnectionType = PgConnection;

#[cfg(feature = "sqlite")]
pub type ConnectionType = SqliteConnection;

pub fn create_connection() -> ConnectionType {
    match CONFIG.database {
        #[cfg(feature = "mysql")]
        DatabaseType::Mysql => {
            MysqlConnection::establish(&CONFIG.database_url)
                .expect("Error Connecting to database")
        },
        #[cfg(feature = "postgres")]
        DatabaseType::Postgres => {
            PgConnection::establish(&CONFIG.database_url)
                .expect("Error Connecting to database")
        },
        #[cfg(feature = "sqlite")]
        DatabaseType::Sqlite => {
            SqliteConnection::establish(&CONFIG.database_url)
                .expect("Error Connecting to database")
        }
    }
}