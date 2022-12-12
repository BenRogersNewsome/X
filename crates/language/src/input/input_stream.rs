pub struct InputStream {
    source: Vec<u8>,
    pos: usize,
    line: usize,
    col: usize,

    pub start: usize,
}

impl Iterator for InputStream {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.source.len() {
            return None;
        };
        
        let last_char = self.source[self.pos];
        self.pos += 1;
        if last_char == b'\n' {
            self.col = 0;
            self.line += 1;
        }else{
            self.col += 1;
        }
        return Some(last_char);
    }
}

impl InputStream {

    pub fn new(source: &[u8]) -> InputStream {
        InputStream {
            source: source.to_vec(),
            pos: 0,
            line: 0,
            col: 0,
            start: 0,
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.pos == self.source.len() {
            return None;
        };
        Some(self.source[self.pos])
    }

    pub fn skip(&mut self) {
        self.pos += 1;
        self.start = self.pos;
    }
    
    pub fn get(&mut self) -> &[u8] {
        let chars = &self.source[self.start .. self.pos];
        self.start = self.pos;
        return chars;
    }
}