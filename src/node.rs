use crate::token::{self, Token, Tokens};

#[derive(Debug)]
pub enum TagNode {
    And(Vec<TagNode>),
    Or(Vec<TagNode>),
    Not(Box<TagNode>),
    Value(usize),
}

#[derive(Debug)]
pub enum Node {
    And(Vec<Node>),
    Or(Vec<Node>),
    Not(Box<Node>),
    Value(usize),
    Tag(TagNode, TagNode),
    None,
}

enum TempNode {
    And(Node),
    Or(Node),
}

enum Operator {
    And,
    Or,
    None,
}

impl Operator {
    fn new() -> Self {
        Self::None
    }
    fn and(&mut self) -> Result<(), &'static str> {
        if let Self::None = self {
            *self = Self::And;
            return Ok(());
        }
        Err("operator error")
    }
    fn or(&mut self) -> Result<(), &'static str> {
        if let Self::None = self {
            *self = Self::Or;
            return Ok(());
        }
        Err("operator error")
    }
    fn get(&self, node: Node) -> TempNode {
        if let Self::Or = self {
            TempNode::Or(node)
        } else {
            TempNode::And(node)
        }
    }
    fn reset(&mut self) {
        *self = Self::None;
    }
}

impl Node {
    fn new(tokens: &[Token]) -> Result<Self, &'static str> {
        let mut temp_nodes = Vec::new();
        let mut node = Node::None;
        let mut operator = Operator::new();
        let mut end = tokens.len() - 1;
        let mut deep = 0;
        let mut tag = false;
        for (i, token) in tokens.iter().enumerate().rev() {
            let val = match token {
                Token::CloseParen => {
                    deep += 1;
                    if deep == 1 {
                        end = i;
                    }
                    continue;
                }
                Token::OpenParen => {
                    deep -= 1;
                    if deep == 0 {
                        Self::new(&tokens[i + 1..end])?
                    } else {
                        continue;
                    }
                }
                _ if deep > 0 => continue,
                _ if deep < 0 => return Err("parentheses error"),
                Token::And => {
                    let node = Self::new(&tokens[i + 1..end])?;
                    if let Some(node) = node {
                        temp_nodes.push(TempNode::And(node));
                    }
                    continue;
                }
                _ => continue,
            };
        }
        Err("write now")
    }
    fn not(self) -> Self {
        if let Self::Not(node) = self {
            return *node;
        }
        Node::Not(Box::new(self))
    }
}

pub struct NodeObject {
    pub node: Option<Node>,
    pub values: Vec<String>,
}

impl From<Tokens> for NodeObject {
    fn from(value: Tokens) -> Self {
        let Tokens { tokens, values } = value;
        let mut nodes = NodeObject {
            node: Node::new(&tokens),
            values,
        };
        nodes
    }
}
