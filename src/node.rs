use crate::token::{Token, Tokens};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Node {
    And(Vec<Node>),
    Or(Vec<Node>),
    Not(Box<Node>),
    Tag(usize, usize),
    Value(usize),
    None,
}

impl Node {
    fn _fmt(&self, f: &mut fmt::Formatter<'_>, values: &Vec<String>) -> fmt::Result {
        match self {
            Node::And(nodes) => {
                write!(f, " AND(")?;
                for (i, node) in nodes.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    node._fmt(f, values)?;
                }
                write!(f, ") ")?;
            }
            Node::Or(nodes) => {
                write!(f, " OR(")?;
                for (i, node) in nodes.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    node._fmt(f, values)?;
                }
                write!(f, ") ")?;
            }
            Node::Not(node) => {
                write!(f, " NOT(")?;
                node._fmt(f, values)?;
                write!(f, ") ")?;
            }
            Node::Tag(category, value) => {
                write!(f, " {}:{} ", values[*category], values[*value])?;
            }
            Node::Value(i) => {
                write!(f, " {} ", values[*i])?;
            }
            Node::None => (),
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Nodes {
    pub node: Node,
    pub values: Vec<String>,
}

impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.node._fmt(f, &self.values)
    }
}

impl Node {
    fn not(self) -> Self {
        match self {
            Node::Not(n) => *n,
            _ => Node::Not(Box::new(self)),
        }
    }
    fn to_tags(&mut self, mut names: Self) -> Result<(), &'static str> {
        match self {
            Node::And(nodes) => {
                for node in nodes {
                    node.to_tags(names.clone())?;
                }
            }
            Node::Or(nodes) => {
                for node in nodes {
                    node.to_tags(names.clone())?;
                }
            }
            Node::Not(node) => {
                node.to_tags(names)?;
            }
            Node::Tag(_, _) => return Err("invalid tag"),
            Node::Value(i) => {
                names.to_tag(*i)?;
                *self = names;
            }
            Node::None => (),
        }
        Ok(())
    }
    fn to_tag(&mut self, category: usize) -> Result<(), &'static str> {
        match self {
            Node::And(nodes) => {
                for node in nodes {
                    node.to_tag(category)?;
                }
            }
            Node::Or(nodes) => {
                for node in nodes {
                    node.to_tag(category)?;
                }
            }
            Node::Not(node) => {
                node.to_tag(category)?;
            }
            Node::Tag(_, _) => return Err("invalid tag"),
            Node::Value(i) => {
                *self = Node::Tag(category, *i);
            }
            Node::None => (),
        }
        Ok(())
    }
    fn fix(&mut self) -> bool{
        true
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    And,
    Or,
    Tag,
}

fn custom_filter<T: PartialEq, U>(v: &mut Vec<T>, u: &mut Vec<U>, item: T) {
    if v.len() + 1 != u.len() {
        panic!("v.len() + 1 != u.len()");
    }
    let mut i = 0;
    let len = v.len();
    for j in 0..len {
        if j != i {
            unsafe {
                std::ptr::write(&mut v[i], std::ptr::read(&v[j]));
                std::ptr::write(&mut u[i], std::ptr::read(&u[j]));
            }
        }
        if v[j] != item {
            i += 1;
        }
    }
    unsafe {
        if i != len {
            std::ptr::write(&mut u[i], std::ptr::read(&u[len]));
        }
        v.set_len(i);
        u.set_len(i + 1);
    }
}

fn replace<T>(v: &mut Vec<T>, index: usize, replacement: T) -> T {
    if let Some(t) = v.get_mut(index) {
        std::mem::replace(t, replacement)
    } else {
        panic!("index {} out of bounds: {}", index, v.len());
    }
}

