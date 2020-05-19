use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::banner;

pub fn parse_markdown_file(_filename: &str) {
    banner::print_short_banner();
    println!("[INFO] Trying to parse {}...", _filename);

    // Get filename with path
    let input_filename = Path::new(_filename);

    // Open file
    let file = File::open(&input_filename).expect("Couldn't open file");

    let mut _ptag: bool = false; // keep track of paragraph tags
    let mut _htag: bool = false; // keep track of h1 tags

    // Initialize tokens vector that will contain the final output.
    let mut tokens: Vec<String> = Vec::new();

    // Open file.
    let reader = BufReader::new(file);

    // Loop through lines in file.
    for line in reader.lines() {

        // Every line is a RESULT object. We unwrap in and store is as a string.
        let line_contents = line.unwrap().to_string();

        // Extract the first char.
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        // Initialize the string to store the result in.
        let mut output_line = String::new();


        // Pattern match against first_char to check wether it is a #.
        // If not, we treat it as a paragraph.
        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n")
                }
                if _htag {
                  _htag = false;
                  output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);

            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                  }

                output_line.push_str(&line_contents);
            }
        };

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    };

    // Loop through and print all parsed lines
    for t in &tokens {
        println!("{}", t);
    }

    // Exract output filename;
    let mut output_filename = String::from(&_filename[.._filename.len()-3]);
    output_filename.push_str(".html");
    println!("---------------------------");
    println!("{}", &output_filename);
    println!("---------------------------");

    let mut outfile = File::create(output_filename.to_string())
        .expect("[Error] Could not create file.");

    for line in &tokens {
      outfile.write_all(line.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");
    };

    println!("[ INFO ] Parsing complete!");

}
