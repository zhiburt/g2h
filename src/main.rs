use std::io::{self};
use std::collections::BTreeMap;

fn main() -> io::Result<()> {
    let mut pane = ConnectedPane::new(vec![3, 3, 3], 1, ConnectorType::Arrow);

    pane.connect(0, 1);
    pane.connect(0, 2);
    pane.connect(2, 0);

    let pane = pane.pane();
    
    let pane_in_string = pane
        .surface
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", pane_in_string);
    println!("111 222 333");

    Ok(())
}

struct ConnectedPane {
    connected_list: Vec<(usize, usize)>,
    concept: Vec<usize>,
    space: usize,
    connector: char,
}

enum ConnectorType {
    General,
    Arrow,
}

impl ConnectedPane {
    fn new(concept: Vec<usize>, space: usize, connection: ConnectorType) -> Self {
        let connector = match connection {
            ConnectorType::General => '|',
            ConnectorType::Arrow => 'v',
        };
        
        ConnectedPane {
            connected_list: Vec::new(),
            connector,
            concept,
            space,
        }
    }

    fn connect(&mut self, who: usize, to: usize) {
        self.connected_list.push((who, to));
        self.connected_list.sort();
    }

    fn pane(&self) -> Pane {
        let width = self.concept.iter().sum::<usize>() + (self.concept.len() - 1) * self.space;
        let hight = self.connected_list.len() * 2;
        let mut pane = Pane::new(width, hight);
        
        struct LineCoordinate {
            from: Point,
            to: Point,
            lhs_connection: Point,
            rhs_connection: Point,
        };

        let mut coordinates: Vec<LineCoordinate> = Vec::new();
        let mut already_used_connector: BTreeMap<usize, usize> = BTreeMap::new();
        let mut current_level = 0;
        for (from, to) in &self.connected_list {
            let from_index = self.start_element_index(*from);
            let to_index = self.start_element_index(*to);

            let from_diff = *already_used_connector.entry(*from).and_modify(|e| *e += 1).or_default();
            let to_diff = *already_used_connector.entry(*to).and_modify(|e| *e += 1).or_default();

            let mut lhs_connection = Point{x: from_index + from_diff, y: current_level + 1};
            let mut rhs_connection = Point{x: to_index + to_diff, y: current_level + 1};
   
            let (from, to) = if lhs_connection.x > rhs_connection.x {
                (Point{x: lhs_connection.x, y: current_level}, Point{x: rhs_connection.x + 1, y: current_level})
            } else {
                (Point{x: lhs_connection.x + 1, y: current_level}, Point{x: rhs_connection.x, y: current_level})
            };

            coordinates.push(LineCoordinate{from, to, lhs_connection, rhs_connection});

            current_level += 2;
        }

        for coordinate in coordinates {
            let lhs_conn_down = Point{x: coordinate.lhs_connection.x, y: hight};
            let rhs_conn_down = Point{x: coordinate.rhs_connection.x, y: hight};
            pane.put(Shape::Line(coordinate.lhs_connection, lhs_conn_down), '|');
            pane.put(Shape::Line(coordinate.rhs_connection, rhs_conn_down), '|');

            pane.put(Shape::Point(Point{x: rhs_conn_down.x, y: rhs_conn_down.y - 1}), self.connector);

            pane.put(Shape::Line(coordinate.from, coordinate.to), '-');
        }
    

        pane
    }

    fn start_element_index(&self, i: usize) -> usize {
        self.concept.iter().take(i).sum::<usize>() + i * self.space
    }
}

#[derive(Debug)]
struct Pane {
    size: (usize, usize),
    surface: Vec<Vec<char>>,
}

impl Pane {
    fn new(width: usize, hight: usize) -> Self {
        Pane {
            size: (width, hight),
            surface: vec![vec![' '; width]; hight],
        }
    }

    fn put(&mut self, shape: Shape, c: char) {
        match shape {
            Shape::Point(Point { x, y }) => {
                self.surface[y][x] = c;
            }
            Shape::Line(point1, point2) => {
                //TODO: simplify
                if point1.y == point2.y {
                    let mut min = std::cmp::min(point1.x, point2.x);
                    let max = std::cmp::max(point1.x, point2.x);

                    while min < max {
                        self.surface[point1.y][min] = c;
                        min += 1;
                    }
                } else if point1.x == point2.x {
                    let mut min = std::cmp::min(point1.y, point2.y);
                    let max = std::cmp::max(point1.y, point2.y);

                    while min < max {
                        self.surface[min][point1.x] = c;
                        min += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Shape {
    Line(Point, Point),
    Point(Point),
}

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
