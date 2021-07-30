extern crate prettytable;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class};
use std::fs;

fn main() {
    scrape_kfc("https://kfccoupons.co.nz");
}

fn scrape_kfc(url: &str) {
    println!("Scraping");
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let document = Document::from_read(resp).unwrap();

    let mut file_string = String::new();

    for node in document.find(Class("grid-items")) {
        let price = node.find(Class("items-price")).next().unwrap().text();
        let title = node.find(Class("items-title")).next().unwrap().text();
        file_string += &title;
        file_string += " | ";
        file_string += &price;
        file_string += "\n";
    }

    println!("{}", file_string);
    fs::write("kfc_deals.txt", file_string).expect("Unable to write file");
}
