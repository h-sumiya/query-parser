use crate::token::{Token, Tokens};

#[derive(Debug)]
pub enum TagNode {
    And(Vec<TagNode>),
    Or(Vec<TagNode>),
    Not(Box<TagNode>),
    Value(String),
}

#[derive(Debug)]
pub enum Node {
    And(Vec<Node>),
    Or(Vec<Node>),
    Not(Box<Node>),
    Value(String),
    Tag(TagNode, TagNode),
}