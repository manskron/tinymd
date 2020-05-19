mod parser;
mod banner;



fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => parser::parse_markdown_file(&args[1]),
        _ => {
            println!("\n[ ERROR ] Please supple a valid markdown file as argument\n");
            banner::print_long_banner();
        }
    }
}
