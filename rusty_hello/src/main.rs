use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: Option<String>,

    /// Repeat the greeting n times
    #[arg(short, long, default_value_t = 1)]
    repeat: u64,

    /// Print the greeting in uppercase
    #[arg(short, long)]
    upper: bool,
}

fn main() {
    let args = Args::parse();

    let name = args.name.as_deref().unwrap_or("World");

    let mut message = format!("Hello, {}!", name);
    if args.upper {
        message = message.to_uppercase();
    }

    for _ in 0..args.repeat {
        println!("{}", message);
    }
}
