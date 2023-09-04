mod file_searcher;

use file_searcher::find;
use regex::Regex;
// use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::process;

extern crate clap;

use clap::{Arg, App};

fn main() {
    // 使用clap库解析命令行参数
    let matches = App::new("File Search")
        .version("0.1.0")
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

    let pattern = matches.value_of("pattern").unwrap();
    let regex = match  Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Invalid regular expression '{}': '{}'", pattern, err);
            process::exit(1);
        }
    };

    let verbose = matches.is_present("verbose"); verbose = matches.is_present("verbose");

    let tar_file = matches.value_of("output");

    let dir = matches.value_of("directory").unwrap();
    match find(dir, &regex) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("Unfind matches.");
            } else {
                if verbose {
                    println!("Find: ");
                    for file in &matches {
                        println!("{}", file);
                    }
                }

                if let Some(output_file) =  tar_file{
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

fn write_matches_to_file(matches: &[String], output_file: &str) -> io::Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    for match_ in matches {
        writeln!(writer, "{}", match_)?;
    }

    writer.flush()?;
    Ok(())
}