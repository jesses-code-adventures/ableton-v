#![allow(dead_code)]
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_xml_event_names(xml: BufReader<GzDecoder<File>>) -> std::io::Result<()> {
    let parser = EventReader::new(xml);
    let mut current_depth = 0;
    let mut depth_map: HashMap<u32, Vec<String>> = HashMap::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                current_depth += 1;
                let entry = depth_map.entry(current_depth).or_insert_with(Vec::new);
                if !entry.contains(&name.local_name) {
                    entry.push(name.local_name.clone());
                }
            }
            Ok(XmlEvent::EndElement { .. }) => {
                if current_depth > 0 {
                    current_depth -= 1;
                }
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {}
        }
    }

    for (current_depth, names) in &depth_map {
        println!("Depth {}: Unique names {:?}", current_depth, names);
    }

    Ok(())
}
