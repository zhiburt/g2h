use crate::pane::{MatrixPane};
use graph::algorithm::{self};

pub fn construct_path(mut matrix: MatrixPane, from: usize, look: usize, path_symbol: &str, checked_symbol: &str) -> MatrixPane {
    let (dst_x, dst_y) = matrix.get_xy(look);
    let path = algorithm::a_star(matrix.graph(), from, look, |n| {
        let (x, y) = matrix.get_xy(n);
        f64::abs(dst_x as f64 - x  as f64) as usize + f64::abs(dst_y as f64 - y  as f64) as usize
    });

    for p in path.as_ref().unwrap() {
        matrix.get_node(*p.0).unwrap().borrow_mut().data = checked_symbol.to_owned();
    }

    for point in &algorithm::path(&path.unwrap(), look, from).unwrap() {
        matrix.get_node(*point).unwrap().borrow_mut().data = path_symbol.to_owned();
    }
    
    matrix
}