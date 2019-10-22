use crate::node::{Graph};
use std::collections::{BTreeMap, BTreeSet};

pub fn dijkstra<T: Eq + Ord>(gh: &Graph<T>, source: usize, look: usize) -> Option<BTreeMap<usize,Option<usize>>> {
    let (_, path) = dijkstra_extra(gh, source, look);
    path
}

pub fn dijkstra_extra<T: Eq + Ord>(gh: &Graph<T>, source: usize, look: usize) -> (Vec<Vec<usize>>, Option<BTreeMap<usize,Option<usize>>>) {
    let src = gh.area[&source].borrow();
    if src.edges.is_none(){
        return (Vec::new(), None);
    }

    let mut rev = BTreeMap::new();
    let mut checked = BTreeSet::new();
    let mut unchecked = BTreeSet::new();
    let mut dist = BTreeMap::new();
    unchecked.insert(source);
    dist.insert(source, 0);
    rev.insert(source, None);

    let mut iteration_info = Vec::new();

    while !unchecked.is_empty() {
        let (u, weight) = match dist.iter().filter(|&(e, _)| !checked.contains(e)).min_by(|(_, lw), (_, rw)| lw.cmp(&rw)) {
            Some(d) => (*d.0, *d.1),
            None => break,
        };
        unchecked.remove(&u);

        // what about case where rev.contains(look)?
        // I get it to be insufficient, since it can be not shortest way.
        if u == look {
            return (iteration_info, Some(rev));
        }

        iteration_info.push(vec![u]);

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

            rev.entry(check_node.index_in).or_insert(Some(u));

            dist.entry(check_node.index_in).and_modify(|old_weight| {
                if weight_from_source < *old_weight {
                    *old_weight = weight_from_source;

                    *rev.get_mut(&check_node.index_in).unwrap() = Some(u);
                }
            }).or_insert(weight_from_source);

            unchecked.insert(check_node.index_in);

            iteration_info.last_mut().unwrap().push(check_node.index_in);
        }

        checked.insert(u);
    }

    (iteration_info, Some(rev))
}

pub fn path(area: &BTreeMap<usize,Option<usize>>, from: usize) -> Option<Vec<usize>> {
    let mut i = area.get(&from);
    if i.is_none() {
        return None;
    }

    let mut path = vec![from];
    while let Some(Some(point)) = i {
        path.push(*point);
        i = area.get(&point);
    }
            
    Some(path)
}