use crate::pane::{MatrixPane};
use graph::algorithm::{self};

pub fn construct_path(mut matrix: MatrixPane, from: usize, look: usize, path_symbol: &str, checked_symbol: &str) -> MatrixPane {
    let path = algorithm::dijkstra(&matrix.gh, from, look);
        
    for p in path.as_ref().unwrap() {
        matrix.gh.node_by_index(*p.0).unwrap().borrow_mut().data = checked_symbol.to_owned();
    }

    for point in &algorithm::path(&path.unwrap(), look).unwrap() {
        matrix.gh.node_by_index(*point).unwrap().borrow_mut().data = path_symbol.to_owned();
    }
    
    matrix
}