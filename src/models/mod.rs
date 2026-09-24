
use clap::Id;
use serde::{Deserialize, Serialize};
use chrono::{Utc};
use serde_json::Value;
use uuid::Uuid;
use std::io;
use std::fs;
use std::println;
use colored_json::ToColoredJson;

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

    let colored_json = file.to_colored_json_auto()?;

    // let json : serde_json::Value = serde_json::from_str(&file).expect("Pas de taches");

    // let formatted_json = serde_json::to_string_pretty(&json)?;

    println!("{}", colored_json);

    Ok(())


}


pub fn delete_task(task_id:String){
    let file = fs::read_to_string("task.json").expect("Impossible de lire le fichier");

    let  mut json : serde_json::Value = serde_json::from_str(&file).expect("Erreur lors du parsing");

    if let Value::Array(ref mut arr) = json {
        arr.retain(|item| {
            item.get("id").and_then(|v| v.as_str()) != Some(task_id.as_str())
        });

           if arr.len() == arr.len() {
            println!("Erreur : La tâche avec l'id '{}' n'existe pas", task_id);
            return;
        }
    }
    
    let json_string = serde_json::to_string_pretty(&json).expect("Erreur de la sérilisation");

    let _ = fs::write("task.json", json_string);
    


}