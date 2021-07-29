#[macro_use]
extern crate prettytable;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use prettytable::Table;
use std::fs;

fn main() {
    scrape_kfc("https://kfccoupons.co.nz");
}

fn scrape_kfc(url: &str) {

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    let mut table = Table::new();
    // let mut file_string = "Today's KFC Deals\n".to_string();
    let mut file_string = String::new();

    for node in document.find(Class("grid-items")) {
        let price = node.find(Class("items-price")).next().unwrap().text();
        let title = node.find(Class("items-title")).next().unwrap().text();
        // price.what_is_it();
        // title.what_is_it();
        table.add_row(row![FdBybl->title]);
        // println!("{}", file_string);
        file_string += &title;
        file_string += " | ";
        file_string += &price;
        file_string += "\n";
        // table.add_row(row![FdBybl->price]);

    }
    println!("{}",file_string);
    // let mut file = File::create("kfc_deals.txt")?;
    // file.write_all(file_string);
    fs::write("kfc_deals.txt", file_string).expect("Unable to write file");
    // table.printstd();
}
