use crate::pane::{MatrixPane, Surface};
use graph::algorithm::{self};

pub fn frames(matrix: &mut MatrixPane, from: usize, look: usize, path_symbol: &str, checked_symbol: &str) -> Vec<String> {
    let (steps, path) = algorithm::dijkstra_extra(matrix.graph(), from, look);
    let mut frames = Vec::new();

    for step_info in steps {
        for step in step_info {
            matrix.get_node(step).unwrap().borrow_mut().data = checked_symbol.to_owned();
        }

        frames.push(matrix.pane().to_string());
    }

    for point in &algorithm::path(&path.unwrap(), look).unwrap() {
        matrix.get_node(*point).unwrap().borrow_mut().data = path_symbol.to_owned();

        frames.push(matrix.pane().to_string());
    }

    
    frames
}