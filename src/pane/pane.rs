use std::collections::BTreeMap;
use graph::Graph;

pub struct MatrixPane {
    gh: Graph<String>,
    pub node_list: NodeList<String>,
    c: String,
    size: (usize, usize),
}

type NodeList<T> = Vec<std::rc::Rc<std::cell::RefCell<graph::Node<T>>>>;

impl MatrixPane {
    pub fn new(width: usize, hight: usize, c: &str) -> Self {
        let (gh, node_list) = MatrixPane::create_matrix_graph(width, hight, String::from(c));
        MatrixPane {
            size: (width, hight),
            c: c.to_owned(),
            gh,
            node_list,
        }
    }

    pub fn create_matrix_graph<T: Clone + Eq + Ord>(w: usize, h: usize, d: T) -> (Graph<T>, NodeList<T>) {
        let mut gh = Graph::new();
        let mut node_list = Vec::new();
        (0..w*h).for_each(|_| {node_list.push(gh.add_node(d.clone()));});

        for i in 1..w*h {
            if i % w == 0 {
                continue;
            }
            Graph::link(node_list[i-1].clone(), node_list[i].clone(), 10);
            Graph::link(node_list[i].clone(), node_list[i-1].clone(), 10);
        }

        for i in 0..(w*h-w) {
            Graph::link(node_list[i].clone(), node_list[i+w].clone(), 10);
            Graph::link(node_list[i+w].clone(), node_list[i].clone(), 10);
        }

        (gh, node_list)
    }

    pub fn orig_pane(&self) -> Pane {
        let mut lines = Vec::new();
        for i in 0..self.size.1 {
            let s = vec![self.c.clone(); self.size.0];
            let line = s.join(" ");
            lines.push(StrPane::new(&line).pane());
        }

        ColumnFittablePane::new(lines).pane()
    }

    pub fn get_xy(&self, n: usize) -> (usize, usize) {
        let (w, h) = self.size;
        let y = n / w;
        let x = n % w;
        (x, y)
    }

    pub fn clean(&mut self) {
        let c = self.c.clone();
        self.gh.for_each(|mut n| n.data = c.clone())
    }

    pub fn get_node(&mut self, index: usize) -> Option<std::rc::Rc<std::cell::RefCell<graph::Node<String>>>> {
        self.gh.node_by_index(index)
    }

    pub fn graph(&self) -> &Graph<String> {
        &self.gh
    }

    pub fn structure(&self) -> Pane {
        let mut lines = Vec::new();
        for (i, node) in self.node_list.iter().enumerate() {
            let node = node.borrow();
            let mut weights = Vec::new();
            if let Some(edges) = &node.edges {
                for c in edges {
                    weights.push(format!("{}", c.weight))
                }
            }

            let line = format!("{} | {}", i, weights.join(" "));
            lines.push(StrPane::new(&line).pane())
        }

        ColumnFittablePane::new(lines).pane()
    }
}

impl Surface for MatrixPane {
    fn size(&self) -> (usize, usize) {
        return (0,0)
    }

    fn pane(&self) -> Pane {
        let mut lines = Vec::new();
        for chunk in self.node_list.chunks(self.size.0) {
            let s = chunk.iter().map(|n| n.borrow().data.clone()).collect::<Vec<String>>();
            let line = s.join(" ");
            lines.push(StrPane::new(&line).pane());
        }

        ColumnFittablePane::new(lines).pane()
    }
}

pub struct ColumnFittablePane {
    panes: Vec<Pane>,
}

impl ColumnFittablePane {
    pub fn new(panes: Vec<Pane>) -> Self {
        ColumnFittablePane {
            panes,
        }
    }
}

impl Surface for ColumnFittablePane {
    fn size(&self) -> (usize, usize) {
        let sizes: Vec<(usize, usize)> = self.panes.iter().map(|l| l.size()).collect();

        let max_width = sizes.iter().map(|(w, _)| w).max().map_or(0, |w| *w);
        let hight = sizes.iter().map(|(_, h)| *h).sum(); 

        (max_width, hight)
    }

    fn pane(&self) -> Pane {
        let size = self.size();
        let mut pane = Pane::new(size.0, size.1);

        let mut i = 0;
        for p in &self.panes {
            let str_pane = p.to_string();
            for line in str_pane.lines() {
                StrPane::str_pane(&mut pane, line, i);
                i += 1;
            }
        }

        pane
    }
}

#[derive(Debug)] 
pub struct StrPane<'a> {
    line: &'a str,
}

impl<'a> StrPane<'a> {
    pub fn new(s: &'a str) -> Self {
        StrPane {
            line: s,
        }
    }

