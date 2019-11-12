
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::pane::{self, Surface};
use colored::Colorize;

pub struct LineGH {
    // might use here real graph?
    pub vertices: BTreeMap<usize, Vec<usize>>,
    pub edges: Vec<(String, Option<(colored::Color, String)>)>,
    pub pane_settings: pane::PaneSettings,
}

impl LineGH {
    pub fn new() -> Self {
        LineGH::new_with_settings(pane::PaneSettings{
            gap_size: 1,
            connection_size: 1,
            connection_type: pane::ConnectorType::General,
        })
    }

    pub fn new_with_settings(settings: pane::PaneSettings) -> Self {
        LineGH {
            edges: Vec::new(),
            vertices: BTreeMap::new(),
            pane_settings: settings,
        }
    }

    pub fn add_edge(&mut self, edge: &str) -> usize {
        self.edges.push((String::from(edge), None));
        self.edges.len() - 1
    }

    pub fn connect(&mut self, e1: usize, e2: usize) {
        self.vertices.entry(e1).or_insert_with(Vec::new).push(e2);
        self.vertices.entry(e2).or_insert_with(Vec::new);
    }

    pub fn change(&mut self, i: usize, col: colored::Color, space_symbol: &str) {
        let element = self.edges.get_mut(i).unwrap();               
        element.1 = Some((col, space_symbol.to_owned()));
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

    pub fn structure(&self) -> BTreeMap<usize, (usize, usize)> {
        BTreeMap::from_iter(
            self.vertices.keys().cloned().
                zip(self.vertices.iter().
                map(|(i, connected)| (connected.len(), self.count_by(*i) - connected.len())))
        )
    }
} 

impl std::fmt::Display for LineGH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: logic with boxes should be refactored
        let boxes = self
            .edges
            .iter()
            .enumerate()
            .map(|(i, (s, opt))| {
                let count_connected = self.count_by(i);
                let single_box = match opt {
                    Some((color , space_sign)) => {
                        FormatBox::with_color(s, *color, space_sign, 1)
                    }
                    None => FormatBox::new(s, 1)
                };
                let max_on_line = f64::ceil(single_box.line_lenght() as f64 / self.pane_settings.connection_size as f64) as usize;
                if count_connected > max_on_line {
                    match opt {
                        Some((color , space_sign)) => {
                            FormatBox::with_color(s, *color, space_sign, (count_connected - max_on_line) * self.pane_settings.connection_size + 1)}
                        None => FormatBox::new(s, (count_connected - max_on_line) * self.pane_settings.connection_size + 1)
                    }
                } else {
                    single_box
                }
            })
            .collect::<Vec<FormatBox>>();

        let boxes_length = boxes
            .iter()
            .map(FormatBox::line_lenght)
            .collect::<Vec<usize>>();
        let mut pane = pane::ConnectedPane::new(&boxes_length, self.pane_settings.clone());

        for (node, friends) in &self.vertices {
            for friend in friends {
                pane.connect(*node, *friend);
            }
        }

        writeln!(f, "{}", pane.pane())?;

        let boxed_edges = flatten_line(&boxes,
            self.pane_settings.gap_size
        );
        write!(f, "{}", boxed_edges)?;
        Ok(())
    }
}

fn new_line(index: usize, count_in: usize, count_out: usize) -> String {
    format!("{} - in {} out {}", index, count_in, count_out)
}

pub struct FormatBox<'a> {
    message: &'a str,
    space_symbol: &'a str,
    tab_size: usize,
    color: Option<colored::Color>,
}

impl<'a> FormatBox<'a> {
    pub fn new(s: &'a str, tab_size: usize) -> Self {
        FormatBox {
            message: s,
            space_symbol: &" ",
            tab_size,
            color: None,
        }
    }

    pub fn with_color(s: &'a str, color: colored::Color, space: &'a str, tab_size: usize) -> Self {
        let mut fbox = FormatBox::new(s, tab_size);
        fbox.space_symbol = space;
        fbox.color = Some(color);
        
        fbox
    }

    fn line_lenght(&self) -> usize {
        2 + self.tab_size * 2 + size_biggest_line(&self.message)
    }

    fn create_lines(&self) -> Vec<String> {
        let horizontal_tab = self.space_symbol.repeat(self.tab_size);
        let horizontal_line = "-".repeat(self.line_lenght());
        let vertical_space = format!("|{}|", self.space_symbol.repeat(self.line_lenght() - 2));
        let max_len = size_biggest_line(&self.message);
        let content = self
            .message
            .lines()
            .map(|l| {
                match self.color {
                    Some(color) => format!("|{}{: <3$}{}|", horizontal_tab, l.color(color), horizontal_tab, max_len),
                    None => format!("|{}{: <3$}{}|", horizontal_tab, l, horizontal_tab, max_len)
                }
            })
            .map(|l| l.replace(" ", self.space_symbol))
            .collect::<Vec<String>>();

        let mut lines = Vec::new();
        lines.push(horizontal_line.clone());
        lines.push(vertical_space.clone());
        lines.extend(content);
        lines.push(vertical_space);
        lines.push(horizontal_line);

        lines
    }
}

impl<'a> std::fmt::Display for FormatBox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.create_lines().join("\n"))
    }
}

impl<'a> std::convert::From<&FormatBox<'a>> for String {
    fn from(b: &FormatBox<'a>) -> String {
        format!("{}", b)
    }
}

// I can try to create lines Vec<String> field in format box and work with it.
// or somehow rewrite this method
fn flatten_line(src: &[FormatBox], gap_size: usize) -> String {
    let src_lines = src.iter().map(FormatBox::create_lines).collect::<Vec<_>>();
    let element_with_max_lines = src_lines
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()));
    let max_lines = match element_with_max_lines {
        Some(element) => element.len(),
        None => 0,
    };

    let mut lines = String::new();
    for line_index in 0..max_lines {
        for source in src_lines.iter() {
            let max_line_size = source.iter().max_by(|x, y| y.len().cmp(&x.len())).map_or(0, |e| e.len());
            let line = match source.get(line_index) {
                Some(line) => format!("{: <1$}", line, max_line_size),
                None => " ".repeat(max_line_size),
            };

            lines.push_str(&line);
            lines.push_str(&" ".repeat(gap_size));
        }
        lines.push('\n');
    }

    String::from(lines.trim())
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
