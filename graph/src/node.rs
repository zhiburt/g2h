
#![allow(unused)]
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Node<T> {
    pub data: T,
    pub parent: Option<Box<Node<T>>>,
    pub children: Option<Vec<Box<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(s: T) -> Self {
        Node{
            data: s,
            parent: None,
            children: None,
        }
    }
    
    pub fn push(&mut self, n: Node<T>) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }
        
        // where here move?
        self.children.as_mut().unwrap().push(Box::new(n));
    }
    
    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
    
    pub fn walk<F>(&self, f: &mut F) where F: FnMut(&Node<T>) {
        match &self.children {
            Some(children) => {
                for child in children {
                    child.walk(f);
                }
            },
            _ => (),
        }
        
        f(self);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tree<T> {
    pub root: Node<T>,
    pub deep: i64,
    pub count: i64,
}

impl<T> Tree<T> {
    pub fn new(n: Node<T>) -> Self {
        let (count, deep) = Tree::go(&n);
        Tree{
            root: n,
            deep,
            count,
        }
    }
    
    pub fn go(n: &Node<T>) -> (i64, i64) {
        let (mut count, mut deep) = (1, 1);
        if let Some(children) = &n.children {
            let mut deep_leafs = Vec::with_capacity(children.len());
            for child in children {
                let (lc, ld) = Tree::go(&child);
                count += lc;
                deep_leafs.push(ld);
            }
            deep += deep_leafs.iter().max().unwrap();
        }
        
        (count, deep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leaf() {
        let mut root = Node::new("example".to_string());
        
        assert!(root.is_leaf());
        
        root.push(Node::new("e".to_string()));
        assert_eq!(root.is_leaf(), false);
    }

    #[test]
    fn test_deep() {
        let tree = example();
        
        assert_eq!(tree.deep, 4);
    }

    #[test]
    fn test_deep_in_root_tree() {
        let tree = Tree::new(Node::new("example".to_string()));
        
        assert_eq!(tree.deep, 1);
    }

    #[test]
    fn test_count() {
        let tree = example();
        
        assert_eq!(tree.count, 6);
    }
    
    #[test]
    fn test_count_in_one_node_tree() {
        let tree = Tree::new(Node::new("example".to_string()));
        
        assert_eq!(tree.count, 1);
    }
    
    fn example() -> Tree {
        Tree::new(Node{
            data: "0".to_string(),
            parent: None,
            children: Some(vec![
                Box::new(Node::new("1".to_string())),
                Box::new(Node{
                    data: "1".to_string(),
                    parent: None,
                    children: Some(vec![
                        Box::new(Node{
                            data: "2".to_string(),
                            parent: None,
                            children: Some(vec![
                                Box::new(Node::new("3".to_string())),
                                Box::new(Node::new("3".to_string())),
                            ]),
                        }),
                    ]),
                }),
            ]),
        })
    }
}
