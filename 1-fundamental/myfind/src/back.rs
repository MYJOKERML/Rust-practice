// mod file_searcher;

// use file_searcher::find;
// use regex::Regex;
// use std::env;
// use std::process;

// // extern crate clap;

// // use clap::{Arg, App};

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 3 {
//         eprintln!("参数小于3个，使用方式: {} <目录目标> <要搜索的正则表达式>", args[0]);
//         process::exit(1);
//     }
//     else if args.len() > 3{
//         eprintln!("参数大于3个，目前只支持3个参数\n使用方式: {} <目录目标> <要搜索的正则表达式>", args[0]);
//         process::exit(1);
//     }
//     let pattern = &args[2];
//     let regex = match  Regex::new(pattern) {
//         Ok(re) => re,
//         Err(err) => {
//             eprintln!("Invalid regular expression '{}': '{}'", pattern, err);
//             process::exit(1);
//         }
//     };

//     match find(&args[1], &regex) {
//         Ok(matches) => {
//             if matches.is_empty() {
//                 println!("Unfind matches.");
//             } else {
//                 println!("Find: ");
//                 for file in matches {
//                     println!("{}", file);
//                 }
//             }
//         }
//         Err(error) => {
//             eprintln!("ERROR: {}", error);
//             process::exit(1);
//         }
//     }
// }

