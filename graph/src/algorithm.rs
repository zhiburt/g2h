use crate::node::{Graph, Node};
use std::collections::{BTreeMap, BTreeSet};

pub fn dijkstra<T: Eq + Ord>(gh: &Graph<T>, source: usize, look: usize) -> Option<BTreeMap<usize, usize>> {
    let (_, path) = dijkstra_extra(gh, source, look);
    path
}

pub fn dijkstra_extra<T: Eq + Ord>(gh: &Graph<T>, source: usize, look: usize) -> (Vec<Vec<usize>>, Option<BTreeMap<usize,usize>>) {
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

            rev.entry(check_node.index_in).or_insert(u);

            dist.entry(check_node.index_in).and_modify(|old_weight| {
                if weight_from_source < *old_weight {
                    *old_weight = weight_from_source;

                    *rev.get_mut(&check_node.index_in).unwrap() = u;
                }
            }).or_insert(weight_from_source);

            unchecked.insert(check_node.index_in);

            iteration_info.last_mut().unwrap().push(check_node.index_in);
        }

        checked.insert(u);
    }

    (iteration_info, Some(rev))
}

pub fn a_star<T: Eq + Ord, H: Fn(usize) -> usize>(gh: &Graph<T>, start: usize, goal: usize, h: H) -> Option<BTreeMap<usize, usize>> {
    let d = | node: &Node<T>, i: usize | -> usize {
        node.edges.as_ref().unwrap()[i].weight
    };

    let mut open_set = BTreeSet::new();
    open_set.insert(start);

    let mut came_from = BTreeMap::new();
    
    let infinity_value = usize::max_value() / 2;
    let mut g_score = BTreeMap::new();
    for i in 0 .. gh.area.len() {
        g_score.insert(i, infinity_value);
    }
    g_score.entry(start).and_modify(|s| *s = 0);

    let mut f_score = BTreeMap::new();
    for i in 0 .. gh.area.len() {
        f_score.insert(i, infinity_value);
    }
    f_score.entry(start).and_modify(|s| *s = h(start));

    while !open_set.is_empty() {
        let max_in_f_score = f_score.iter().map(|s| (*s.0, *s.1)).max_by(|lhs, rhs| lhs.1.cmp(&rhs.1)).unwrap();
        let mut min = max_in_f_score.1;
        let mut current = max_in_f_score.0;
        for open in &open_set {
            if f_score[open] <= min {
                min = f_score[open];
                current = *open;
            }
        }

        if current == goal {
            return Some(came_from);
        }

        open_set.remove(&current);
        let node = gh.node_by_index(current).unwrap();
        let node = node.borrow();
        let edges = match node.edges.as_ref() {
            Some(e) => e,
            None => continue,
        };

        for (i, neighbor) in edges.iter().enumerate() {
            let tentative_g_score =  g_score[&current] + d(&neighbor.from.borrow(), i);
            let neighbor_index = neighbor.to.borrow().index_in;
            if tentative_g_score < g_score[&neighbor_index] {
                came_from.entry(neighbor_index).and_modify(|n| *n = current).or_insert(current);
                g_score.entry(neighbor_index).and_modify(|s| *s = tentative_g_score);
                f_score.entry(neighbor_index).and_modify(|s| *s = tentative_g_score + h(neighbor_index));
                
                if !open_set.contains(&neighbor_index) {
                    open_set.insert(neighbor_index);
                }
            }
        }
    }

    None
}

pub fn path(area: &BTreeMap<usize, usize>, from: usize, to: usize) -> Option<Vec<usize>> {
    let mut i = area.get(&from);
    if i.is_none() {
        return None;
    }

    let mut path = vec![from];
    while i.is_some() {
        let point = i.unwrap();
        path.push(*point);
        if *point == to {
            break;
        }
        i = area.get(&point);
    }
            
    Some(path)
}

pub fn color_gh<T: Eq + Ord>(gh: &Graph<T>) -> BTreeMap<usize, usize> {
    let mut vertecs_colors = BTreeMap::new();
    let index = 0;
    mark_node(gh, index, &mut vertecs_colors);

    vertecs_colors
}

fn mark_node<T: Eq + Ord>(gh: &Graph<T>, i: usize, mut colors: &mut BTreeMap<usize, usize>) {
    let node = gh.node_by_index(i).unwrap();
    let node = node.borrow();
    let edges = match node.edges.as_ref() {
        Some(e) => e,
        None => {
            colors.insert(i, 0);
            return;
        },
    };

    let mut neighbors = Vec::new();
    for neighbor in edges.iter() {
        neighbors.push(neighbor.to.borrow().index_in);
    }

    for neighbor in neighbors.iter() {
        if !colors.contains_key(neighbor) {
            mark_node(gh, *neighbor, &mut colors);
        }
    }

    let mut neighbors_colors = Vec::new();
    for neighbor in neighbors.iter() {
        neighbors_colors.push(colors[neighbor]);
    }

    let mut unused_color = *neighbors_colors.iter().max().unwrap() as isize;
    let mut min_color = 0;
    for color in 0 .. unused_color {
        if !neighbors_colors.contains(&min_color) {
            unused_color = min_color as isize - 1;
            break;
        }
        min_color += 1;
    }

    colors.insert(i, (unused_color + 1) as usize);
}
