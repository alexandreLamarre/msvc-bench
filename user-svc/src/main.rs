mod api;
mod traits;
use crate::api::User;
use crate::traits::UserStore;
use async_trait::async_trait;
use axum::{extract::Extension, routing::get, Router};
use clap::Parser;
use mongodb::{options::ClientOptions, Client};
use std::{
    cell::{RefCell, RefMut},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

static APP_NAME: &'static str = "user-svc";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, value_parser)]
    mongodb_url: String,
    #[clap(short, long, value_parser)]
    port: u16,
}

#[derive(Clone)]
pub struct ServiceState {
    mongodb: Arc<Box<dyn UserStore>>,
}

// Abstract away access to the database
#[async_trait]
impl UserStore for mongodb::Client {
    async fn insert_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    async fn query_user_name(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    async fn query_user_id(&self, id: String) -> Result<User, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl ServiceState {
    async fn default() -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some(APP_NAME.to_string());
        let client = Client::with_options(client_options)?;
        Ok(ServiceState {
            mongodb: Arc::new(Box::new(client)),
        })
    }
    async fn try_or_default(mongo_uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_options = match ClientOptions::parse(mongo_uri).await {
            Err(e) => {
                println!("{}", e);
                return ServiceState::default().await;
            }
            Ok(options) => options,
        };
        client_options.app_name = Some(APP_NAME.to_string());
        let client = Client::with_options(client_options)?;
        Ok(ServiceState {
            mongodb: Arc::new(Box::new(client)),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let shared_state = Arc::new(
        ServiceState::try_or_default(args.mongodb_url.as_str())
            .await
            .unwrap(),
    );
    // Enable http tracing with RUST_LOG=tower_http=trace cargo run
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(shared_state)),
        );
    let addr = format!("0.0.0.0:{}", args.port);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
