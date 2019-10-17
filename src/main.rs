use regex::Regex;
use std::io::{self, BufRead, Write};

use g2h::{
    pane::{self},
    line_gh as gh,
};

fn main() -> io::Result<()> {
    let mut gh = gh::LineGH::new();

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
            handle_command(&mut stdout.lock(), &mut gh, command)?;
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
    }else {
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
    command: Option<Command>,
) -> io::Result<()> {
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
        None => {
            writeln!(w, "cannot hold this type of command")?;
        },
    }

    Ok(())
}
