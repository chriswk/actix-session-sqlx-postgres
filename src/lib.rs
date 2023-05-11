use crate::ConnectionData::{ConnectionPool, ConnectionString};
use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use chrono::Utc;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng as _};
use serde_json::{self, Value};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use std::sync::Arc;
use time::Duration;

/// Use Postgres via Sqlx as session storage backend.
///
/// ```no_run
/// use actix_web::{web, App, HttpServer, HttpResponse, Error};
/// use actix_session_sqlx::SqlxPostgresqlSessionStore;
/// use actix_session::SessionMiddleware;
/// use actix_web::cookie::Key;
///
/// // The secret key would usually be read from a configuration file/environment variables.
/// fn get_secret_key() -> Key {
///     # todo!()
///     // [...]
/// }
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     let secret_key = get_secret_key();
///     let psql_connection_string = "postgres://<username>:<password>@127.0.0.1:5432/<yourdatabase>";
///     let store = SqlxPostgresqlSessionStore::new(psql_connection_string).await.unwrap();
///
///     HttpServer::new(move ||
///             App::new()
///             .wrap(SessionMiddleware::new(
///                 store.clone(),
///                 secret_key.clone()
///             ))
///             .default_service(web::to(|| HttpResponse::Ok())))
///         .bind(("127.0.0.1", 8080))?
///         .run()
///         .await
/// }
/// ```
/// If you already have a connection pool, you can use something like
/*/// ```no_run
/// use actix_web::{web, App, HttpServer, HttpResponse, Error};
/// use actix_session_sqlx::SqlxPostgresqlSessionStore;
/// use actix_session::SessionMiddleware;
/// use actix_web::cookie::Key;
///
/// // The secret key would usually be read from a configuration file/environment variables.
/// fn get_secret_key() -> Key {
///     # todo!()
///     // [...]
/// }
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     use sqlx::postgres::PgPoolOptions;
/// let secret_key = get_secret_key();
///     let pool = PgPoolOptions::find_some_way_to_build_your_pool(psql_connection_string);
///     let store = SqlxPostgresqlSessionStore::from_pool(pool).await.expect("Could not build session store");
///
///     HttpServer::new(move ||
///             App::new()
///             .wrap(SessionMiddleware::new(
///                 store.clone(),
///                 secret_key.clone()
///             ))
///             .default_service(web::to(|| HttpResponse::Ok())))
///         .bind(("127.0.0.1", 8080))?
///         .run()
///         .await
/// }
/// ```
*/
#[derive(Clone)]
struct CacheConfiguration {
    cache_keygen: Arc<dyn Fn(&str) -> String + Send + Sync>,
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            cache_keygen: Arc::new(str::to_owned),
        }
    }
}

#[derive(Clone)]
pub struct SqlxPostgresqlSessionStore {
    client_pool: Pool<Postgres>,
    configuration: CacheConfiguration,
}

fn generate_session_key() -> SessionKey {
    let value = std::iter::repeat(())
        .map(|()| OsRng.sample(Alphanumeric))
        .take(64)
        .collect::<Vec<_>>();

    // These unwraps will never panic because pre-conditions are always verified
    // (i.e. length and character set)
    String::from_utf8(value).unwrap().try_into().unwrap()
}

impl SqlxPostgresqlSessionStore {
    pub fn builder<S: Into<String>>(connection_string: S) -> SqlxPostgresqlSessionStoreBuilder {
        SqlxPostgresqlSessionStoreBuilder {
            connection_data: ConnectionString(connection_string.into()),
            configuration: CacheConfiguration::default(),
        }
    }

    pub async fn new<S: Into<String>>(
        connection_string: S,
    ) -> Result<SqlxPostgresqlSessionStore, anyhow::Error> {
        Self::builder(connection_string).build().await
    }

    pub async fn from_pool(pool: Pool<Postgres>) -> SqlxPostgresqlSessionStoreBuilder {
        SqlxPostgresqlSessionStoreBuilder {
            connection_data: ConnectionPool(pool),
            configuration: CacheConfiguration::default(),
        }
    }
}

pub enum ConnectionData {
    ConnectionString(String),
    ConnectionPool(Pool<Postgres>),
}

#[must_use]
pub struct SqlxPostgresqlSessionStoreBuilder {
    connection_data: ConnectionData,
    configuration: CacheConfiguration,
}

impl SqlxPostgresqlSessionStoreBuilder {
    pub async fn build(self) -> Result<SqlxPostgresqlSessionStore, anyhow::Error> {
        match self.connection_data {
            ConnectionString(conn_string) => PgPoolOptions::new()
                .max_connections(1)
                .connect(conn_string.as_str())
                .await
                .map_err(Into::into)
                .map(|pool| SqlxPostgresqlSessionStore {
                    client_pool: pool,
                    configuration: self.configuration,
                }),
            ConnectionPool(pool) => Ok(SqlxPostgresqlSessionStore {
                client_pool: pool,
                configuration: self.configuration,
            }),
        }
    }
}
pub(crate) type SessionState = HashMap<String, String>;

#[async_trait::async_trait(?Send)]
impl SessionStore for SqlxPostgresqlSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let key = (self.configuration.cache_keygen)(session_key.as_ref());
        let row =
            sqlx::query("SELECT session_state FROM sessions WHERE key = $1 AND expires > NOW()")
                .bind(key)
                .fetch_optional(&self.client_pool)
                .await
                .map_err(Into::into)
                .map_err(LoadError::Other)?;
        match row {
            None => Ok(None),
            Some(r) => {
                let data: Value = r.get("session_state");
                let state: SessionState = serde_json::from_value(data)
                    .map_err(Into::into)
                    .map_err(LoadError::Deserialization)?;
                Ok(Some(state))
            }
        }
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        let body = serde_json::to_value(&session_state)
            .map_err(Into::into)
            .map_err(SaveError::Serialization)?;
        let key = generate_session_key();
        let cache_key = (self.configuration.cache_keygen)(key.as_ref());
        let expires = Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        sqlx::query("INSERT INTO sessions(key, session_state, expires) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(cache_key)
            .bind(body)
            .bind(expires)
            .execute(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(SaveError::Other)?;
        Ok(key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let body = serde_json::to_value(&session_state)
            .map_err(Into::into)
            .map_err(UpdateError::Serialization)?;
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());
        let new_expires = Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        sqlx::query("UPDATE sessions SET session_state = $1, expires = $2 WHERE key = $3")
            .bind(body)
            .bind(new_expires)
            .bind(cache_key)
            .execute(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(session_key)
    }

    async fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> Result<(), anyhow::Error> {
        let new_expires = Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        let key = (self.configuration.cache_keygen)(session_key.as_ref());
        sqlx::query("UPDATE sessions SET expires = $1 WHERE key = $2")
            .bind(new_expires)
            .bind(key)
            .execute(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        let key = (self.configuration.cache_keygen)(session_key.as_ref());
        sqlx::query("DELETE FROM sessions WHERE key = $1")
            .bind(key)
            .execute(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(())
    }
}
