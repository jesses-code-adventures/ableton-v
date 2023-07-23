use crate::version::version::ProjectVersion;
use std::fmt::{self, Display};
use std::fs;
use std::path::PathBuf;

pub fn has_als_files(path: PathBuf) -> bool {
    if !path.as_path().is_dir() {
        return false;
    }
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path_str = path.unwrap().file_name().into_string().unwrap();
        let parts = path_str.split(".").collect::<Vec<&str>>();
        if parts.len() == 1 {
            continue;
        }
        if parts[1] == "als" {
            return true;
        }
    }
    false
}

pub struct AbletonProjectDirectory {
    pub name: String,
    pub path: PathBuf,
    pub versions: Vec<ProjectVersion>,
}

impl AbletonProjectDirectory {
    pub fn new(path_buf: PathBuf) -> AbletonProjectDirectory {
        let path = path_buf.as_path();
        let mut name = String::from(path.file_name().unwrap().to_str().unwrap());
        if name.ends_with(" Project") {
            name = String::from(name.strip_suffix(" Project").unwrap());
        }
        let dir = fs::read_dir(path);
        let mut versions = vec![];
        for entry in dir.unwrap() {
            let path = entry.as_ref().unwrap().path();
            let parts: Vec<&str> = path.to_str().unwrap().split(".").collect();
            if parts.len() == 1 {
                continue;
            }
            if parts[1] != "als" {
                continue;
            }
            let metadata = entry.as_ref().unwrap().metadata();
            let created_at = metadata.as_ref().unwrap().created().unwrap();
            let modified_at = metadata.as_ref().unwrap().modified().unwrap();
            let accessed_at = metadata.as_ref().unwrap().accessed().unwrap();
            let version = ProjectVersion::new(
                entry.as_ref().unwrap().path(),
                created_at,
                modified_at,
                accessed_at,
                None,
                None,
            );
            versions.push(version);
        }
        return AbletonProjectDirectory {
            name,
            path: path_buf,
            versions,
        };
    }
}

impl Display for AbletonProjectDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "project: {}\nproject path: {:?}\n", self.name, self.path).unwrap();
        write!(f, "{:?} versions:\n", self.versions.len()).unwrap();
        for version in &self.versions {
            write!(f, "{}", version).unwrap();
        }
        write!(f, "\n")
    }
}
