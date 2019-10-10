mod pane;

use std::io::{self};
use pane::{ConnectedPane, ConnectorType};

fn main() -> io::Result<()> {
    let mut pane = ConnectedPane::new(vec![3, 3, 3], 1, ConnectorType::Arrow);

    pane.connect(0, 1);
    pane.connect(0, 2);
    pane.connect(2, 0);

    let pane = pane.pane();

    println!("{}", pane.to_string());
    println!("111 222 333");

    Ok(())
}
