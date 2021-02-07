use cursive::traits::*;
use cursive::views::{Dialog, EditView, TextView, SelectView, ScrollView, LinearLayout};
use cursive::Cursive;

use std::process::Command;
use serde_json::Value;
use cli_clipboard;


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
    let mut siv = cursive::default();
    siv.add_layer(
        Dialog::new()
            .title("Enter Your Password")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    // Call show_popup when the user presses Enter
                    .on_submit(show_popup)
                    // Give the EditView a name
                    .with_name("pass")
                    .fixed_width(40),
            )
            .button("Ok", |s| { 
                let pass = s.call_on_name("pass", |view: &mut EditView| {view.get_content()})
                    .unwrap();

                // password dialog
                show_popup(s, &pass);
            }),
    );

    siv.run();
}

// This will replace the current layer with a new popup.
// If the name is empty, we'll show an error message instead.
fn show_popup(s: &mut Cursive, pass: &str) {
    if pass.is_empty() {
        // Try again if empty
        s.add_layer(Dialog::info("Please enter a password"));
    } else {
        // Remove the initial popup
        s.pop_layer();
        
        // get session key
        let session_key = get_session_key(pass.to_owned()); 
        
        // get items using session key
        let items = get_items(session_key); 

        // Create select view using the name of the items
        let select = SelectView::<String>::new()
            .on_submit(|s, _: &str| {
                let selectview = s.find_name::<SelectView>("select").unwrap();
                let selected_id = selectview.selected_id().unwrap();
                let s = selectview.get_item(selected_id);
                let x = s.unwrap();
                //let id_int: usize = x.1.parse().unwrap();
                //cli_clipboard::set_contents(items[id_int].password.clone()).unwrap();
                cli_clipboard::set_contents(x.1.to_owned()).unwrap();
            })
            .with_name("select");

        let scroll = ScrollView::new(select)
            .with_name("scroll")
            .fixed_width(50);

        s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(scroll))
            .title("Select a profile"));

        for (i, item) in items.iter().enumerate() {
            s.call_on_name("select", |view: &mut SelectView<String>| {
                //view.add_item_str(item.name.clone())
                //view.add_item(item.name.clone(), format!("{}", i))
                view.add_item(item.name.clone(), item.password.clone())
            });
        }

        /*s.add_layer(
            Dialog::around(TextView::new(names_str))
                .button("Quit", |s| s.quit()),
        );*/
    }
}

fn get_session_key(pass: String) -> String {
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
    return session.to_owned();
}

fn get_items(session_key: String) -> Vec<Item> {
    // run bitwarden-cli's list!
    let bwitems = Command::new("bw")
        .arg("list")
        .arg("items")
        .arg("--session")
        .arg(session_key)
        .output()
        .expect("failed to execute process");

    // process stdout for json
    let listoutput = String::from_utf8(bwitems.stdout).unwrap();
    let itemsjson: Vec<Value> = serde_json::from_str(&listoutput).unwrap();

    // create Vec of Item struct from Vec<Value> (pretty hacky but it works)
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
    }
    return items;
}

fn select_item(s: &mut Cursive, _: &str) {
    let selectview = s.find_name::<SelectView>("select").unwrap();
    let selected_id = selectview.selected_id().unwrap();
    let s = selectview.get_item(selected_id);
    let x = s.unwrap();
    let id_str = format!("{}", x.1);
    cli_clipboard::set_contents(id_str).unwrap();
}
