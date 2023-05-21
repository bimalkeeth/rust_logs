#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused)]

mod advance;

use log::{error, info, warn};
use log4rs;

fn main() {
    println!("Hello, world!");
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();


    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");

    let result =(|x:i32| x * 2)(5);

    println!("{}",result);


    advance::channels::check_current_line();

    std::thread::sleep(std::time::Duration::from_secs(1));

}
