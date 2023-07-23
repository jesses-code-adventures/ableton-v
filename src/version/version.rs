use crate::state::database::DatabaseModel;
use std::fmt::{self, Display};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub struct ProjectVersion {
    pub path: PathBuf,
    pub name: String,
    pub created_at: SystemTime,
    pub accessed_at: SystemTime,
    pub modified_at: SystemTime,
    pub description: Option<String>,
}

impl ProjectVersion {
    pub fn new(
        path: PathBuf,
        created_at: SystemTime,
        accessed_at: SystemTime,
        modified_at: SystemTime,
        description: Option<String>,
        name: Option<String>,
    ) -> ProjectVersion {
        match name {
            Some(name) => ProjectVersion {
                path,
                name,
                created_at,
                accessed_at,
                modified_at,
                description,
            },
            None => {
                let mut name = String::from(path.as_path().file_name().unwrap().to_str().unwrap());
                let parts: Vec<&str> = name.split(".").collect();
                name = String::from(parts[0]);
                ProjectVersion {
                    path,
                    name,
                    created_at,
                    accessed_at,
                    modified_at,
                    description,
                }
            }
        }
    }
}

impl DatabaseModel for ProjectVersion {
    fn create_table_query(&self) -> String {
        format!("CREATE TABLE IF NOT EXISTS project_version (path varchar(300), name varchar(150), created_at timestamp, accessed_at timestamp, modified_at timestamp, description varchar(300), primary key (name, created_at))")
    }

    fn insert_into_query(&self) -> String {
        format!("INSERT OR REPLACE INTO project_version (path, name, created_at, accessed_at, modified_at, description) VALUES ('{}', '{}', '{:?}', '{:?}', '{:?}', '{:?}')",
        self.path.to_str().unwrap().replace("'", "\""),
        self.name.replace("'", "\""),
        self.created_at.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        self.accessed_at.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        self.modified_at.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        self.description
        )
    }
}

impl Display for ProjectVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "---> version: {}\n        version path: {:?}\n        created: {:?}\n        updated: {:?}\n        accessed: {:?}\n        description: {:?}",
            self.name, self.path, self.created_at, self.modified_at, self.accessed_at, self.description
        )
    }
}
