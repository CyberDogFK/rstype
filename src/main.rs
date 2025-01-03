use clap::Parser;
use rstype::app::App;
use rstype::database::{
    load_text_from_database, load_text_from_database_based_on_difficulty,
    load_text_from_database_with_random_difficulty,
};
use rstype::{exit, load_text_from_file, AppError, AppResult, PreparedText};
use rstype::history::{show_history, NumberOfRecords};

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(short, long, action)]
    /// Show rstype version
    version: bool,
    #[clap(short, long, value_name = "FILENAME")]
    /// File to use text from as sample text
    file: Option<String>,
    #[clap(short, long, value_name = "id")]
    /// ID to retrieve text from database
    id: Option<u32>,
    #[clap(short, long, value_name = "N", default_value = "2")]
    /// Choose difficulty withing range 1-5
    difficulty: Option<u32>,
    #[clap(short = 'H', long, default_missing_value="0", require_equals = false, num_args=0..=1)]
    /// Show rstype score history
    history: Option<u32>,
}

fn main() {
    let args = Arguments::parse();

    if let Err(e) = run_app_with_args(args) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run_app_with_args(args: Arguments) -> AppResult<()> {
    // Start the parser
    let prepared_text = resolve_command_line_args(args)?;

    let mut app = App::from_prepared_text(prepared_text);

    let window = pancurses::initscr();
    pancurses::start_color();
    window.refresh();
    app.run(&window)
}

fn resolve_command_line_args(args: Arguments) -> Result<PreparedText, AppError> {
    let database_file = "data.db";
    let prepared_text: PreparedText = if args.version {
        println!("Rstype version 0.1.0");
        exit(0)
    } else if let Some(history) = args.history {
        let number_of_records = match history {
            0 => NumberOfRecords::All,
            _ => NumberOfRecords::Last(history as usize),
        };
        show_history(number_of_records)?;
        exit(0)
    } else if let Some(file_path) = args.file {
        load_text_from_file(file_path).unwrap()
    } else if let Some(id) = args.id {
        load_text_from_database(id, database_file)?
    } else if let Some(difficulty) = args.difficulty {
        load_text_from_database_based_on_difficulty(difficulty, database_file)?
    } else {
        load_text_from_database_with_random_difficulty(database_file)?
    };
    Ok(prepared_text)
}
