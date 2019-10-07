use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let gh = graph::Node {
        data: (0, "0"),
        parent: None,
        children: Some(
            vec![
            Box::new(graph::Node::new((1, "1"))),
            Box::new(graph::Node::new((2, "2"))),
            Box::new(graph::Node::new((3, "3"))),
            Box::new(graph::Node::new((4, "4"))),
        ]),
    };
    let linear_graph = LineGH::new(&["aaa", "bbb", "ccc", "ddd"]).
        connect(0, 1).
        connect(1, 2).
        connect(1, 3).
        connect(0, 3);
    println!("{}", linear_graph);
}

#[derive(Debug)]
struct LineGH<'a> {
    vertices: BTreeMap<usize, Vec<usize>>,
    edges:  Vec<&'a str>,
}

impl<'a> LineGH<'a> {
    pub fn new(edges: &[&'a str]) -> Self {
        let mut pins = BTreeMap::new();
        for i in 0 .. edges.len() {
            pins.insert(i, 0);
        }

        LineGH{
            edges: edges.to_vec(),
            vertices: BTreeMap::new(),
        }
    }

    pub fn connect(mut self, e1: usize, e2: usize) -> Self {
        self.vertices.entry(e1).or_insert(Vec::new()).push(e2);

        self
    }
}

impl<'a> std::fmt::Display for LineGH<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let edge_space = " ";
        let size_edge_space = 1;
        let max_space = size_edge_space * (self.edges.len() - 1);
        let len_line = len_line(&self.edges) + max_space;
        let mut connected_index: BTreeMap<usize, usize> = BTreeMap::new();
        let mut draw_times: BTreeMap<usize, usize> = BTreeMap::new();

        let mut iteration = 0;
        for (node, friends) in &self.vertices {
            let current_edge_space =  size_edge_space * node;

            let mut draw_iteration = 0;
                for friend in friends  {
                connected_index.entry(*node).and_modify(|already_used| *already_used += 1).or_default();
                connected_index.entry(*friend).and_modify(|already_used| *already_used += 1).or_default();
                let friend_edge_space =  size_edge_space * friend;

                let start = lenght_before(&self.edges, *node) + current_edge_space + connected_index[node] ;
                let size = lenght_before(&self.edges, *friend) + friend_edge_space - start + connected_index[friend] ;
                let mut line = filled_line(len_line, start, size as isize - 1, '-');

                for (dn, count) in &draw_times {
                    let start = lenght_before(&self.edges, *dn) + size_edge_space * dn;
                    line = filled_from(&line, start, *count, '|');
                }

                let mut connect = filled_line(len_line, 0, len_line as isize, ' ');
                connect = change_by_index(&connect, start, '|');
                connect = change_by_index(&connect, start + size, '|');  
                
                for (dn, count) in &draw_times {
                    let start = lenght_before(&self.edges, *dn) + size_edge_space * dn;
                    connect = filled_from(&connect, start, *count, '|');
                }

                draw_times.entry(*node).and_modify(|already_used| *already_used += 1).or_insert(1);
                draw_times.entry(*friend).and_modify(|already_used| *already_used += 1).or_insert(1);
            
                draw_iteration += 1;

                writeln!(f, "{}", line)?;
                writeln!(f, "{}", connect)?;
            }

            iteration += 1;
        }

        let boxes = self.edges.iter().map(|s| boxed(s, 0)).collect::<Vec<String>>();
        let boxed_edges = flatten_line(&boxes.iter().map(|s| s.as_str()).collect::<Vec<&str>>()).unwrap();
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

fn len_line(nodes: &[&str]) -> usize {
    nodes.iter().fold(0, |acc, n| acc + n.len())
}

fn with_pin(s: &str, n: usize) -> String {
    let mut pin = String::new();
    let mut i = 0;
    for (pos, symb) in s.char_indices() {
        if i < n {
            pin.push('|');
        } else {
            pin.push(symb);
        }
        i += 1;
    }

    pin
}

fn with_line(s: &str, from: usize, to: usize) -> String {
    let mut pin = String::new();
    let mut i = 0;
    for (pos, symb) in s.char_indices() {
        if i > from && i < to {
            pin.push('-');
        } else {
            pin.push(symb);
        }
        i += 1;
    }

    pin
}

fn cross_space(s: &str, mut used_points: usize, cross: char) -> String {
    let mut crossed = String::new();
    let mut i = 0;
    for (pos, symb) in s.char_indices() {
        if i < used_points {
            crossed.push(cross);
        } else {
            crossed.push(symb);
        }
        i += 1;
    }

    crossed
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

fn space(n: usize) -> String {
    " ".repeat(n)
}

fn line(n: usize) -> String {
    "-".repeat(n)
}

fn lenght_before(words: &[&str], i: usize) -> usize {
    words.iter().take(i).fold(0, |acc, w| acc + w.len())
}

fn index_to_start(words: &[&str], i: usize) -> usize {
    words.iter().take(i).fold(0, |acc, w| acc + w.len())
}

fn boxed(s: &str, tab_size: usize) -> String {
    let horizontal_tab = " ".repeat(tab_size);
    let horizontal_line = "-".repeat(s.len() + 2 + tab_size + tab_size);
    let vertical_space = format!("|{}|", " ".repeat(s.len() + tab_size + tab_size));
    let content: String = s
        .lines()
        .map(|l| format!("|{}{}{}|", horizontal_tab, l, horizontal_tab))
        .collect();

    
    let vertical_space_lined = match tab_size > 0 {
        true => format!("{}\n", vertical_space),
        false => "".to_owned(),
    };

    format!(
        "{}\n\
         {}\
         {}\n\
         {}\
         {}",
        horizontal_line, vertical_space_lined, content, vertical_space_lined, horizontal_line
    )
}

fn flatten_line(src: &[&str]) -> Option<String> {
    if src.len() < 0 {
        return None;
    }

    let size = src[0].lines().count();
    if !src.iter().all(|e| e.lines().count() == size) {
        println!("{} {:?}", size, src);
        return None;
    }

    let mut lines = String::new();
    for line_index in 0..size {
        for source in src {
            let line = source.lines().collect::<Vec<&str>>();
            lines.push_str(line[line_index]);
            lines.push(' ');
        }
        lines.push('\n');
    }

    Some(lines)
}
