pub struct ParsedResult {
    name: String,
    lines: u32,
    words: u32,
    chars: u32,
    bytes: u32
}

impl ParsedResult {
    pub fn parse(filename: String) -> ParsedResult {
        let mut result = ParsedResult {
            name: filename,
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0
        };

        todo!("You can't parse anything yet!");

        result
    }
}