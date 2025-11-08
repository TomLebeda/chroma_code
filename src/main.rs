//! main file, everything starts here

/// output formatting
mod formatter;

/// tree-sitter's output html parsing
mod parser;

/// interactive-mode validation of user inputs
mod validator;

// my local imports
use formatter::generate_latex_verbatim;
use parser::{extract_highlighted_pieces, parse_header};
use validator::{is_input_ok, is_output_ok};

// external imports
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(
    author = "Tomáš Lebeda <tom.lebeda@gmail.com>",
    version = "1.1.0",
    about = "Generate LaTeX for highlighted code listings with the power of TreeSitter."
)]

/// command line arguments
pub struct CliArgs {
    /// Input file with the code that should be highlighted.
    #[arg(short, long)]
    pub input: Option<String>,

    /// Output file where generated LaTeX code will be stored.
    #[arg(short, long)]
    pub output: Option<String>,

    /// String that marks start of evaluation inside LaTeX's verbatim environment.
    #[arg(long, default_value_t = String::from("<@"))]
    pub escape_start: String,

    /// String that marks end of evaluation inside LaTeX's verbatim environment.
    #[arg(long, default_value_t = String::from("@>"))]
    pub escape_end: String,

    /// Tab size as a number of spaces
    #[arg(long, default_value_t = 4)]
    pub tab_size: usize,

    /// If enabled, the output will not be wrapped by verbatim environment, it will be only the "raw insides".
    #[arg(short, long)]
    pub raw: bool,

    /// Generate necessary directories and/or files on disk and overwrite anything that stands in the way.
    #[arg(short, long)]
    pub force: bool,

    /// Disable the interactive fallback and fail if some inputs are wrong or missing.
    #[arg(short, long)]
    pub trust: bool,

    /// Print more detailed information while running.
    #[arg(short, long)]
    pub verbose: bool,

    /// Print the final text also into stdout
    #[arg(short, long)]
    pub dump: bool,

    /// Escape double quotes `"` as `\dq{}` for users of babel with [ngerman]
    #[arg(short, long)]
    pub german: bool,

    /// Use the provided string as a caption for the listing.
    /// Can be overridden by a header in the input file (see `header-comment-types`).
    #[arg(short, long, default_value_t = String::from(formatter::DEFAULT_CAPTION))]
    pub caption: String,

    /// Use the provided string as a label for the listing.
    /// Can be overridden by a header in the input file (see `header-comment-types`).
    #[arg(short, long, default_value_t = String::from(formatter::DEFAULT_LABEL))]
    pub label: String,

    /// Skip the output file argument and use the input file with the extension swapped to .tex
    #[arg(long)]
    pub swap_ext: bool,

    /// Comma-separated list of comment prefixes to check for a header on the first line of the input file.
    /// A header can set the caption and/or label.
    /// Example: `# chroma_code: caption: My Caption label: my-label`
    #[arg(long, default_value_t = String::from("#,//"))]
    pub header_comment_types: String,

    /// default color for the text
    #[arg(long, default_value_t = String::from("000000"))]
    pub default_color: String,

    /// if true, the first line of the input file will be skipped
    #[arg(skip)]
    pub skip_first_line: bool,
}

#[derive(Debug)]
pub struct HighlightedText {
    /// the text that is highlighted
    text: String,
    /// hexadecimal value of the font color
    hex_color: String,
    /// if true, the text will be bold
    bold: bool,
    /// if true, the text will be underlined
    underline: bool,
    /// if true, the text will be italic
    italic: bool,
}

/// utility function for handling user input in interactive mode
fn wait_for_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim().to_string();
        }
        Err(_) => {
            // I don't even know what might cause this branch to happen
            println!("Sorry, input reading failed, this shouldn't normally happen.");
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
}

