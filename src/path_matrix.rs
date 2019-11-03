use crate::pane::{MatrixPane};
use graph::algorithm::{self};

pub fn construct_path(mut matrix: MatrixPane, from: usize, look: usize, path_symbol: &str, checked_symbol: &str) -> MatrixPane {
    let path = algorithm::dijkstra(matrix.graph(), from, look);
        
    for p in path.as_ref().unwrap() {
        matrix.get_node(*p.0).unwrap().borrow_mut().data = checked_symbol.to_owned();
    }

    for point in &algorithm::path(&path.unwrap(), look, from).unwrap() {
        matrix.get_node(*point).unwrap().borrow_mut().data = path_symbol.to_owned();
    }
    
    matrix
}