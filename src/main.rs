extern crate requests;
extern crate webbrowser;

use configparser::ini::Ini;
use requests::ToJson;
use download_rs::sync_download::Download;
use std::thread;
use std::time::Duration;

fn main() {
    let mut config = Ini::new();
    let _map = config.load("settings.ini");
    let _tags = vec!["neko", "neko-r19", "hentai", "hentai-gif"];
    //print!("{}", _tags[0]);
    let tag = config.get("DEFAULT", "tag").clone().unwrap();
    let mut main = String::from("https://img.trinets.xyz/static/");
    main.push_str(&(tag.to_owned()+ "/"));
    let mut img =  String::from("https://img.trinets.xyz/api/v1?target=");
    img.push_str(&(tag.to_owned() + "&type=list"));
    let response =  requests::get(img).unwrap();
    if  response.is_json() == false {
        return;
    }
    let data = response.json().unwrap();
    let len:usize = data.len() - 1;
    // (args[0], view, download) (args[1], img_number)
    //number or imageid (ex. 114fc734bb)
    /* let num= s.trim();
    let number:usize =  num.trim().parse().unwrap_or(0); */
    let num = config.get("DEFAULT", "number").clone().unwrap();
    match num.trim().parse::<usize>() {
        Ok(a) => {
            let url = main.to_owned() + &data[a].clone().to_string();
            let folder: String = config.get("DEFAULT", "folder").clone().unwrap();
            let filename = folder.to_owned() + &data[a].clone().to_string();

            let choice = config.get("DEFAULT", "choice").clone().unwrap().trim().to_lowercase();

            if choice  == "view" {
                // view
                let _view = webbrowser::open(&url.to_string());
        
            } else if choice == "download" {
                // download
                let download = Download::new(&url,Some(&filename),None);
        
                match download.download() {
                    Ok(_) => println!("downloaded"),
                    Err(e) => println!("Error ： {}",e.to_string()),
                }
        
            } else if choice== "all" {}
        },
        Err(_err) => {
            let url = main.to_owned() + &num.to_string();
            let folder = config.get("DEFAULT", "folder").clone().unwrap();
            let filename = folder.to_owned() + &num.to_string() + ".jpg";
            
            let choice = config.get("DEFAULT", "choice").clone().unwrap().trim().to_lowercase();

            if choice  == "view" {
                // view
                let _view = webbrowser::open(&url.to_string());
        
            } else if choice == "download" {
                // download
                let download = Download::new(&url,Some(&filename),None);
        
                match download.download() {
                    Ok(_) => println!("downloaded"),
                    Err(e) => println!("Error ： {}",e.to_string()),
                }
        
            } else if choice== "all" {}
        }
    };
}

