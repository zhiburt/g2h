use crate::node::{Graph};
use std::collections::{BTreeMap, BTreeSet};

pub fn dijkstra<T: Eq + Ord>(gh: &Graph<T>, source: usize, look: usize) -> Option<BTreeMap<usize,usize>> {
    let src = gh.area[&source].borrow();
    if src.edges.is_none(){
        return None;
    }

    let mut rev = BTreeMap::new();
    let mut checked = BTreeSet::new();
    let mut unchecked = BTreeSet::new();
    let mut dist = BTreeMap::new();
    unchecked.insert(source);
    dist.insert(source, 0);

    while !unchecked.is_empty() {
        let (u, weight) = match dist.iter().filter(|&(e, _)| !checked.contains(e)).min_by(|(_, lw), (_, rw)| lw.cmp(&rw)) {
            Some(d) => (*d.0, *d.1),
            None => break,
        };
        unchecked.remove(&u);

        let u_node = gh.node_by_index(u).unwrap();
        let u_node = u_node.borrow();
        let edges = match u_node.edges.as_ref() {
            Some(e) => e,
            None => {
                checked.insert(u);
                continue;
            }
        };

        for child in edges {
            
            let check_node = &child.to.borrow();
            let weight_from_source = child.weight + weight;

            dist.entry(check_node.index_in).and_modify(|old_weight| {
                if weight_from_source < *old_weight {
                    *old_weight = weight_from_source;
                }
            }).or_insert(weight_from_source);

            unchecked.insert(check_node.index_in);

            if !checked.contains(&check_node.index_in) {
                rev.entry(check_node.index_in).and_modify(|old| {
                    *old = u
                }).or_insert(u);
            }
        }

        checked.insert(u);
    }

    return Some(rev)
}
