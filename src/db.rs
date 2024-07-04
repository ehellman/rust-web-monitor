use sqlx::{error::BoxDynError, Sqlite, SqlitePool};
use std::error::Error;

// init db
pub async fn init_db(db_url: &str) -> Result<SqlitePool, BoxDynError> {
    // todo: check if db exists, if not create it
    let pool = SqlitePool::connect(db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn init_test_db() -> Result<SqlitePool, BoxDynError> {
    let pool = SqlitePool::connect("sqlite::memory:?cache=shared").await?;
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            code TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        );
        CREATE TABLE IF NOT EXISTS requests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            language_id INTEGER NOT NULL,
            url TEXT NOT NULL,
            body TEXT NOT NULL,
            response_time REAL,
            file_size INTEGER,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (language_id) REFERENCES languages(id)
        );
        "#
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

pub trait DbOperations {
    type Entity;
    type NewEntity;
    type UpdateEntity;

    async fn create(pool: &SqlitePool, new_entity: &Self::NewEntity) -> Result<i64, BoxDynError>;
    async fn delete(pool: &SqlitePool, id: i64) -> Result<u64, BoxDynError>;
    async fn update(
        pool: &SqlitePool,
        update_entity: &Self::UpdateEntity,
    ) -> Result<Self::Entity, BoxDynError>;
    async fn get_all(pool: &SqlitePool) -> Result<Vec<Self::Entity>, BoxDynError>;
    async fn get(pool: &SqlitePool, id: i64) -> Result<Self::Entity, BoxDynError>;
}

#[derive(Debug)]
pub struct InteractionSelector {
    pub selector: String,
}

#[derive(Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub interaction_selectors: Vec<InteractionSelector>,
}

#[derive(Debug)]
pub struct NewProject {
    pub name: String,
    pub interaction_selectors: Option<Vec<InteractionSelector>>,
}

#[derive(Debug)]
pub struct UpdateProject {
    pub id: i64,
    pub name: Option<String>,
    pub interaction_selectors: Option<Vec<InteractionSelector>>,
}

impl DbOperations for Project {
    type Entity = Project;
    type NewEntity = NewProject;
    type UpdateEntity = UpdateProject;

    async fn get_all(pool: &SqlitePool) -> Result<Vec<Self::Entity>, BoxDynError> {
        let rows = sqlx::query!("SELECT id, name FROM projects")
            .fetch_all(pool)
            .await?;

        let projects = rows
            .into_iter()
            .map(|row| Self::Entity {
                id: row.id,
                name: row.name,
                interaction_selectors: row.interaction_selectors,
            })
            .collect();

        Ok(projects)
    }
    async fn get(pool: &SqlitePool, id: i64) -> Result<Self::Entity, BoxDynError> {
        let result = sqlx::query_as!(Self::Entity, "SELECT * FROM projects WHERE id = ?", id)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    async fn create(pool: &SqlitePool, new_entity: &Self::NewEntity) -> Result<i64, BoxDynError> {
        let result = sqlx::query!("INSERT INTO projects (name) VALUES (?)", new_entity.name)
            .execute(pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete(pool: &SqlitePool, id: i64) -> Result<u64, BoxDynError> {
        let result = sqlx::query!("DELETE FROM projects WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected().clone())
    }
    async fn update(
        pool: &SqlitePool,
        update_entity: &Self::UpdateEntity,
    ) -> Result<Self::Entity, BoxDynError> {
        let result = Self::get(pool, update_entity.id).await?;

        let update_result = sqlx::query(r#"UPDATE projects SET name = ? WHERE id = ?"#)
            .bind(update_entity.name.as_ref().unwrap_or_else(|| &result.name))
            .bind(result.id)
            .execute(pool)
            .await?;

        // todo: error if no rows affected?
        if update_result.rows_affected() == 0 {
            //error?
        }

        let updated_project = Self::get(pool, update_entity.id).await?;

        Ok(updated_project)
    }
}

// create language
