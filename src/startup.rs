use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;

use crate::{
    cache::app_cache::AppCache,
    configuration::{DatabaseSettings, Settings},
};

/// Application struct
pub struct Application {
    #[allow(unused)]
    port: u16,
    server: Server,
}

impl Application {
    /// Build the application
    pub async fn build(configuration: Settings) -> Result<Self, Box<dyn std::error::Error>> {
        let connection_pool = get_connection_pool(&configuration.database);
        let app_cache = AppCache::build(&connection_pool).await?;
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listner = TcpListener::bind(address)?;
        let port = listner.local_addr().unwrap().port();
        let server = run(listner, app_cache)?;
        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Returns the connection pool from the postgres db
pub fn get_connection_pool(db_settings: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect_lazy_with(db_settings.connect_db())
}

/// Builds the http server and returns it
pub fn run(listen: TcpListener, app_cache: AppCache) -> Result<Server, std::io::Error> {
    let app_cache = web::Data::new(app_cache);
    let server = HttpServer::new(move || App::new().app_data(app_cache.clone()))
        .listen(listen)?
        .run();

    Ok(server)
}
