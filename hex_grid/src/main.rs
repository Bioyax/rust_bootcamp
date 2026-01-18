use clap::Parser;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use pathfinding::prelude::astar;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Generate a new map (e.g., 5x5)
    #[arg(long)]
    generate: Option<String>,

    /// The output file for the generated map
    #[arg(long)]
    output: Option<PathBuf>,

    /// The map file to find the shortest path on
    #[arg(long)]
    map: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Hex {
    q: i32,
    r: i32,
}

impl Hex {
    fn new(q: i32, r: i32) -> Self {
        Hex { q, r }
    }

    fn distance(&self, other: &Hex) -> u32 {
        ((self.q - other.q).abs() + (self.q + self.r - other.q - other.r).abs() + (self.r - other.r).abs()) as u32 / 2
    }

    fn neighbors(&self) -> Vec<Hex> {
        vec![
            Hex::new(self.q + 1, self.r),
            Hex::new(self.q - 1, self.r),
            Hex::new(self.q, self.r + 1),
            Hex::new(self.q, self.r - 1),
            Hex::new(self.q + 1, self.r - 1),
            Hex::new(self.q - 1, self.r + 1),
        ]
    }
}

struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<u32>>,
}

impl Grid {
    fn get_weight(&self, hex: &Hex) -> Option<u32> {
        let col = hex.q + (hex.r - (hex.r & 1)) / 2;
        let row = hex.r;
        if col >= 0 && col < self.width as i32 && row >= 0 && row < self.height as i32 {
            Some(self.tiles[row as usize][col as usize])
        } else {
            None
        }
    }
}

fn read_map(path: &PathBuf) -> std::io::Result<Grid> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    let width: usize = parts[0].parse().unwrap();
    let height: usize = parts[1].parse().unwrap();

    let mut tiles = Vec::new();
    for line in reader.lines() {
        let row: Vec<u32> = line?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        tiles.push(row);
    }

    Ok(Grid {
        width,
        height,
        tiles,
    })
}

fn find_shortest_path(grid: &Grid) -> Option<(Vec<Hex>, u32)> {
    let start = Hex::new(0, 0);
    let end = Hex::new(grid.width as i32 - 1, grid.height as i32 - 1);

    astar(
        &start,
        |p| {
            p.neighbors()
                .into_iter()
                .filter_map(|n| grid.get_weight(&n).map(|w| (n, w)))
                .collect::<Vec<_>>()
        },
        |p| p.distance(&end),
        |p| *p == end,
    )
}

fn generate_map(size: &str, output: &PathBuf) -> std::io::Result<()> {
    let parts: Vec<&str> = size.split('x').collect();
    if parts.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid size format. Use WxH (e.g., 5x5).",
        ));
    }
    let width: usize = parts[0].parse().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid width.",
        )
    })?;
    let height: usize = parts[1].parse().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid height.",
        )
    })?;

    let mut file = File::create(output)?;
    writeln!(file, "{} {}", width, height)?;

    let mut rng = rand::thread_rng();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(rng.gen_range(1..=9));
        }
        let row_str: Vec<String> = row.iter().map(|n| n.to_string()).collect();
        writeln!(file, "{}", row_str.join(" "))?;
    }

    println!("Map saved to: {}", output.display());
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Some(size) = cli.generate {
        if let Some(output) = cli.output {
            if let Err(e) = generate_map(&size, &output) {
                eprintln!("Error generating map: {}", e);
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: --output is required when --generate is used.");
            std::process::exit(1);
        }
    } else if let Some(map_path) = cli.map {
        match read_map(&map_path) {
            Ok(grid) => {
                if let Some((_path, _cost)) = find_shortest_path(&grid) {
                    println!("MINIMUM COST PATH");
                } else {
                    eprintln!("No path found.");
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error reading map: {}", e);
                std::process::exit(1);
            }
        }
    }
}