    fn str_pane(pane: &mut Pane, s: &str, row: usize) {
        for (i, c) in s.chars().enumerate() {
            pane.put(Shape::Point(Point::new(i, row)), c);
        }
    }
}

impl<'a> Surface for StrPane<'a> {
    fn size(&self) -> (usize, usize) {
        (self.line.chars().count(), 1)
    }

    fn pane(&self) -> Pane {
        let size = self.size();
        let mut pane = Pane::new(size.0, size.1);

        StrPane::str_pane(&mut pane, self.line, 0);

        pane
    }
}

pub struct ConnectedPane {
    connected_list: Vec<(usize, usize)>,
    concept: Vec<usize>,
    settings: PaneSettings,
}

#[derive(Debug, Clone)]
pub struct PaneSettings {
    pub gap_size: usize,
    pub connection_size: usize,
    pub connection_type: ConnectorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorType {
    General,
    Arrow,
}

impl ConnectedPane {
    pub fn new(concept: &[usize], settings: PaneSettings) -> Self {
        ConnectedPane {
            connected_list: Vec::new(),
            concept: concept.to_owned(),
            settings,
        }
    }

    pub fn connect(&mut self, who: usize, to: usize) {
        self.connected_list.push((who, to));
        self.connected_list.sort();
    }

    fn start_element_index(&self, i: usize) -> usize {
        self.concept.iter().take(i).sum::<usize>() + i * self.settings.gap_size
    }

    fn connector(ct: ConnectorType) -> char {
        match ct {
            ConnectorType::General => '|',
            ConnectorType::Arrow => 'v',
        }
    }
}

impl Surface for ConnectedPane {
    fn size(&self) -> (usize, usize) {
        let width =
            self.concept.iter().sum::<usize>() + (self.concept.len() - 1) * self.settings.gap_size;
        let hight = self.connected_list.len() * 2;

        (width, hight)
    }

    fn pane(&self) -> Pane {
        let (width, hight) = self.size();
        let mut pane = Pane::new(width, hight);
        struct LineCoordinate {
            from: Point,
            to: Point,
            lhs_connection: Point,
            rhs_connection: Point,
        };

        let mut coordinates: Vec<LineCoordinate> = Vec::new();
        let mut used: BTreeMap<usize, usize> = BTreeMap::new();
        let mut current_level = 0;
        for (from, to) in &self.connected_list {
            let from_index = self.start_element_index(*from);
            let to_index = self.start_element_index(*to);

            let from_diff = *used.entry(*from).and_modify(|e| *e += self.settings.connection_size).or_default();
            let to_diff = *used.entry(*to).and_modify(|e| *e += self.settings.connection_size).or_default();

            let lhs_connection = Point::new(from_index + from_diff, current_level + 1);
            let rhs_connection = Point::new(to_index + to_diff, current_level + 1);
            let (from, to) = if lhs_connection.x > rhs_connection.x {
                (Point::new(lhs_connection.x, current_level), Point::new(rhs_connection.x + 1, current_level))
            } else {
                (Point::new(lhs_connection.x + 1, current_level), Point::new(rhs_connection.x, current_level))
            };

            coordinates.push(LineCoordinate {from, to, lhs_connection, rhs_connection});

            current_level += 2;
        }

        let connector = ConnectedPane::connector(self.settings.connection_type);
        for coordinate in coordinates {
            let lhs_conn_down = Point::new(coordinate.lhs_connection.x, hight);
            let rhs_conn_down = Point::new(coordinate.rhs_connection.x, hight);
            pane.put(Shape::Line(coordinate.lhs_connection, lhs_conn_down), '|');
            pane.put(Shape::Line(coordinate.rhs_connection, rhs_conn_down), '|');

            pane.put(Shape::Point(Point::new(rhs_conn_down.x, rhs_conn_down.y - 1)), connector);

            pane.put(Shape::Line(coordinate.from, coordinate.to), '-');
        }

        pane
    }
}

pub trait Surface {
    fn size(&self) -> (usize, usize);
    fn pane(&self) -> Pane;
}

#[derive(Debug)]
pub struct Pane {
    size: (usize, usize),
    surface: Vec<Vec<char>>,
}

impl Pane {
    pub fn new(width: usize, hight: usize) -> Self {
        Pane {
            size: (width, hight),
            surface: vec![vec![' '; width]; hight],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn put(&mut self, shape: Shape, c: char) {
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

impl std::fmt::Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self
            .surface
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", lines)
    }
}

#[derive(Debug)]
pub enum Shape {
    Line(Point, Point),
    Point(Point),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
