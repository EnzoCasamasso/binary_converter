mod file;
mod view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Try use {} <file>", args[0]);
        std::process::exit(1);
    }
}
