#[cfg(test)]
pub mod tests {
    use actix_session::storage::SessionStore;
    use actix_session_sqlx::SqlxPostgresqlSessionStore;
    use sqlx::postgres::PgPoolOptions;
    use std::collections::HashMap;
    use testcontainers::clients;
    use testcontainers::core::WaitFor;
    use testcontainers::images::generic;

    async fn postgres_store(url: String) -> SqlxPostgresqlSessionStore {
        SqlxPostgresqlSessionStore::new(url).await.expect("")
    }

    #[actix_web::test]
    async fn test_session_workflow() {
        let docker = clients::Cli::default();
        let postgres = docker.run(
            generic::GenericImage::new("postgres", "14-alpine")
                .with_wait_for(WaitFor::message_on_stderr(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_DB", "sessions")
                .with_env_var("POSTGRES_PASSWORD", "example")
                .with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
                .with_env_var("POSTGRES_USER", "tests"),
        );
        let url = format!(
            "postgres://tests:example@localhost:{}/sessions",
            postgres.get_host_port_ipv4(5432)
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
             session_state TEXT,
             expires TIMESTAMP WITH TIME ZONE NOT NULL
        );"#,
        )
        .execute(&pool)
        .await
        .expect("Could not create table");
        let mut session = HashMap::new();
        session.insert("key".to_string(), "value".to_string());
        let data = postgres_store
            .save(session, &time::Duration::days(1))
            .await;
        println!("{:#?}", data);
        assert!(data
            .is_ok());
        //acceptance_test_suite(move || postgres_store.clone(), true).await;
    }
}
