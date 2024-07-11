use crate::{CliArgs, HighlightedText};

/// default text for the caption
pub const DEFAULT_CAPTION: &str = "CHANGE ME";
/// default text for the label
pub const DEFAULT_LABEL: &str = "CHANGE_ME";

impl HighlightedText {
    /// Returns string that is piece of formatted LaTeX code that represents highlighted text.
    fn as_str(&self, conf: &CliArgs) -> String {
        if self.text == "\n" {
            return "\n".to_string();
        }
        let mut mini_buffer: Vec<String> = vec![];
        // some nodes contain line breaks in the middle of a text and they need to be split and
        // formatted separately otherwise it will cause issues in pdf after rendering
        for text_piece in self.text.split('\n') {
            let color_name = &self.hex_color;
            let mut out = format!(
                "\\texttt{{{}}}",
                // the first 5 replacements are because we parse html (we need to un-escape)
                // the rest is necessary because LaTeX wouldn't compile otherwise
                text_piece
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&quot;", "\"")
                    .replace("&#39;", "'")
                    .replace("&amp;", "&")
                    .replace('\\', "\\textbackslash") // don't add the braces here yet
                    .replace('}', "\\}") // escape the braces
                    .replace('{', "\\{")
                    .replace("\\textbackslash", "\\textbackslash{}") // now add the empty braces to break the command without escaping them
                    .replace(' ', "\\ ")
                    .replace('\t', "\\ ".repeat(conf.tab_size).as_str())
                    .replace('_', "\\_")
                    .replace('^', " \\^") // the space in front is necessary otherwise it would add hat to letters
                    .replace('&', "\\&")
                    .replace('%', "\\%")
                    .replace('#', "\\#")
                    .replace('~', "\\~")
                    .replace('$', "\\$")
                    .replace('"', if conf.german { "\\dq{}" } else { "\"" })
            );

            // if there is some other additional highlight, wrap the text in it
            if self.bold {
                out = format!("\\textbf{{{}}}", out);
            }
            if self.underline {
                // underbar is better than underline in verbatim
                // underline causes spacing issues
                out = format!("\\underbar{{{}}}", out);
            }
            if self.italic {
                out = format!("\\textit{{{}}}", out);
            }
            out = format!("\\textcolor[HTML]{{{}}}{{{}}}", color_name, out);
            // wrap the text in the escaped sequence that will cause LaTeX to validate the string even inside verbatim
            out = format!("{}{}{}", conf.escape_start, out, conf.escape_end);
            mini_buffer.push(out);
        }
        return mini_buffer.join("\n");
    }
}

/// generates the LaTeX verbatim string that will be placed in the output file
pub fn generate_latex_verbatim(
    highlighted_text_pieces: Vec<HighlightedText>,
    conf: &CliArgs,
) -> String {
    let header = format!("\\begin{{lstlisting}}[
	% there are many more options of styling, see the official documentation, these are just the defaults I like
	frame=single, % make single-line frame around the verbatim
	framesep=2mm, % put some more spacing between the frame and text
	aboveskip=5mm, % put some more space above the box
	basicstyle={{\\linespread{{0.9}}\\small\\ttfamily}}, % use typewriter (monospace) font
	caption={{{}}}, % set the caption text
	captionpos=b, % put the caption at the bottom (b) or top (t) or both (bt)
    label={{{}}}, % label to be referenced via \\ref{{}}
	numbers=left, % line numbers on the left
	numberstyle={{\\scriptsize\\ttfamily\\color{{black!60}}}}, % the style for line numbers
	escapeinside={{{}}}{{{}}} % between those sequences are commands evaluated
]", conf.caption, conf.label, conf.escape_start, conf.escape_end);
    let footer = r"\end{lstlisting}";
    let mut buffer = "".to_string();
    if !conf.raw {
        buffer += &header;
        buffer += "\n";
    }
    for text_piece in highlighted_text_pieces {
        buffer += &text_piece.as_str(conf);
    }
    if !conf.raw {
        buffer += "\n"; // this is probably unnecessary blank line
        buffer += footer;
    }
    if conf.verbose {
        println!("Successfully constructed output string (LaTeX code).")
    }
    return buffer;
}
