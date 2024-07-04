#[cfg(test)]
mod tests {
    use web_monitor::db::{init_test_db, DbOperations, NewProject, Project, UpdateProject};

    #[tokio::test]
    async fn test_create_project() {
        let pool = init_test_db()
            .await
            .expect("Failed to initialize test database");

        let new_project = NewProject {
            name: "Test Project".to_string(),
        };
        let project_id = Project::create(&pool, &new_project)
            .await
            .expect("Failed to create project");

        assert!(project_id > 0);
    }

    #[tokio::test]
    async fn test_get_all_projects() {
        let pool = init_test_db()
            .await
            .expect("Failed to initialize test database");

        let new_project = NewProject {
            name: "Test Project".to_string(),
        };
        Project::create(&pool, &new_project)
            .await
            .expect("Failed to create project");

        let projects = Project::get_all(&pool)
            .await
            .expect("Failed to get projects");

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");
    }

    #[tokio::test]
    async fn test_update_project() {
        let pool = init_test_db()
            .await
            .expect("Failed to initialize test database");

        let new_project = NewProject {
            name: "Test Project".to_string(),
        };
        let project_id = Project::create(&pool, &new_project)
            .await
            .expect("Failed to create project");

        let updated_project = UpdateProject {
            id: project_id,
            name: Some("Updated Project".to_string()),
        };
        Project::update(&pool, &updated_project)
            .await
            .expect("Failed to update project");

        let projects = Project::get_all(&pool)
            .await
            .expect("Failed to fetch projects");

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Updated Project");
    }

    #[tokio::test]
    async fn test_delete_project() {
        let pool = init_test_db()
            .await
            .expect("Failed to initialize test database");

        let new_project = NewProject {
            name: "Test Project".to_string(),
        };
        let project_id = Project::create(&pool, &new_project)
            .await
            .expect("Failed to create project");

        let deleted = Project::delete(&pool, project_id)
            .await
            .expect("Failed to delete project");

        assert!(deleted > 0);

        let projects = Project::get_all(&pool)
            .await
            .expect("Failed to fetch projects");

        assert!(projects.is_empty());
    }
}
