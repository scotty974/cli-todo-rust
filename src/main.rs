use clap::Parser;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    // name of the person to greet
    #[arg(short, long)]
    name:String
}
fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.name);
    
}
