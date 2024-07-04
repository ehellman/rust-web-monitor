use db::{DatabaseOperations, NewProject};
use sqlx::error::BoxDynError;

mod crawler;
mod db;

#[tokio::main]
async fn main() -> Result<(), BoxDynError> {
    dotenv::dotenv().ok();

    // let url = "https://www.lyko.com/sv";
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_db(&database_url).await?;

    let selectors = vec![crawler::InteractionSelector {
        selector: "#CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll".to_string(),
    }];

    // let create_result = db::Project::create(
    //     &pool,
    //     &db::NewProject {
    //         name: "Lyko".to_string(),
    //     },
    // )
    // .await?;

    // let delete_result = db::Project::delete(&pool, 3).await?;

    // println!("Deleted: {:?}", delete_result);

    Ok(())

    // match crawler::crawl_url(url, selectors).await {
    //     Ok(content) => println!("success"),
    //     Err(e) => eprintln!("Error fetching URL: {}", e),
    // }
}
