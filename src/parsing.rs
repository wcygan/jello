use clap::ArgMatches;

pub struct WordCountModifiers {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool
}

impl WordCountModifiers {
    pub fn new(matches: &ArgMatches) -> WordCountModifiers {
        WordCountModifiers {
            lines: matches.occurrences_of("lines") > 0,
            words: matches.occurrences_of("words") > 0,
            chars: matches.occurrences_of("chars") > 0,
            bytes: matches.occurrences_of("bytes") > 0
        }
    }
}