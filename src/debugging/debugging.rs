#![allow(dead_code, unused_imports)]
// use std::time::SystemTime;

use crate::state::database::{Database, DatabaseModel};
// use crate::version::version::ProjectVersion;
use crate::AbletonProjectDirectory;
use sqlx::{sqlite::SqliteRow, Error, Row};

pub async fn update_projects(
    mut db: Database,
    ableton_projects: Vec<AbletonProjectDirectory>,
) -> anyhow::Result<()> {
    for project in ableton_projects {
        for version in project.versions {
            let query = version.insert_into_query();
            let _ = db.execute_insert(query).await?;
        }
    }
    Ok(())
}

pub async fn get_project_names(mut db: Database) -> anyhow::Result<Vec<String>> {
    Ok(db.execute_fetchall(String::from("select name from project_version")).await?.iter().map(|row| row.get::<String, usize>(0)).collect::<Vec<String>>())
}

pub async fn get_project_paths(mut db: Database) -> anyhow::Result<Vec<String>> {
    Ok(db.execute_fetchall(String::from("select path from project_version")).await?.iter().map(|row| row.get::<String, usize>(0)).collect::<Vec<String>>())
}

// pub async fn get_project_versions(mut db: Database) -> Result<Vec<ProjectVersion>, Error> {
//     let resp = db.execute_fetchall(String::from("select name, path, created_at, accessed_at, modified_at, description  from project_version")).await;
//     let mut versions: Vec<ProjectVersion> = vec![];
//     for version in resp.as_ref().expect("resp to be successful") {
//         let version = ProjectVersion::from(version);
//         versions.push(version);
//     }
//     Ok(versions)
// }

// impl From<&SqliteRow> for ProjectVersion {
//     fn from(row: &SqliteRow) -> Self {
//         let name = row.get::<String, usize>(0);
//         let path = row.get::<String, usize>(1);
//         let created_at = row.get::<SystemTime, usize>(2);
//         let accessed_at = row.get::<DateTime<Utc>, usize>(3);
//         let modified_at = row.get::<DateTime<Utc>, usize>(4);
//         let description = row.get::<String, usize>(5);
//         ProjectVersion::new(
//             path.into(),
//             created_at.into(),
//             accessed_at.into(),
//             modified_at.into(),
//             Some(description),
//             Some(name),
//         )
//     }
// }
