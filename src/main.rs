use std::collections::BTreeMap;

fn main() {
    let mut linear_graph = LineGH::new(&[])
    .connect(1, 3)
    .connect(0, 3)
    .connect(0, 2)
    .connect(1, 6)
    .connect(2, 7)
    .connect(0, 1);

    linear_graph.add_edge("edge: &'a str");
    linear_graph.add_edge("maxim");
    linear_graph.add_edge("at work");
    linear_graph.add_edge("should");
    linear_graph.add_edge("I");
    linear_graph.add_edge("be");
    linear_graph.add_edge("left");
    linear_graph.add_edge("!?!?");

    println!("{}", linear_graph);
}

#[derive(Debug)]
struct LineGH {
    // might use here real graph?
    vertices: BTreeMap<usize, Vec<usize>>,
    edges: Vec<String>,
}

impl LineGH {
    pub fn new() -> Self {
        LineGH {
            edges: Vec::new(),
            vertices: BTreeMap::new(),
        }
    }

    pub fn add_edge(&mut self, edge: &str) -> usize {
        self.edges.push(String::from(edge));
        
        self.edges.len() - 1
    }

    pub fn connect(&mut self, e1: usize, e2: usize) {
        self.vertices.entry(e1).or_insert_with(Vec::new).push(e2);
    }

    pub fn count_by(&self, i: usize) -> usize {
        match self.vertices.get(&i) {
            Some(connected_edges) => {
                connected_edges.len()
                    + self
                        .vertices
                        .values()
                        .fold(0, |acc, ver| acc + ver.iter().filter(|&&v| v == i).count())
            }
            None => 0,
        }
    }
}

impl std::fmt::Display for LineGH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: logic with boxes should be refactored
        let boxes = self
            .edges
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let count_connected = self.count_by(i);
                let single_box = FormatBox::new(s, 1);
                if count_connected > single_box.line_lenght() {
                    FormatBox::new(s, count_connected - s.len())
                } else {
                    single_box
                }
            })
            .collect::<Vec<FormatBox>>();
        let size_edge_space = 1;
        let max_space = size_edge_space * (self.edges.len() - 1);
        let len_line = box_len_line(&boxes) + max_space;
        let mut connected_index: BTreeMap<usize, usize> = BTreeMap::new();
        let mut draw_times: BTreeMap<usize, usize> = BTreeMap::new();
        for (node, friends) in &self.vertices {
            let current_edge_space = size_edge_space * node;

            for friend in friends {
                connected_index
                    .entry(*node)
                    .and_modify(|already_used| *already_used += 1)
                    .or_default();
                connected_index
                    .entry(*friend)
                    .and_modify(|already_used| *already_used += 1)
                    .or_default();
                let friend_edge_space = size_edge_space * friend;

                let start =
                    boxed_lenght_before(&boxes, *node) + current_edge_space + connected_index[node];
                let size = boxed_lenght_before(&boxes, *friend) + friend_edge_space - start
                    + connected_index[friend];
                let mut line = filled_line(len_line, start, size as isize - 1, '-');

                for (dn, count) in &draw_times {
                    let start = boxed_lenght_before(&boxes, *dn) + size_edge_space * dn;
                    line = filled_from(&line, start, *count, '|');
                }

                let mut connect = filled_line(len_line, 0, len_line as isize, ' ');
                connect = change_by_index(&connect, start, '|');
                connect = change_by_index(&connect, start + size, '|');
                for (dn, count) in &draw_times {
                    let start = boxed_lenght_before(&boxes, *dn) + size_edge_space * dn;
                    connect = filled_from(&connect, start, *count, '|');
                }

                draw_times
                    .entry(*node)
                    .and_modify(|already_used| *already_used += 1)
                    .or_insert(1);
                draw_times
                    .entry(*friend)
                    .and_modify(|already_used| *already_used += 1)
                    .or_insert(1);

                writeln!(f, "{}", line)?;
                writeln!(
                    f,
                    "{} {} {} {}",
                    connect,
                    start,
                    size,
                    boxed_lenght_before(&boxes, *friend)
                )?;
            }
        }

        let str_boxes = boxes.iter().map(String::from).collect::<Vec<String>>();
        let boxed_edges =
            flatten_line(&str_boxes.iter().map(|b| b.as_ref()).collect::<Vec<&str>>());
        write!(f, "{}", boxed_edges)?;
        Ok(())
    }
}

