#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused)]

use std::any::type_name;
use std::sync::mpsc::channel;
use std::thread;
use actix::prelude::*;
use std::time::Duration;
use std::time::Instant;
use futures::future::Future;
use futures::future;

use std::io::stdin;

pub fn send_channel() {
    let (tx, rx) = channel();

    let join_handle = thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("received {}", n);
        }
    });

    for i in 0..10 {
        tx.send(i).unwrap();
    }

    join_handle.join().unwrap();
}


macro_rules! scanline {
    ($x:expr)=>({
        stdin().read_line(&mut $x).unwrap();
        $x.trim();
    });

}

pub fn macro_test() {
    let mut input = String::new();
    scanline!(input);
    println!("I read: {:?}", input);

    let d = dbg!(&input);

    println!("{}",d);

    let ss=option_env!("GOROOT");

    println!("{}",ss.unwrap());
}

macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

pub fn check_current_line(){
    let function_name = function_name!();
    let file = file!();
    let line = line!();
    let column = column!();
    let module_path = module_path!();

    println!("Function name: {}", function_name);
    println!("Source file: {}", file);
    println!("Line number: {}", line);
    println!("Column: {}", column);
    println!("Module path: {}", module_path);
}