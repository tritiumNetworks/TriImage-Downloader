extern crate requests;
extern crate webbrowser;

use requests::ToJson;
use download_rs::sync_download::Download;
use std::io::{stdin,stdout,Write};

fn main()  {
    let tag = "neko";
    let mut main = String::from("https://img.trinets.xyz/static/");
    main.push_str(&(tag.to_owned()+ "/"));
    let mut img =  String::from("https://img.trinets.xyz/api/v1?target=");
    img.push_str(&(tag.to_owned() + "&type=list"));
    let response =  requests::get(img).unwrap();
    let data = response.json().unwrap();
    // (args[0], view, download) (args[1], img_number)

    let mut a = String::new();
    print!("view or download: ");
    let _= stdout().flush();
    stdin().read_line(&mut a).expect("Error");

    //number or imageid (ex. 114fc734bb)
    let mut s = String::new();
    print!("Please enter some number or string: ");
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Error");

    let num= s.trim();
    let number:usize = match num.trim().parse() {
        Ok(a) => a,
        _ => unreachable!(),
    };
    let url = main.to_owned() + &data[number].clone().to_string();
    let folder = "./tests/";
    let filename = folder.to_owned() + &data[number].clone().to_string();

    if a.to_lowercase().trim() == "view" {
    // view
    let _view = webbrowser::open(&url.to_string());

    } else if a.to_lowercase().trim() == "download" {
    // download
    let download = Download::new(&url,Some(&filename),None);

    match download.download() {
        Ok(_) => println!("downloaded"),
        Err(e) => println!("Error ： {}",e.to_string()),
    }
}
}