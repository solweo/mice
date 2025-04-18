cfg_if::cfg_if! { if #[cfg(feature = "back")] {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use app::*;
    use leptos::logging::log;
    use dotenvy::dotenv;
    use serde::{Deserialize, Serialize};
    use surrealdb::{
        Surreal,
        engine::local::Mem
    };
    use surrealdb::engine::any;
    use surrealdb::opt::auth::Root;
    use surrealdb::opt::Config;
    use tokio::net::TcpListener;
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::Resource;
    use surrealdb::{Datetime, RecordId};
    use std::env;
    use std::env::args; 
}}

#[cfg(feature = "back")]
#[tokio::main]
async fn main() {
    dotenv().ok();

    // let db =  {
    //     let endpoint = env::var("SURREAL_LOCAL_ENDPOINT")
    //         .expect("Expected `SURREAL_LOCAL_ENDPOINT` environment variable to be present");
        
    //     if let Ok(db) = any::connect(endpoint).await { log!("cool!"); db } 
    //     else {
    //         let endpoint = env::var("SURREAL_REMOTE_ENDPOINT")
    //             .expect("Expected `SURREAL_REMOTE_ENDPOINT` environment variable to be present");
    //         any::connect(endpoint).await.unwrap()
    //     }
    // };

    // db.signin(Root {
	// 	username: &env::var("DB_USER").unwrap(),
	// 	password: &env::var("DB_PASS").unwrap(),
	// }).await.unwrap();
    
    // db.use_ns(env::var("SURREAL_NS").unwrap())
    //   .use_db(env::var("SURREAL_DB").unwrap())
    //   .await.unwrap();

    // db.query("DEFINE TABLE note SCHEMALESS").await.unwrap();

    let db = interop::db_init().await.unwrap();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(&leptos_options, routes, 
            move || provide_context(db.clone()), 
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
