use std::process::Command;
use std::io;
use serde_json::Value;

struct Field {
    //TODO: define struct for the field array found in the JSON.    
}

struct Item {
    id: String,
    folder_id: String,
    name: String,
    favorite: bool,
    username: String,
    password: String,

}

fn main() {
    // Get Password From User Input
    let mut pass = String::new();
    io::stdin().read_line(&mut pass).expect("Could not read line");

    // run bitwarden-cli's unlock
    let bwunlock = Command::new("bw")
        .arg("unlock")
        .arg(pass.trim())
        .output()
        .expect("failed to execute process");
    
    // process stdout for session key and print for debugging
    let sessionoutput = String::from_utf8_lossy(&bwunlock.stdout);
    let start_bytes = sessionoutput.find("BW_SESSION=").unwrap_or(0);
    let end_bytes = sessionoutput.find("==\"").unwrap_or(sessionoutput.len());
    let session = &sessionoutput[start_bytes+12..end_bytes+2];
    println!("Session key is: {}", session);

    // run bitwarden-cli's list!
    let bwitems = Command::new("bw")
        .arg("list")
        .arg("items")
        .arg("--session")
        .arg(session)
        .output()
        .expect("failed to execute process");
    
    // debugging
    //println!("status: {}", output2.status);
    //println!("stdout: {}", String::from_utf8_lossy(&output2.stdout));
    //println!("stderr: {}", String::from_utf8_lossy(&output2.stderr));
    
    // process stdout for json
    let listoutput = String::from_utf8(bwitems.stdout).unwrap();
    
    let itemsjson: Vec<Value> = serde_json::from_str(&listoutput).unwrap();

    //println!("{}", v.items[0].name); 
    let mut items: Vec<Item> = Vec::new();
    for item in itemsjson.iter() { 
        let entry = Item {
            id: item["id"].as_str().unwrap_or_default().to_owned(),
            folder_id: item["folderid"].as_str().unwrap_or_default().to_owned(),
            name: item["name"].as_str().unwrap_or_default().to_owned(),
            favorite: item["favorite"].as_bool().unwrap_or_default().to_owned(),
            username: item["login"]["username"].as_str().unwrap_or_default().to_owned(),
            password: item["login"]["password"].as_str().unwrap_or_default().to_owned(),
        };
        items.push(entry);
        //println!("{} | {} | {}", item["name"], item["login"]["username"], item["login"]["password"] );
    }
    for i in items.iter() {
        println!("{} | {} | {}", i.name, i.username, i.password );
    }
}
