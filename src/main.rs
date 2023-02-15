#![allow(unused_variables)]

use console::style;
use std::env;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

mod sync;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let sheet_path_arg = args.get(1);
    let sheet_extension_arg = args.get(2);
    let output_path_arg = args.get(3);
    let output_verbose = args.get(4);

    if sheet_path_arg == None || output_path_arg == None || sheet_extension_arg == None {
        println!("Invalid Args, Requires 1)-->ExcelWorkbook.Path 2)-->extension .ods .xlsx 3)-->OutputFolder.Path");
        pause();
    } else {
        let mut verbose = false;
        if let Some(arg) = output_verbose {
            if arg == "--verbose" {
                verbose = true;
            }
        };

        //let temp_input  = String::from("G:/REPOS/minute-mayhem/design/appendix/");
        //let temp_ext    = String::from(".xlsx");
        //let temp_output = String::from("G:/REPOS/minute-mayhem/MinuteFun_Unity/Assets/DataSynced");

        let started = Instant::now();
        //match sync::start(temp_input, temp_ext, temp_output, false)
        match sync::start(
            String::from(sheet_path_arg.unwrap()),
            String::from(sheet_extension_arg.unwrap()),
            String::from(output_path_arg.unwrap()),
            verbose,
        ) {
            Ok(success) => {
                println!(
                    "{} ({}ms)",
                    style("Completed!").green().bold(),
                    started.elapsed().as_millis()
                );
            }
            Err(error) => {
                println!(
                    "{}",
                    style(format!("Main -> Final Error: {}", error))
                        .red()
                        .bold()
                );
            }
        }
        pause();
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[test]
fn test_main() {
    let temp_input = String::from("Sheets/");
    let temp_ext = String::from(".xlsx");
    let temp_output = String::from("Output/");

    let started = Instant::now();
    match sync::start(temp_input, temp_ext, temp_output, false) {
        Ok(success) => {
            println!(
                "{} ({}ms)",
                style("Completed!").green().bold(),
                started.elapsed().as_millis()
            );
        }
        Err(error) => {
            panic!(
                "{}",
                style(format!("Main -> Final Error: {}", error))
                    .red()
                    .bold()
            );
        }
    }
}

#[test]
fn test_current_game() {
    // let temp_input = String::from("C:/REPOS/ktai-books/Design");
    // let temp_ext = String::from(".xlsx");
    // let temp_output = String::from("C:/REPOS/ktai-books/Assets/DataSynced");

    // let started = Instant::now();
    // match sync::start(temp_input, temp_ext, temp_output, false) {
    //     Ok(success) => {
    //         println!(
    //             "{} ({}ms)",
    //             style("Completed!").green().bold(),
    //             started.elapsed().as_millis()
    //         );
    //     }
    //     Err(error) => {
    //         panic!(
    //             "{}",
    //             style(format!("Main -> Final Error: {}", error))
    //                 .red()
    //                 .bold()
    //         );
    //     }
    // }
}
