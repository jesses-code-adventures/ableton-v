#![allow(dead_code)]
use crate::parser::structs::ableton::ParserOutput;
use anyhow::Result;
use flate2::read::GzDecoder;
use std::collections::LinkedList;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub struct AbletonXmlParser<'a> {
    tree: AbletonXmlTree<'a>,
}

impl<'a> AbletonXmlParser<'a> {
    pub fn new() -> AbletonXmlParser<'a> {
        AbletonXmlParser {
            tree: AbletonXmlTree::new(),
        }
    }

    pub fn parse_xml(&mut self, file: File) -> Result<()> {
        let mut reader = EventReader::new(self.parse_to_xml_buffer(file)?);
        let mut tracks_events: Vec<XmlEvent> = vec![];
        let mut push_to_tracks_events = false;
        loop {
            let e = reader.next();
            if e.is_err() {
                println!("no more events, breaking");
                break;
            }
            match e {
                Ok(e) => {
                    println!("event: {:?}", e);
                    if push_to_tracks_events {
                        if let XmlEvent::EndElement { name, .. } = e.clone() {
                            if name.local_name == "Tracks" {
                                push_to_tracks_events = false;
                            }
                        }
                        tracks_events.push(e);
                    } else {
                        match e.clone() {
                            XmlEvent::StartElement { name, .. } => {
                                if name.local_name == "Tracks" {
                                    push_to_tracks_events = true;
                                    tracks_events.push(e);
                                } else {
                                    self.parse_xml_chunk(&e);
                                }
                            }
                            XmlEvent::EndDocument => {
                                println!("end of document");
                                break;
                            }
                            _ => self.parse_xml_chunk(&e),
                        }
                    }
                }
                Err(_) => break,
            }
            println!("current depth: {}", self.tree.current_depth);
        }
        Ok(())
    }

    fn parse_to_xml_buffer(&self, file: File) -> Result<BufReader<GzDecoder<File>>> {
        let decoded = GzDecoder::new(file);
        let buff_daddy = BufReader::new(decoded);
        Ok(buff_daddy)
    }

    fn parse_xml_chunk(&mut self, chunk: &XmlEvent) {
        let output = ParserOutput::from(chunk);
        self.tree.create_node(output);
    }
}

#[derive(Debug)]
struct AbletonXmlTree<'a> {
    current_depth: u32,
    max_depth: u32,
    open_indexes: Vec<u32>,
    nodes: Vec<AbletonXmlTreeNode<'a>>,
    last_node: Option<&'a AbletonXmlTreeNode<'a>>,
}

impl<'a> AbletonXmlTree<'a> {
    fn new() -> AbletonXmlTree<'a> {
        AbletonXmlTree {
            current_depth: 0,
            max_depth: 0,
            open_indexes: vec![],
            nodes: vec![],
            last_node: None,
        }
    }

    fn close_node(&mut self) {
        self.current_depth -= 1;
        let opened = self.open_indexes.pop().expect("open node should exist");
        let node = self
            .nodes
            .get_mut(opened as usize)
            .expect("node should exist");
        node.close();
    }

    fn create_node(&mut self, parser_output: ParserOutput<'a>) {
        let mut parent: Option<&AbletonXmlTreeNode> = None;
        match self.last_node {
            Some(node) => {
                if node.open {
                    parent = Some(node);
                } else if node.parent.is_some() {
                    parent = node.parent;
                }
            }
            None => {}
        }
        let new_node = AbletonXmlTreeNode::new(parser_output, parent, self.nodes.len() as u32);
        match new_node {
            AbletonCreateNodeResponse::AbletonXmlTreeNode(node) => {
                self.nodes.push(node);
                self.open_indexes.push(self.nodes.len() as u32 - 1);
                self.current_depth += 1;
                if self.current_depth > self.max_depth {
                    self.max_depth = self.current_depth;
                }
            }
            AbletonCreateNodeResponse::AbletonXmlTreeCloseEvent => {
                self.close_node();
            }
            AbletonCreateNodeResponse::None => {}
        }
        if self.current_depth > self.max_depth {
            self.max_depth = self.current_depth;
        }
    }
}

#[derive(Debug)]
struct AbletonXmlTreeNode<'a> {
    parser_output: ParserOutput<'a>,
    children: Vec<&'a AbletonXmlTreeNode<'a>>,
    parent: Option<&'a AbletonXmlTreeNode<'a>>,
    open: bool,
    index: u32,
}

enum AbletonCreateNodeResponse<'a> {
    None,
    AbletonXmlTreeNode(AbletonXmlTreeNode<'a>),
    AbletonXmlTreeCloseEvent,
}

impl<'a> AbletonXmlTreeNode<'a> {
    fn new(
        parser_output: ParserOutput<'a>,
        parent: Option<&'a AbletonXmlTreeNode>,
        index: u32,
    ) -> AbletonCreateNodeResponse<'a> {
        match parser_output {
            ParserOutput::Close(_) => return AbletonCreateNodeResponse::AbletonXmlTreeCloseEvent,
            ParserOutput::AbletonEnd(_) => {
                return AbletonCreateNodeResponse::AbletonXmlTreeCloseEvent
            }
            ParserOutput::LiveSetEnd(_) => {
                return AbletonCreateNodeResponse::AbletonXmlTreeCloseEvent
            }
            ParserOutput::TracksEnd(_) => {
                return AbletonCreateNodeResponse::AbletonXmlTreeCloseEvent
            }
            ParserOutput::None => return AbletonCreateNodeResponse::None,
            _ => {}
        };
        AbletonCreateNodeResponse::AbletonXmlTreeNode(AbletonXmlTreeNode {
            parser_output,
            children: vec![],
            parent,
            open: true,
            index,
        })
    }

    fn close(&mut self) {
        self.open = false;
    }
}

#[derive(Debug)]
pub struct OpenNodeStack<'a> {
    stack: LinkedList<&'a AbletonXmlTreeNode<'a>>,
}

impl<'a> OpenNodeStack<'a> {
    fn new() -> OpenNodeStack<'a> {
        OpenNodeStack {
            stack: LinkedList::new(),
        }
    }

    fn push(&mut self, node: &'a AbletonXmlTreeNode<'a>) {
        self.stack.push_front(node);
    }

    fn pop(&mut self) -> Option<&'a AbletonXmlTreeNode<'a>> {
        let val = self.stack.pop_front();
        val
    }
}
