mod test_helpers;
use actix_session::storage::SessionStore;
use actix_session_sqlx_postgres::SqlxPostgresqlSessionStore;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};

async fn postgres_store(url: String) -> SqlxPostgresqlSessionStore {
    SqlxPostgresqlSessionStore::new(url).await.expect("")
}

#[actix_web::test]
async fn test_session_workflow() {
    let postgres = GenericImage::new("postgres", "15-alpine")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_DB", "sessions")
        .with_env_var("POSTGRES_PASSWORD", "example")
        .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
        .with_env_var("POSTGRES_USER", "tests")
        .start()
        .await
        .expect("Postgres started");
    let url = format!(
        "postgres://tests:example@localhost:{}/sessions",
        postgres
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port")
    );

    let postgres_store = postgres_store(url.clone()).await;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(url.as_str())
        .await
        .expect("Could not connect to database");
    sqlx::query(
        r#"CREATE TABLE sessions(
             key TEXT PRIMARY KEY NOT NULL,
             session_state JSONB,
             expires TIMESTAMP WITH TIME ZONE NOT NULL
        );"#,
    )
    .execute(&pool)
    .await
    .expect("Could not create table");
    let mut session = HashMap::new();
    session.insert("key".to_string(), "value".to_string());
    let data = postgres_store.save(session, &time::Duration::days(1)).await;
    println!("{:#?}", data);
    assert!(data.is_ok());
    test_helpers::acceptance_test_suite(move || postgres_store.clone(), true).await;
}
