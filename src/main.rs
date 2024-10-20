use std::env::vars_os;
use std::error::Error;
use std::io::{stdout, Stdout};
use std::path::Path;
use std::process::exit;
use clap::Parser;
use log::error;
use rstype::database::{fetch_text_with_id, load_text_from_database, load_text_from_database_based_on_difficulty, load_text_from_database_with_random_difficulty};
use rstype::{load_text_from_file, PreparedText};
use rstype::app::App;

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(short, long, action)]
    /// Show rstype version
    version: bool,
    #[clap(short, long, value_name="FILENAME")]
    /// File to use text from as sample text
    file: Option<String>,
    #[clap(short, long, value_name="id")]
    /// ID to retrieve text from database
    id: Option<u32>,
    #[clap(short, long, value_name="N", default_value="2")]
    /// Choose difficulty withing range 1-5
    difficulty: Option<u32>,
    #[clap(short='H', long, action)]
    /// Show rstype score history
    history: bool
}


fn main() {
    let args = Arguments::parse();

    // Start the parser
    let prepared_text = resolve_command_line_args(args);

    let mut app = App::from_prepared_text(prepared_text);
    
    let window = pancurses::initscr();
    pancurses::start_color();
    window.refresh();
    app.main(&window);
}

fn resolve_command_line_args(args: Arguments) -> PreparedText {
    let database_file = "data.db";
    let prepared_text: PreparedText = if args.version {
        println!("Rstype version 0.1.0");
        exit(0)
    } else if args.history {
        todo!("History not implemented yet");
    } else if let Some(file_path) = args.file {
        load_text_from_file(file_path)
    } else if let Some(id) = args.id {
        load_text_from_database(id, database_file)
    } else if let Some(difficulty) = args.difficulty {
        load_text_from_database_based_on_difficulty(difficulty, database_file)
    } else {
        load_text_from_database_with_random_difficulty(database_file)
    }.unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    });
    prepared_text
}


