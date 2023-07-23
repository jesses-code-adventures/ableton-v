mod debugging;
mod parser;
mod project;
mod state;
mod version;

use debugging::debugging::get_project_paths;
use parser::als::AbletonXmlParser;
use project::project::AbletonProjectDirectory;
use state::database::Database;
use std::path::Path;
use std::fs;

#[tokio::main]
async fn main() {
    let db = Database::new().await;
    // let state = ProgramState::new(None, None);
    let mut paths = get_project_paths(db).await.unwrap();
    let first = paths.pop().unwrap();
    println!("reading from {}", first);
    let file = fs::File::open(&Path::new(&first)).unwrap();
    let mut parser = AbletonXmlParser::new();
    parser.parse_xml(file).unwrap();
}
