use clap::Parser;
use fold_core::hash::Hasher;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    println!("FOLD CLI v0.1.0");
    let _h = Hasher::hash_bytes(b"init");
}