use std::collections::HashMap;
use std::sync::Arc;
use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use sqlx::postgres::PgPoolOptions;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng as _};
use time::Duration;
use serde_json;

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
            connection_string: connection_string.into(),
            configuration: CacheConfiguration::default()
        }
    }

    pub async fn new<S: Into<String>>(connection_string: S) -> Result<SqlxPostgresqlSessionStore, anyhow::Error> {
        Self::builder(connection_string).build().await
    }
}

#[must_use]
pub struct SqlxPostgresqlSessionStoreBuilder {
    connection_string: String,
    configuration: CacheConfiguration,
}

impl SqlxPostgresqlSessionStoreBuilder {
    pub async fn build(self) -> Result<SqlxPostgresqlSessionStore, anyhow::Error> {
        PgPoolOptions::new()
            .max_connections(1)
            .connect(self.connection_string.as_str())
            .await
            .map_err(Into::into)
            .map(|pool| {
                SqlxPostgresqlSessionStore {
                    client_pool: pool,
                    configuration: self.configuration
                }
            })
    }
}
pub(crate) type SessionState = HashMap<String, String>;

#[async_trait::async_trait(?Send)]
impl SessionStore for SqlxPostgresqlSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let key = (self.configuration.cache_keygen)(session_key.as_ref());
        let row = sqlx::query("SELECT session_state FROM sessions WHERE key = $1 AND expiry > NOW()")
            .bind( key)
            .fetch_optional(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(LoadError::Other)?;
        match row {
            None => Ok(None),
            Some(r) => {
                let data: String = r.get("session_state");
                let state: SessionState = serde_json::from_str(&data).map_err(Into::into).map_err(LoadError::Deserialization)?;
                Ok(Some(state))
            }
        }
    }

    async fn save(&self, session_state: SessionState, ttl: &Duration) -> Result<SessionKey, SaveError> {
        let body = serde_json::to_string(&session_state)
            .map_err(Into::into)
            .map_err(SaveError::Serialization)?;
        let key = generate_session_key();
        let cache_key = (self.configuration.cache_keygen)(key.as_ref());
        let expiry = Utc::now() + chrono::Duration::seconds(ttl.whole_seconds() as i64);
        sqlx::query("INSERT INTO sessions(key, value, expiry) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(cache_key)
            .bind( body)
            .bind( expiry)
            .execute(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(SaveError::Other)?;
        Ok(key)
    }

    async fn update(&self, session_key: SessionKey, session_state: SessionState, ttl: &Duration) -> Result<SessionKey, UpdateError> {
        let body = serde_json::to_string(&session_state).map_err(Into::into).map_err(UpdateError::Serialization)?;
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());
        let new_expiry = Utc::now() + chrono::Duration::seconds(ttl.whole_seconds());
        sqlx::query("UPDATE sessions SET value = $1, expiry = $2 WHERE key = $3")
            .bind( body)
            .bind( new_expiry)
            .bind( cache_key)
            .execute(&self.client_pool)
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;
        Ok(session_key)
    }

    async fn update_ttl(&self, session_key: &SessionKey, ttl: &Duration) -> Result<(), anyhow::Error> {
        let new_expiry = Utc::now() + chrono::Duration::seconds(ttl.whole_seconds() as i64);
        let key = (self.configuration.cache_keygen)(session_key.as_ref());
        sqlx::query("UPDATE sessions SET expiry = $1 WHERE key = $2")
            .bind(new_expiry)
            .bind( key)
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
