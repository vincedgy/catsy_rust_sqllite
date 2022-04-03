use crate::show_cat_colors::show_cat_colors;
use crate::initdb::initdb;
use crate::populate::populate;

mod initdb;
mod populate;
mod show_cat_colors;

fn main() {
    println!("Starting");

    initdb().map_err(|err| println!("{:?}", err)).ok();
    populate().map_err(|err| println!("{:?}", err)).ok();    
    show_cat_colors().map_err(|err| println!("{:?}", err)).ok();
}