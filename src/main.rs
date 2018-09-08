extern crate chrono;
extern crate encoding_rs;

use log_file_reader::LogFileReader;
use log_file_reader::State;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::iter::Iterator;
use std::process;
use utf8_line_reader::Utf8LineReader;

mod utf8_line_reader;
mod log_file_reader;

fn main() {
    if let Err(err) = run() {
        eprintln!("Failed because: {}", err);
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let filenames = env::args().skip(1).collect::<Vec<String>>();

    let mut log_file_readers = Vec::new();

    // Open all of the files, setting up a Utf8LineReader to read their lines irrespective of their
    // encodings, then wrap them in a filter for valid timestamps.
    for filename in filenames {
        let f = File::open(&filename)?;
        let mut reader = BufReader::new(f);
        let mut utf8_line_reader = Utf8LineReader::new(reader)?;
        let mut log_line_reader = LogFileReader::new(filename, utf8_line_reader);
        log_file_readers.push(log_line_reader);
    }

    loop {
        // Advance each hungry LogFileReader to the next timestamped line.
        for reader in log_file_readers.iter_mut() {
            reader.advance();
        }

        // Determine which (if any) readable LogFileReader has the earliest timestamp.
        let reader = log_file_readers.iter_mut()
            .filter(|f| f.state == State::Readable)
            .min_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // If there's an LogFileReader available then take a line from it and display it.
        if let Some(reader) = reader {
            if let Some((filename, timestamp, line)) = reader.take_line() {
                println!("{}: {}{}", filename, timestamp, line);
            }
        } else {
            // That's it - there are no more lines available.
            break;
        }
    }

    Ok(())
}
