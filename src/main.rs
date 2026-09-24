use std::println;

use clap::Parser;
mod models;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    // name of the task
    #[arg(short, long, value_name="TASK")]
    add:Option<String>,

    #[arg(short, long)]
    show:bool
}

fn main(){
    let args = Args::parse();
    
    if let Some(task) = args.add {
        let _ = models::build_task(task);
    }

    if args.show {
        let _ = models::read_tasks();
    }
    
}
