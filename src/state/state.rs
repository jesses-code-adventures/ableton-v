#![allow(dead_code)]
use crate::project::project::{has_als_files, AbletonProjectDirectory};
use std::{
    fmt::{Display, Error, Formatter},
    fs,
    path::PathBuf,
};
use sysinfo::{System, SystemExt};

pub struct ProgramState {
    pub ableton_application_path: PathBuf,
    pub ableton_running: bool,
    pub ableton_projects_directory: PathBuf,
    pub ableton_projects: Vec<AbletonProjectDirectory>,
}

impl ProgramState {
    pub fn new(
        mut sessions_path: Option<PathBuf>,
        mut ableton_path: Option<PathBuf>,
    ) -> ProgramState {
        if sessions_path.is_none() {
            let mut new_path = PathBuf::new();
            new_path.push("/Users/jessewilliams/Ableton Sessions");
            sessions_path = Some(new_path);
        }
        if ableton_path.is_none() {
            let mut new_path = PathBuf::new();
            new_path.push("/Applications/Ableton Live 11 Suite.app");
            ableton_path = Some(new_path);
        }
        assert!(sessions_path.as_ref().unwrap().as_path().is_dir());
        let ableton_projects = get_projects_and_versions(&sessions_path.clone().unwrap());
        ProgramState {
            ableton_application_path: ableton_path.unwrap(),
            ableton_running: ableton_is_running(),
            ableton_projects_directory: sessions_path.unwrap(),
            ableton_projects,
        }
    }

    #[allow(dead_code)]
    pub fn refresh_ableton_sessions(&mut self) {
        self.ableton_projects = get_projects_and_versions(&self.ableton_projects_directory)
    }

    fn count_projects(&self) -> usize {
        self.ableton_projects.len()
    }

    fn count_versions(&self) -> usize {
        let mut version_count = 0;
        for project in &self.ableton_projects {
            for _ in &project.versions {
                version_count += 1;
            }
        }
        version_count
    }

    #[allow(dead_code)]
    pub fn print_projects(&self) {
        for project in &self.ableton_projects {
            println!("{}", project);
        }
    }

    #[allow(dead_code)]
    pub fn print_project_names(&self) {
        for project in &self.ableton_projects {
            println!("{}", project.name);
        }
    }
}

impl Display for ProgramState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Program State:\n    Ableton application path: {:?}\n    Ableton sessions directory: {:?}\n    Ableton running: {:?}\n    Ableton projects found: {:?}\n    Ableton project versions found: {:?}\n\n",
            self.ableton_application_path, self.ableton_projects_directory, self.ableton_running, self.count_projects(), self.count_versions(),
        )
    }
}

fn ableton_is_running() -> bool {
    System::new_all()
        .processes_by_exact_name("Live")
        .next()
        .is_some()
}

fn get_session_directories(search_dir: fs::ReadDir) -> Vec<PathBuf> {
    let mut ableton_session_directories = vec![];
    let mut search_dirs: Vec<PathBuf> = search_dir.map(|x| return x.unwrap().path()).collect();
    while search_dirs.len() > 0 {
        let current_path = search_dirs.pop().unwrap();
        if !current_path.as_path().is_dir() {
            continue;
        }
        let current_dir = fs::read_dir(current_path).unwrap();
        for path in current_dir {
            if has_als_files(path.as_ref().unwrap().path()) {
                ableton_session_directories.push(path.as_ref().unwrap().path());
                continue;
            }
            if !path.as_ref().unwrap().path().is_dir() {
                continue;
            }
            search_dirs.push(path.as_ref().unwrap().path());
        }
    }
    return ableton_session_directories;
}

fn get_projects_and_versions(ableton_projects_path: &PathBuf) -> Vec<AbletonProjectDirectory> {
    let root_dir = fs::read_dir(ableton_projects_path).expect("path to be valid");
    let ableton_session_directories = get_session_directories(root_dir);
    let mut project_directories: Vec<AbletonProjectDirectory> = vec![];
    for path in ableton_session_directories {
        let ableton_project_directory = AbletonProjectDirectory::new(path);
        project_directories.push(ableton_project_directory);
    }
    project_directories
}
