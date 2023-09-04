mod file_searcher;
mod writer;

use writer::write_matches_to_file;

use file_searcher::find;
use regex::Regex;
// use std::env;
use std::process;

extern crate clap;

use clap::{Arg, App};

fn main() {
    // 使用clap库解析命令行参数
    let matches = App::new("File Search")
        .version("1.0")
        .author("ljy")
        .about("Search for files using regular expressions")
        .arg( // 搜索目录，必须填写
            Arg::with_name("directory")
                .help("The directory to search")
                .required(true)
                .index(1),
        )
        .arg(   // 搜索的正则表达式，必须填写
            Arg::with_name("pattern")
                .help("The regular expression pattern to search for in file names")
                .required(true)
                .index(2),
        )
        .arg( // -o 输出文件，可选
            Arg::with_name("output")
                .help("Sets the output file to use")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg( // -v 输出遍历的每一个文件，可选
            Arg::with_name("verbose")
                .help("Print verbose output")
                .short("v")
                .long("verbose"),
        )
        .get_matches();

    // let num_args = matches.args.len();
    // 获取命令行参数并解析程序名称
    // let program_name = env::args()
    //     .next()
    //     .unwrap_or("MyApp".to_string()); // 默认为"MyApp"，如果找不到程序名称则使用默认值

    // 转换为正则表达式
    let pattern = matches.value_of("pattern").unwrap();
    let regex = match  Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Invalid regular expression '{}': '{}'", pattern, err);
            process::exit(1);
        }
    };

    let verbose = matches.is_present("verbose");

    let tar_file = matches.value_of("output");

    // 在指定目录下搜索文件
    let dir = matches.value_of("directory").unwrap();
    match find(dir, &regex, &verbose) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("Unfind matches.");
            } else {
                println!("Find: ");
                for file in &matches {
                    println!("{}", file);
                }
                // 如果指定了输出文件，则将结果写入文件
                if let Some(output_file) =  tar_file {
                    if let Err(err) = write_matches_to_file(&matches, output_file) {
                        eprintln!("Failed to write matches to file: {}", err);
                        process::exit(1);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            process::exit(1);
        }
    }
}

