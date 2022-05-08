pub struct InputStream<'a> {
    source: &'a[u8],
    pos: usize,
    line: usize,
    col: usize,

    pub start: usize,
}

impl InputStream<'_> {

    pub fn new(source: &[u8]) -> InputStream {
        InputStream {
            source,
            pos: 0,
            line: 0,
            col: 0,
            start: 0,
        }
    }

    pub fn next(&mut self) -> u8 {
        let last_char = self.source[self.pos];
        self.pos += 1;
        if last_char == b'\n' {
            self.col = 0;
            self.line += 1;
        }else{
            self.col += 1;
        }
        return last_char;
    }

    pub fn peek(&mut self) -> u8 {
        self.source[self.pos]
    }

    pub fn skip(&mut self) {
        self.pos += 1;
        self.start = self.pos;
    }

    pub fn is_end(&mut self) -> bool {
        self.pos == self.source.len()
    }

    pub fn get(&mut self) -> &[u8] {
        let chars = &self.source[self.start .. self.pos];
        self.start = self.pos;
        return chars;
    }
}