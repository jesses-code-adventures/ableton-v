#![allow(dead_code)]
use std::fmt::Display;
use xml::reader::XmlEvent;

#[derive(Debug, Clone)]
pub enum ParserOutput<'a> {
    Ableton(Ableton<'a>),
    AbletonEnd(XmlEvent),
    LiveSet(LiveSet<'a>),
    LiveSetEnd(XmlEvent),
    UnchangedChunk(XmlEvent),
    TracksStart(XmlEvent),
    TracksEnd(XmlEvent),
    Close(XmlEvent),
    EndDocument,
    None,
}

impl<'a> From<&XmlEvent> for ParserOutput<'a> {
    fn from(event: &XmlEvent) -> ParserOutput<'a> {
        match event {
            XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "Ableton" => ParserOutput::Ableton(Ableton::from(event)),
                "LiveSet" => ParserOutput::LiveSet(LiveSet::from(event)),
                "Tracks" => ParserOutput::TracksStart(event.clone()),
                _ => {
                    println!("undefined: \n{}\n{:?}", name.local_name, event);
                    ParserOutput::UnchangedChunk(event.clone())
                }
            },
            XmlEvent::EndElement { name, .. } => match name.local_name.as_str() {
                "Ableton" => ParserOutput::AbletonEnd(event.clone()),
                "LiveSet" => ParserOutput::LiveSetEnd(event.clone()),
                "Tracks" => ParserOutput::TracksEnd(event.clone()),
                _ => ParserOutput::Close(event.clone()),
            },
            XmlEvent::StartDocument { .. } => ParserOutput::EndDocument,
            _ => ParserOutput::None,
        }
    }
}

impl<'a> Display for ParserOutput<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserOutput::Ableton(ableton) => write!(f, "{}", ableton),
            ParserOutput::AbletonEnd(chunk) => write!(f, "AbletonEnd command for {:?}", chunk),
            ParserOutput::LiveSet(live_set) => write!(f, "{}", live_set),
            ParserOutput::LiveSetEnd(chunk) => write!(f, "LiveSetEnd command for {:?}", chunk),
            ParserOutput::UnchangedChunk(chunk) => write!(f, "{:?}", chunk),
            ParserOutput::TracksStart(chunk) => write!(f, "TracksStart command for {:?}", chunk),
            ParserOutput::TracksEnd(chunk) => write!(f, "TracksEnd command for {:?}", chunk),
            ParserOutput::Close(chunk) => write!(f, "Close command for {:?}", chunk),
            ParserOutput::EndDocument => write!(f, "EndDocument"),
            ParserOutput::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ableton<'a> {
    pub major_version: String,
    pub minor_version: String,
    pub schema_change_count: String,
    pub creator: String,
    pub revision: String,
    pub children: Vec<&'a ParserOutput<'a>>,
}

impl Display for Ableton<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Ableton major_version=\"{}\" minor_version=\"{}\" schema_change_count=\"{}\" creator=\"{}\" revision=\"{}\">",
            self.major_version, self.minor_version, self.schema_change_count, self.creator, self.revision
        )
    }
}

impl From<&XmlEvent> for Ableton<'_> {
    fn from(event: &XmlEvent) -> Self {
        match event {
            XmlEvent::StartElement { attributes, .. } => Ableton {
                major_version: attributes.get(0).unwrap().value.clone(),
                minor_version: attributes.get(1).unwrap().value.clone(),
                schema_change_count: attributes.get(2).unwrap().value.clone(),
                creator: attributes.get(3).unwrap().value.clone(),
                revision: attributes.get(4).unwrap().value.clone(),
                children: vec![],
            },
            _ => {
                panic!("not an ableton definition")
            }
        }
    }
}

impl<'a> Ableton<'a> {
    pub fn add_child(&mut self, child: &'a ParserOutput) {
        self.children.push(child);
    }
}

#[derive(Debug, Clone)]
pub struct LiveSet<'a> {
    pub parent: Option<&'a Ableton<'a>>,
    pub children: Vec<&'a ParserOutput<'a>>,
}

impl<'a> LiveSet<'a> {
    pub fn add_child(&mut self, child: &'a ParserOutput) {
        self.children.push(child);
    }
    pub fn set_parent(&mut self, parent: &'a Ableton) {
        self.parent = Some(parent);
    }
}

impl Display for LiveSet<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "live set chunk open",)
    }
}

impl From<&XmlEvent> for LiveSet<'_> {
    fn from(_: &XmlEvent) -> Self {
        LiveSet {
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tracks<'a> {
    pub parent: Option<&'a LiveSet<'a>>,
    pub children: Vec<&'a ParserOutput<'a>>,
}

pub struct Track<'a> {
    pub parent: Option<&'a Tracks<'a>>,
    pub children: Vec<&'a ParserOutput<'a>>,
}
