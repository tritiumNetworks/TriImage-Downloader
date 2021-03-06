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
    let tag = config.get("DEFAULT", "tag").clone().unwrap();
    let mut main = String::from("https://img.trinets.xyz/static/");
    main.push_str(&(tag.to_owned()+ "/"));
    let mut img =  String::from("https://img.trinets.xyz/api/v1?target=");
    img.push_str(&(tag.to_owned() + "&type=list"));
    let response =  requests::get(img).unwrap();
    let data = response.json().unwrap();
    let len:usize = data.len() - 1;

    let num = config.get("DEFAULT", "imgid").clone().unwrap();
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
        
            } else if choice== "all" {
                let down = thread::spawn(move || {
                    for l in 0..len {
                        let a = main.to_owned()+  &data[l].to_string();
                    
                        let filename = folder.to_owned() + &data[l].clone().to_string()  + ".jpg";
                        // download
                        let download = Download::new(&a,Some(&filename),None);
        
                        match download.download() {
                            Ok(_) => {
                                println!("downloaded");
                                thread::sleep(Duration::from_millis(1));
                            },
                            Err(e) => println!("Error ： {}",e.to_string()),
                        }
                    }
                });
                down.join().unwrap();
            }
        },
        Err(_err) => {
            let url = main.to_owned() + &num.to_string() + ".jpg";
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
        
            } else if choice== "all" {
                let down = thread::spawn(move || {
                    for l in 0..len {
                        let a = main.to_owned()+  &data[l].to_string();
                    
                        let filename = folder.to_owned() + &data[l].clone().to_string()  + ".jpg";
                        // download
                        let download = Download::new(&a,Some(&filename),None);
        
                        match download.download() {
                            Ok(_) => {
                                println!("downloaded");
                                thread::sleep(Duration::from_millis(1));
                            },
                            Err(e) => println!("Error ： {}",e.to_string()),
                        }
                    }
                });
                down.join().unwrap();
            }
        }
    };
}

