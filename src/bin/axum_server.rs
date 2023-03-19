use std::net::SocketAddr;

use hello_tonic::{MyGreeter, pb::hello::greeter_server::GreeterServer, multiplex_service::MultiplexService};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{routing::get, Router};

#[tokio::main]
async fn main()  {


    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hello_server=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
 
    // build the rest service
    let rest = Router::new().route("/", get(web_root));

    // build the grpc service
    let grpc = GreeterServer::new(MyGreeter::default());

    let service = MultiplexService::new(rest, grpc);

    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();
}



async fn web_root() -> &'static str {
    "Hello, World!"
}
