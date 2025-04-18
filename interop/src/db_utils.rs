use surrealdb::engine::any;
use surrealdb::opt::auth::Root;
use std::env;

type Db = surrealdb::Surreal<surrealdb::engine::any::Any>;

pub async fn db_init() -> Result<Db, Box<dyn std::error::Error>> {
    let db =  {
        let endpoint = env::var("SURREAL_LOCAL_ENDPOINT")
            .expect("Expected `SURREAL_LOCAL_ENDPOINT` environment variable to be present");
        
        if let Ok(db) = any::connect(endpoint).await { db } 
        else {
            let endpoint = env::var("SURREAL_REMOTE_ENDPOINT")
                .expect("Expected `SURREAL_REMOTE_ENDPOINT` environment variable to be present");
            any::connect(endpoint).await?
        }
    };

    db.signin(Root {
		username: &env::var("DB_USER")?,
		password: &env::var("DB_PASS")?,
	}).await?;
    
    db.use_ns(env::var("SURREAL_NS")?)
      .use_db(env::var("SURREAL_DB")?)
      .await?;

    db.query("DEFINE TABLE note SCHEMALESS").await?;

    Ok(db)
}