use crate::node::{Node};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;

pub fn dijkstra<T: Eq + Ord>(source_node: Rc<RefCell<Node<T>>>, look: Rc<RefCell<Node<T>>>) {
    let source = source_node.borrow();
    if source.edges.is_none(){
        return;
    }

    let mut checked = BTreeSet::new();
    let mut unchecked = BTreeSet::new();
    let mut dist = BTreeMap::new();
    unchecked.insert(source_node.clone());
    dist.insert(source_node.clone(), 0);

    while !unchecked.is_empty() {
        let (u, weight) = match dist.iter().map(|(u, w)| (u.clone(), *w)).filter(|(e, _)| !checked.contains(e)).min_by(|l, r| l.cmp(&r)) {
            Some(d) => d,
            None => break,
        };
        let uu = u.clone();
        let uu = uu.borrow();
        let edges = match uu.edges.as_ref() {
            Some(e) => e,
            None => break,
        };
        for child in edges {
            
            let check_node = &child.to;
            let weight_from_source = child.weight + weight;

            dist.entry(check_node.clone()).and_modify(|old_weight| {
                if weight_from_source < *old_weight {
                    *old_weight = weight_from_source;
                }
            }).or_insert(weight_from_source);

            unchecked.insert(check_node.clone());

            println!("{:?}", dist.values());
        }

        unchecked.remove(&u);
        checked.insert(u.clone());
    }
}