fn main() {
    let mut conf = CliArgs::parse();
    let cwd = std::env::current_dir().unwrap_or_else(|_| {
        println!("Sorry, can't get current working directory.");
        println!("Something went really wrong, this can only happend when the current directory doesn't exist or you don't have permission to read it.");
        println!("Can't continue further, exiting.");
        std::process::exit(exitcode::UNAVAILABLE);
    });
    let cwd = cwd.as_path();

    if !conf.trust {
        // this loop is interactive way of getting and validating input and output files from user
        while !is_input_ok(&mut conf, cwd) {
            continue;
        }
        if !conf.swap_ext {
            while !is_output_ok(&mut conf, cwd) {
                continue;
            }
        }
    }

    if conf.swap_ext && conf.output.is_none() {
        if let Some(input_path_str) = &conf.input {
            let input_path = Path::new(input_path_str);
            let output_path = input_path.with_extension("tex");
            conf.output = Some(output_path.to_str().unwrap().to_string());
            if output_path.exists() {
                if !conf.force {
                    println!("Output file already exists, use --force to overwrite it.");
                    std::process::exit(exitcode::UNAVAILABLE);
                } else {
                    println!("Output file already exists, overwriting it.");
                }
            }
        }
    }

    let Some(input_file) = &conf.input.clone() else {
        println!("Can't continue without input file. You probably used \"--trust\" without any input file.");
        std::process::exit(exitcode::USAGE)
    };
    let Some(output_file) = &conf.output else {
        println!("Can't continue without output file. You probably used \"--trust\" without any output file, or you used \"--swap-ext\" without providing an input file.");
        std::process::exit(exitcode::USAGE)
    };

    let content = std::fs::read_to_string(input_file).unwrap_or_else(|err| {
        println!(
            "Sorry, there was an error while reading the input file: {}",
            err
        );
        std::process::exit(exitcode::NOINPUT);
    });

    if let Some(header_info) = parse_header(
        &content,
        &conf.header_comment_types.split(",").collect::<Vec<&str>>(),
    ) {
        println!("Using info from header: {:?}", header_info);

        conf.caption = header_info.caption.unwrap_or(conf.caption);
        conf.label = header_info.label.unwrap_or(conf.label);
        conf.skip_first_line = true;
    }

    let command = "tree-sitter";
    let args = ["highlight", "-H", input_file];
    let cmd_output = std::process::Command::new(command).args(args).output();
    if conf.verbose {
        println!("executing command \"{} {}\"", command, args.join(" "))
    }
    match cmd_output {
        Ok(out) => {
            if !out.status.success() {
                println!(
                    "Sorry, the command \"{} {}\" did not succeed and returned status {}.",
                    command,
                    args.join(" "),
                    out.status.code().unwrap_or(-1)
                );
                println!(
                    "Captured stderr: {}",
                    std::str::from_utf8(&out.stderr).unwrap_or("[nothing was captured]")
                );
                std::process::exit(exitcode::NOINPUT);
            } else if !out.stderr.is_empty() {
                let err_msg = String::from_utf8(out.stderr).unwrap_or(String::from("???"));
                if err_msg.contains("No language found for") {
                    println!(
                        "Sorry, TreeSitter couldn't find language parser for the file you entered."
                    );
                    println!("You can inspect all currently usable parsers by executing 'tree-sitter dump-languages'.");
                    println!("Parsers can be downloaded from github.");
                    println!("For more information, please refer to the official TreeSitter highlight documentation at 'https://tree-sitter.github.io/tree-sitter/'.");
                    std::process::exit(exitcode::UNAVAILABLE);
                } else {
                    println!(
                    "WARNING: The command \"{} {}\" finished, but returned error message: \"{}\". ",
                    command,
                    args.join(" "),
                    err_msg
                );
                    println!("The output will probably be not correct.");
                    println!("This is probably not an issue of this program, but rather the external TreeSitter call.");
                }
            }
            if conf.verbose {
                println!(
                    "Successfully called TreeSitter, received {} bytes of data.",
                    out.stdout.len()
                );
            }

            let html_bytes: Vec<u8> = out.stdout;
            let highlighted_text_pieces = extract_highlighted_pieces(html_bytes, &conf);
            let generated_latex = generate_latex_verbatim(highlighted_text_pieces, &conf);
            if conf.dump {
                println!("{}", generated_latex);
            }
            if let Err(write_err) = std::fs::write(output_file, generated_latex) {
                println!(
                    "Sorry, there was error while trying to write the output file: {}",
                    write_err
                );
            } else if conf.verbose {
                println!("Successfully written output to \"{}\".", output_file);
            };
        }
        Err(e) => {
            println!(
                "Sorry, the required command \"{} {}\" failed with error message:\n{}.",
                command,
                args.join(" "),
                e
            );
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
}
