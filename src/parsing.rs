use clap::ArgMatches;

///
/// Determines how to print out word count info
///
pub struct WordCountModifiers {
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
}

impl WordCountModifiers {
    pub fn new(matches: &ArgMatches) -> WordCountModifiers {
        WordCountModifiers {
            lines: matches.occurrences_of("lines") > 0,
            words: matches.occurrences_of("words") > 0,
            chars: matches.occurrences_of("chars") > 0,
        }
    }
}