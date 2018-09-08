use encoding_rs::{Decoder, UTF_16LE, UTF_8};
use std::io;
use std::io::{BufRead, Read};

/// Reads text files (any reasonable encoding) a line at a time, returning the line as UTF-8.
pub struct Utf8LineReader<T: Read + BufRead> {
    reader: T,
    decoder: Decoder,
    line: String,
}

impl<T: Read + BufRead> Utf8LineReader<T> {
    /// Creates a Utf8LineReader.
    pub fn new(mut reader: T) -> io::Result<Utf8LineReader<T>> {
        let mut bom = [0; 4];
        reader.read(&mut bom)?;

        let mut line = String::with_capacity(1024);

        // Create a decoder and prime it by trying to read a BOM.
        let mut decoder = UTF_8.new_decoder();
        let (_result, _size, _had_errors) = decoder.decode_to_string(&bom[..], &mut line, false);

        Ok(Utf8LineReader {
            reader,
            decoder,
            line,
        })
    }

    /// Reads a single line from the file, returning true if there are no lines remaining.
    fn read_single_line(&mut self) -> io::Result<bool> {
        let mut buf = vec![];

        // Read up to a line feed or the end of the file, whichever is sooner.
        let mut done = false;
        while !done {
            let num_bytes = self.reader.read_until(0x0a, &mut buf)?;
            if num_bytes == 0 || self.decoder.encoding() != UTF_16LE {
                done = true;
            } else {
                // If it's UTF16_LE then we need one more byte to determine if we have a line feed, ie, U+000a, and not,
                // for instance, U+020a (Ȋ - latin capital I with inverted breve).
                let mut b = [0; 1];
                self.reader.read(&mut b)?;
                buf.push(b[0]);
                done = b[0] == b'\0';
            }
        }

        // If we didn't read anything into the buffer then we're truly done.
        if buf.len() == 0 {
            return Ok(true);
        }

        // We have a buffer, so decode it to a string (which is UTF-8).
        let (_result, _size, _had_errors) =
            self.decoder
                .decode_to_string(&buf[..], &mut self.line, false);

        // Trim the trailing LF and CR if present so that we're left with just the line.
        if self.line.ends_with("\n") {
            self.line.pop();
            if self.line.ends_with("\r") {
                self.line.pop();
            }
        }

        Ok(false)
    }
}

impl<T: Read + BufRead> Iterator for Utf8LineReader<T> {
    type Item = io::Result<String>;

    /// Advances the Utf8LineReader and returns the next line.
    fn next(&mut self) -> Option<Self::Item> {
        let last_line = self.read_single_line();
        match last_line {
            Ok(true) => None,
            Ok(false) => {
                let result = self.line.clone();
                self.line.clear();
                Some(Ok(result))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
