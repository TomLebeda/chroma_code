use crate::{CliArgs, HighlightedText};
use regex::Regex;
use scraper::{Html, Selector};

pub fn extract_highlighted_pieces(stdout: Vec<u8>, conf: &CliArgs) -> Vec<HighlightedText> {
    let mut colored_text_pieces: Vec<HighlightedText> = vec![];
    let Ok(out_str) = String::from_utf8(stdout) else {
        println!("Sorry, couldn't create string from captured stdout.\n Raw (bytes) stdout:\n");
        std::process::exit(exitcode::DATAERR);
    };
    if conf.verbose {
        println!("Successfully converted received bytes into string.");
    }
    let document = Html::parse_document(&out_str);
    // following line is hard-coded tested selector, so unwrap() should never panic here
    let line_class_selector = Selector::parse("td.line").unwrap();
    let lines = document.select(&line_class_selector);
    // following line is hard-coded tested regex for hex-color used in the html, so unwrap() should never panic here
    let hex_color_regex = Regex::new(r"#[0-9a-fA-F]{6}").unwrap();
    for line in lines {
        for child in line.descendants() {
            let node = child.value();
            if node.is_text() {
                let node_text = &node.as_text().unwrap().text;
                let Some(parent) = child.parent() else {
                    continue;
                };
                let Some(parent_element) = parent.value().as_element() else {
                    // text node should always have parent element that defines the style, 
                    // but just in case, there will be default black style here
                    let text_piece = HighlightedText {
                        text: node_text.to_string(),
                        hex_color: String::from("000000"),
                        bold: false,
                        underline: false,
                        italic: false,
                    };
                    colored_text_pieces.push(text_piece);
                    continue;
                };
                // if there is no style in the parent node, use black color as default
                let style_text = parent_element.attr("style").unwrap_or("color: #000000");
                let capture = hex_color_regex.find(style_text);
                let parsed_color = match capture {
                    None => String::from("000000"), // again, use black if no match is found
                    Some(capture) => capture.as_str().replace('#', "").to_uppercase(),
                };
                let text_piece = HighlightedText {
                    text: node_text.to_string(),
                    hex_color: parsed_color,
                    bold: style_text.contains("bold"),
                    underline: style_text.contains("underline"),
                    italic: style_text.contains("italic"),
                };
                colored_text_pieces.push(text_piece);
            }
        }
    }
    if conf.verbose {
        println!(
            "Successfully parsed the TreeSitter's html into {} nodes.",
            colored_text_pieces.len()
        );
    }
    return colored_text_pieces;
}
