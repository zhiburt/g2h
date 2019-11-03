use regex::Regex;
use std::io::{self, BufRead, Write};

use colored::Colorize;
use rand::Rng;

use g2h::{
    pane::{self, Surface},
    line_gh as gh,
    path_matrix,
    animated_path,
};

fn main() -> io::Result<()> {
    let mut gh = gh::LineGH::new();
    let mut matrix = pane::MatrixPane::new(0, 0, "");

    let command_prefix = b">>> ";
    let stdin = io::stdin();
    let stdout = io::stdout();
    loop {
        stdout.lock().write_all(command_prefix)?;
        stdout.lock().flush()?;

        let mut stdin = stdin.lock();
        let buffer = stdin.fill_buf()?;
        if buffer.is_empty() {
            return Ok(());
        }

        let lines = buffer
            .split(u8::is_ascii_control)
            .map(std::str::from_utf8)
            .flatten()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        for line in lines {
            let command = parse_command(&line);
            matrix = handle_command(&mut stdout.lock(), &mut gh, matrix, command)?;
        }

        let len = buffer.len();
        stdin.consume(len);
    }
}

#[derive(Debug)]
enum Command {
    Print,
    SetGHType,
    SetGap(usize),
    SetConnectionSize(usize),
    Structure,
    AddEdge(Box<String>),
    ConnectEdges(usize, usize),
    MatrixInit(usize, usize),
    RandomMatrixInit(usize, usize),
    MatrixPrint,
    MatrixSearch(usize, usize),
    MatrixSearchAnimated(usize, usize),
    MatrixSetWeight(usize, usize, usize),
    MatrixBlockVertices(usize),
    StructureMatrix,
}

fn parse_command(line: &str) -> Option<Command> {
    let clean_line = line.trim();

    if clean_line.starts_with("print") {
        Some(Command::Print)
    } else if clean_line.starts_with("structure") {
        Some(Command::Structure)
    } else if clean_line.starts_with("settings") {
        let gap_regex = Regex::new(r"settings gap edge (?P<size>.+)").unwrap();
        let connection_size_regex = Regex::new(r"settings gap vert (?P<size>.+)").unwrap();

        if gap_regex.is_match(clean_line) {
            let caps = gap_regex.captures(clean_line).unwrap();
            let size = caps["size"].parse().unwrap();

            Some(Command::SetGap(size))
        } else if connection_size_regex.is_match(clean_line) {
            let caps = connection_size_regex.captures(clean_line).unwrap();
            let size = caps["size"].parse().unwrap();

            Some(Command::SetConnectionSize(size))
        } else if clean_line.contains("settings related") {
            Some(Command::SetGHType)
        } else {
            None
        }
    } else if clean_line.starts_with("matrix") {
        let init_command = Regex::new(r"matrix (?P<weight>\d+) (?P<hight>\d+)").unwrap();
        let search_command = Regex::new(r"matrix search (?P<from>\d+) (?P<look>\d+)").unwrap();
        let search_animated_command = Regex::new(r"matrix search animated (?P<from>\d+) (?P<look>\d+)").unwrap();
        let set_weight_command = Regex::new(r"matrix weight (?P<index>\d+) (?P<edge>\d+) (?P<weight>\d+)").unwrap();
        let block_command = Regex::new(r"matrix block (?P<index>\d+)").unwrap();
        let random_command = Regex::new(r"matrix random (?P<weight>\d+) (?P<hight>\d+)").unwrap();
        let structure_command = Regex::new(r"matrix structure").unwrap();

        if clean_line.contains("matrix print") {
            Some(Command::MatrixPrint)
        } else if init_command.is_match(clean_line) {
            let caps = init_command.captures(clean_line).unwrap();
            let w = caps["weight"].parse().unwrap();
            let h = caps["hight"].parse().unwrap();
            Some(Command::MatrixInit(w, h))
        } else if search_command.is_match(clean_line) {
            let caps = search_command.captures(clean_line).unwrap();
            let w = caps["from"].parse().unwrap();
            let h = caps["look"].parse().unwrap();
            Some(Command::MatrixSearch(w, h))
        } else if search_animated_command.is_match(clean_line) {
            let caps = search_animated_command.captures(clean_line).unwrap();
            let w = caps["from"].parse().unwrap();
            let h = caps["look"].parse().unwrap();
            Some(Command::MatrixSearchAnimated(w, h))
        } else if set_weight_command.is_match(clean_line) {
            let caps = set_weight_command.captures(clean_line).unwrap();
            let index = caps["index"].parse().unwrap();
            let edge = caps["edge"].parse().unwrap();
            let weight = caps["weight"].parse().unwrap();
            Some(Command::MatrixSetWeight(index, edge, weight))
        } else if block_command.is_match(clean_line) {
            let caps = block_command.captures(clean_line).unwrap();
            let index = caps["index"].parse().unwrap();
            Some(Command::MatrixBlockVertices(index))
        } else if random_command.is_match(clean_line) {
            let caps = random_command.captures(clean_line).unwrap();
            let w = caps["weight"].parse().unwrap();
            let h = caps["hight"].parse().unwrap();
            Some(Command::RandomMatrixInit(w, h))
        }  else if structure_command.is_match(clean_line) {
            Some(Command::StructureMatrix)
        } else {
            None
        }
    } else {
        let add_edge_command = Regex::new(r"edge add (?P<data>.+)").unwrap();
        let add_verticale_command =
            Regex::new(r"edge connect (?P<first>\d+) (?P<second>\d+)").unwrap();

        if add_edge_command.is_match(clean_line) {
            let caps = add_edge_command.captures(clean_line).unwrap();
            Some(Command::AddEdge(Box::new(String::from(&caps["data"]))))
        } else if add_verticale_command.is_match(clean_line) {
            let caps = add_verticale_command.captures(clean_line).unwrap();
            let first = caps["first"].parse().unwrap();
            let second = caps["second"].parse().unwrap();

            Some(Command::ConnectEdges(first, second))
        } else {
            None
        }
    }
}

