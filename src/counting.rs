use crate::parsing::WordCountModifiers;

///
/// Holds information about the word count of text
///
#[derive(Debug)]
pub struct ParsedResult<'a> {
    filepath: Option<&'a str>,
    lines: usize,
    words: usize,
    chars: usize,
}

impl<'a> ParsedResult<'_> {
    ///
    /// Parse a file into text and then parse the text
    ///
    pub fn parse_file(filepath: &'a str) -> ParsedResult {
        let text = std::fs::read_to_string(&filepath).unwrap();
        ParsedResult::parse_lines(text, Some(filepath))
    }

    ///
    /// Parse text into word count information
    ///
    pub fn parse_lines(text: String, filepath: Option<&'a str>) -> ParsedResult<'a> {
        let (lines, words, chars) = text
            .lines()
            .map(|l| {
                let words = l.split_ascii_whitespace().count();
                let chars = l.chars().count();
                (1, words, chars)
            })
            .fold((0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));

        ParsedResult {
            filepath,
            lines,
            words,
            chars,
        }
    }

    ///
    /// Merge the results of multiple ParsedResults into one
    ///
    pub fn totals(results: &Vec<ParsedResult>) -> ParsedResult<'a> {
        let (lines, words, chars) = results.iter().fold((0, 0, 0), |a, b| {
            (a.0 + b.lines, a.1 + b.words, a.2 + b.chars)
        });

        ParsedResult {
            filepath: Some("total"),
            lines,
            words,
            chars,
        }
    }

    pub fn print(&self, mods: &WordCountModifiers) {
        let all = !mods.lines && !mods.words && !mods.chars;

        let l = match mods.lines || all {
            true => {
                format!("{:<10} ", self.lines)
            }
            false => "".to_string(),
        };

        let w = match mods.words || all {
            true => {
                format!("{:<10} ", self.words)
            }
            false => "".to_string(),
        };

        let c = match mods.chars || all {
            true => {
                format!("{:<10} ", self.chars)
            }
            false => "".to_string(),
        };

        let name = self.filepath.unwrap_or("");

        println!("{}{}{}{}", l, w, c, name)
    }
}
