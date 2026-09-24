
use serde::{Deserialize, Serialize};
use chrono::{Utc};
use uuid::Uuid;
use std::io;
use std::fs;
use std::println;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id:String,
    content: String,
    created_at: String
}

pub fn build_task(content:String) -> std::io::Result<()>{
    let existing_data = match fs::read_to_string("task.json") {
        Ok(data) if !data.trim().is_empty() => data,
        Ok(_) | Err(_) => "[]".to_string(),
    };

    let mut tasks: Vec<Task> = serde_json::from_str(&existing_data)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;


    let new_task =  Task {
        id : Uuid::new_v4().to_string(),
        content: content,
        created_at : Utc::now().to_string()
    };

    tasks.push(new_task);

    let json = serde_json::to_string_pretty(&tasks).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write("task.json", json)?;
    
    Ok(())
    
}


pub fn read_tasks()->std::io::Result<()>{

    let file = fs::read_to_string("task.json").expect("Impossible de lire le fichier des taches");

    let json = serde_json::to_string_pretty(&file).expect("Pas de taches");

    println!("Vos taches : {}", json);

    Ok(())


}