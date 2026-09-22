use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::io::prelude::*;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    // name of the task
    #[arg(short, long, value_name="TASK")]
    add:Option<String>,

    #[arg(short, long)]
    show:bool
}
fn main()-> io::Result<()>{
    let args = Args::parse();
    
    if let Some(task) = args.add {
        let mut file = OpenOptions::new().create(true).append(true).open("todo.txt")?;
        writeln!(file, "{}", task)?;
    }

     if args.show {
        let mut file = File::open("todo.txt")?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        if contents.is_empty() {
            println!("Aucune tâche.");
        } else {
            println!("Tâches :");
            print!("{}", contents);
        }
    }

    Ok(())
    
}