fn change_by_index(origin: &str, index: usize, c: char) -> String {
    let mut str = String::with_capacity(origin.len());
    for (i, symbol) in origin.chars().enumerate() {
        if i == index {
            str.push(c);
        } else {
            str.push(symbol);
        }
    }

    str
}

fn box_len_line(boxes: &[FormatBox]) -> usize {
    boxes.iter().fold(0, |acc, n| acc + n.line_lenght())
}

fn filled_line(size: usize, from: usize, mut s: isize, symbol: char) -> String {
    let mut line = String::new();
    let mut i = 0;
    while i < size {
        if i > from && s > 0 {
            line.push(symbol);
            s -= 1;
        } else {
            line.push(' ');
        }
        i += 1;
    }

    line
}

fn filled_from(origin: &str, from: usize, to: usize, symbol: char) -> String {
    let mut line = String::new();
    let mut added = 0;
    for (i, s) in origin.chars().enumerate() {
        if i >= from && added < to {
            line.push(symbol);
            added += 1;
        } else {
            line.push(s);
        }
    }

    line
}

fn boxed_lenght_before(words: &[FormatBox], i: usize) -> usize {
    words.iter().take(i).fold(0, |acc, w| acc + w.line_lenght())
}

struct FormatBox<'a> {
    message: &'a str,
    tab_size: usize,
}

impl<'a> FormatBox<'a> {
    fn new(s: &'a str, tab_size: usize) -> Self {
        FormatBox {
            message: s,
            tab_size,
        }
    }

    fn line_lenght(&self) -> usize {
        2 + self.tab_size * 2 + self.message.len()
    }
}

impl<'a> std::fmt::Display for FormatBox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let horizontal_tab = " ".repeat(self.tab_size);
        let horizontal_line = "-".repeat(self.line_lenght());
        let vertical_space = format!("|{}|", " ".repeat(self.line_lenght() - 2));
        let content: String = self
            .message
            .lines()
            .map(|l| format!("|{}{}{}|", horizontal_tab, l, horizontal_tab))
            .collect();

        let vertical_space_lined = if self.tab_size > 0 {
            format!("{}\n", vertical_space)
        } else {
            "".to_owned()
        };

        write!(
            f,
            "{}\n\
             {}\
             {}\n\
             {}\
             {}",
            horizontal_line, vertical_space_lined, content, vertical_space_lined, horizontal_line
        )?;
        Ok(())
    }
}

impl<'a> std::convert::From<&FormatBox<'a>> for String {
    fn from(b: &FormatBox<'a>) -> String {
        format!("{}", b)
    }
}

fn flatten_line(src: &[&str]) -> String {
    let element_with_max_lines = src
        .iter()
        .max_by(|x, y| x.lines().count().cmp(&y.lines().count()));
    let max_lines = match element_with_max_lines {
        Some(element) => element.lines().count(),
        None => 0,
    };

    let mut lines = String::new();
    for line_index in 0..max_lines {
        for source in src {
            let element_lines = source.lines().collect::<Vec<&str>>();
            let max_line_size = size_biggest_line(source);
            let line = match element_lines.get(line_index) {
                Some(line) => format!("{: <1$}", line, max_line_size),
                None => " ".repeat(max_line_size),
            };

            lines.push_str(&line);
            lines.push(' ');
        }
        lines.push('\n');
    }

    lines
}

fn size_biggest_line(s: &str) -> usize {
    s.lines().fold(
        0,
        |max, item| {
            if item.len() > max {
                item.len()
            } else {
                max
            }
        },
    )
}