impl Tokens {
    pub fn parse(self) -> Result<Nodes, &'static str> {
        let Tokens { tokens, values } = self;
        let node = Self::_parse(&tokens)?;
        Ok(Nodes { node, values })
    }
    fn _parse(tokens: &[Token]) -> Result<Node, &'static str> {
        if tokens.is_empty() {
            return Ok(Node::None);
        }
        let mut nest = 0;
        let mut operators = Vec::new();
        let mut nodes = Vec::new();
        let mut node = None;
        let mut start = tokens.len() - 1;
        for (i, token) in tokens.iter().enumerate().rev() {
            let operator = match token {
                Token::CloseParen => {
                    if nest == 0 {
                        start = i;
                    }
                    nest += 1;
                    continue;
                }
                Token::OpenParen => {
                    nest -= 1;
                    if nest < 0 {
                        return Err("unmatched parenthesis");
                    } else if nest == 0 {
                        if let Some(node) = node {
                            nodes.push(node);
                            operators.push(Operator::And);
                        }
                        node = Some(Self::_parse(&tokens[(i + 1)..start])?);
                    }
                    continue;
                }
                _ if nest > 0 => continue,
                Token::Value(i) => {
                    if let Some(node) = node {
                        nodes.push(node);
                        operators.push(Operator::And);
                    }
                    node = Some(Node::Value(*i));
                    continue;
                }
                Token::Not => {
                    if let None = node {
                        return Err("invalid not");
                    }
                    node = node.map(|n| n.not());
                    continue;
                }
                Token::Split => Operator::Tag,
                Token::And => Operator::And,
                Token::Or => Operator::Or,
            };
            if let Some(node) = node {
                nodes.push(node);
                operators.push(operator);
            } else {
                return Err("invalid operator");
            }
            node = None;
        }
        if nest != 0 {
            return Err("unmatched parenthesis");
        }
        if let Some(node) = node {
            nodes.push(node);
        } else {
            return Err("invalid operator");
        }
        for (i, op) in operators.iter().enumerate() {
            if &Operator::Tag == op {
                let names = replace(&mut nodes, i, Node::None);
                nodes[i + 1].to_tags(names)?;
            }
        }
        custom_filter(&mut operators, &mut nodes, Operator::Tag);
        let mut a_nodes: Option<Vec<Node>> = None;
        for (i, op) in operators.iter().enumerate() {
            if &Operator::And == op {
                if let Some(a_nodes) = a_nodes.as_mut() {
                    a_nodes.push(replace(&mut nodes, i, Node::None));
                } else {
                    a_nodes = Some(vec![replace(&mut nodes, i, Node::None)]);
                }
            } else {
                a_nodes
                    .as_mut()
                    .map(|a_n| a_n.push(replace(&mut nodes, i, Node::None)));
                if let Some(a_n) = a_nodes {
                    nodes[i] = Node::And(a_n);
                    a_nodes = None;
                }
            }
        }
        a_nodes
            .as_mut()
            .map(|a_n| a_n.push(replace(&mut nodes, operators.len(), Node::None)));
        if let Some(a_n) = a_nodes {
            nodes[operators.len()] = Node::And(a_n);
        }
        custom_filter(&mut operators, &mut nodes, Operator::And);
        let mut o_nodes: Option<Vec<Node>> = None;
        for (i, op) in operators.iter().enumerate() {
            if &Operator::Or == op {
                if let Some(o_nodes) = o_nodes.as_mut() {
                    o_nodes.push(replace(&mut nodes, i, Node::None));
                } else {
                    o_nodes = Some(vec![replace(&mut nodes, i, Node::None)]);
                }
            } else {
                o_nodes
                    .as_mut()
                    .map(|o_n| o_n.push(replace(&mut nodes, i, Node::None)));
                if let Some(o_n) = o_nodes {
                    nodes[i] = Node::Or(o_n);
                    o_nodes = None;
                }
            }
        }
        o_nodes
            .as_mut()
            .map(|o_n| o_n.push(replace(&mut nodes, operators.len(), Node::None)));
        if let Some(o_n) = o_nodes {
            nodes[operators.len()] = Node::Or(o_n);
        }
        custom_filter(&mut operators, &mut nodes, Operator::Or);
        return Ok(nodes.pop().unwrap());
    }
}
