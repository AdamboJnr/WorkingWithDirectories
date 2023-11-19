extern crate rmdir;

use std::env;
use std::process;

fn main() {
   let args: Vec<String> = env::args().collect();
   
   let dir =  rmdir::CreateDir::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing arguments: {}",err);
        process::exit(1);
   });

   if let Err(e) = rmdir::run(&dir) {
      println!("Application Error: {}", e);

      process::exit(1)
   };

}