fn handle_command<W: Write>(
    w: &mut W,
    gh: &mut gh::LineGH,
    mut matrix:  pane::MatrixPane,
    command: Option<Command>,
) -> io::Result<pane::MatrixPane> {
    match command {
        Some(Command::Print) => {
            writeln!(w, "{}", gh)?;
        },
        Some(Command::Structure) => {},
        Some(Command::AddEdge(data)) => {
            gh.add_edge(&data);
        },
        Some(Command::ConnectEdges(from, to)) => {
            gh.connect(from, to);
        },
        Some(Command::SetGap(size)) => { gh.pane_settings.gap_size = size },
        Some(Command::SetConnectionSize(size)) => { gh.pane_settings.connection_size = size },
        Some(Command::SetGHType) => { 
            if gh.pane_settings.connection_type == pane::ConnectorType::General {
                gh.pane_settings.connection_type = pane::ConnectorType::Arrow;
            } else {
                gh.pane_settings.connection_type = pane::ConnectorType::General;
            }
        },
        Some(Command::MatrixInit(w, h)) => {
            matrix = pane::MatrixPane::new(w, h, &"▅".black().to_string());
        },
        Some(Command::MatrixPrint) => {
            writeln!(w, "{}", matrix.pane())?;
        }
        Some(Command::MatrixSearch(from, look)) => {
            matrix = path_matrix::construct_path(matrix, from, look, &"▅".red().to_string(), &"▅".yellow().to_string());
            writeln!(w, "{}", matrix.pane())?;
            matrix.clean();
        },
        Some(Command::MatrixSearchAnimated(from, look)) => {
            let frames = animated_path::frames(&mut matrix, from, look, &"▅".red().to_string(), &"▅".yellow().to_string());
            for frame in frames {
                writeln!(w, "{}", termion::clear::BeforeCursor)?;
                writeln!(w, "{}", frame)?;
                std::thread::sleep(std::time::Duration::from_millis(20)); // should be a relation from a size of frames or graph?
            }
            matrix.clean();
        },
        Some(Command::MatrixBlockVertices(index)) => {
            if let Some(node) = matrix.get_node(index) {
                node.borrow_mut().edges = None;
            }
        },
        Some(Command::MatrixSetWeight(index, edge_index, weight)) => {
            if let Some(node) = matrix.get_node(index) {
                if let Some(edges) = node.borrow_mut().edges.as_mut() {
                    edges[edge_index].weight = weight
                }
            }
        },
        Some(Command::RandomMatrixInit(w, h)) => {
            matrix = pane::MatrixPane::new(w, h, &"▅".black().to_string());
            for node in &matrix.node_list[..matrix.node_list.len()-1] {
                if let Some(edges) = node.borrow_mut().edges.as_mut() {
                    if let Some(edge) = edges.get_mut(0) {
                        edge.weight = rand::thread_rng().gen_range(0, 100);
                    }
                    if let Some(edge) = edges.get_mut(1) {
                        edge.weight = rand::thread_rng().gen_range(0, 100);
                    }
                }
            }
        },
        Some(Command::StructureMatrix) => {
            let structure = matrix.structure().to_string();
            let structure = gh::FormatBox::new(&structure, 1);
            writeln!(w, "{}", structure)?;
        },
        None => {
            writeln!(w, "cannot hold this type of command")?;
        },
    }

    Ok(matrix)
}
