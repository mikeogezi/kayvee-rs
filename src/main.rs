use std::env;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod config;
mod context;
mod schema;
mod store;

use crate::config::{DEBUG, PORT};
use crate::context::GraphQLContext;
use crate::schema::{create_schema, Schema};
use crate::store::Store;

async fn graphiql() -> HttpResponse {
    let endpoint: String;
    unsafe {
        endpoint = format!("//127.0.0.1:{}/graphql", PORT);
    }
    let html = graphiql_source(endpoint.as_str());
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<(Schema, GraphQLContext)>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st.0, &st.1);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Use default port or pull from env vars
    let port: String;
    let debug: bool;
    match env::var("PORT") {
        Ok(val) => port = val,
        _ => port = String::from("8080"),
    }
    match env::var("DEBUG") {
        Ok(_) => debug = true,
        _ => debug = false,
    }

    // Create Juniper schema
    let store = Mutex::new(Store::with_debug(debug));
    let gql_context = GraphQLContext { store };

    let schema = std::sync::Arc::new((create_schema(), gql_context));

    unsafe {
        DEBUG = debug;
        PORT = port.to_string();
    }
    // Start http server
    let factory = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    });

    let bind_result = factory.bind(format!("127.0.0.1:{}", port).as_str());

    if bind_result.is_ok() {
        println!(
            "The Kayvee service started successfully. It's running on port {}",
            port
        );
    } else {
        println!("Couldn't start the Kayvee service")
    }
    return bind_result.unwrap().run().await;
}
