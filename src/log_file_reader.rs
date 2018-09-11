use chrono::{DateTime, Utc};
use chrono::offset::TimeZone;
use std::io::BufRead;
use std::iter::Peekable;
use utf8_line_reader::Utf8LineReader;

#[derive(PartialEq)]
pub enum State {
    // We have a validly timestamped line that is ready to be read.
    Readable,
    // We don't have anything ready to be read and need to be advanced.
    Hungry,
    // There are no lines remaining to be read.
    Done,
}

/// A reader for log files with timestamps.
pub struct LogFileReader<T: BufRead> {
    filename: String,
    lines: Peekable<Utf8LineReader<T>>,
    format: Option<String>,
    length: usize,
    pub timestamp: DateTime<Utc>,
    pub state: State,
}

impl<T: BufRead> LogFileReader<T> {
    /// Creates an LogFileReader for the given filename and reader.
    pub fn new(filename: String, reader: Utf8LineReader<T>) -> LogFileReader<T> {
        LogFileReader {
            filename,                   // The name of the log file being read.
            lines: reader.peekable(),   // The lines of the file.
            format: None,               // The datetime format.
            length: 0,                  // The length of the timestamp in the given format.
            timestamp: Utc::now(),      // The last read timestamp.
            state: State::Hungry,       // What state is this reader in?
        }
    }

    /// Advances the LogFileReader to the next timestamped line, if there is one.
    pub fn advance(&mut self) {
        if self.state != State::Hungry {
            return;
        }

        let lines = &mut self.lines;

        // Advance the input until we find a line with a timestamp, or reach the end.
        loop {
            if let Some(next_line) = lines.peek() {
                // There's a next line, so see if it starts with a timestamp.
                if let Ok(line) = next_line {
                    let line = line.as_str();
                    if line.len() >= self.length {
                        // If we don't yet know the timestamp format then attempt to guess it.
                        if self.format.is_none() {
                            if let Some((format, length)) = guess_datetime_format(line) {
                                self.format = Some(format);
                                self.length = length;
                            }
                        }

                        // If we do know the timestamp format then inspect the line to see if it has a timestamp,
                        // and set our state to Readable if it does.
                        if let Some(format) = &self.format {
                            let timestamp: &str = &line[..self.length];
                            if let Ok(timestamp) = Utc.datetime_from_str(&timestamp, format) {
                                self.timestamp = timestamp;
                                self.state = State::Readable;
                                break;
                            }
                        }
                    }
                }
            } else {
                // There isn't a next line.
                self.state = State::Done;
                break;
            }
            // We've established that the next line does not start with a timestamp, so consume it
            // and move on.
            lines.next();
        }
    }

    /// Returns the next line if the LogFileReader is in a readable state and it has a valid timestamp.
    pub fn take_line(&mut self) -> Option<(&str, DateTime<Utc>, String)> {
        if self.state != State::Readable {
            return None;
        }

        self.state = State::Hungry;

        // Return the next line if it has a valid timestamp.
        let lines = &mut self.lines;
        if let Some(line) = lines.next() {
            if let Ok(line) = line {
                let line = line.as_str();
                let length = self.length;
                let timestamp: &str = &line[..length];
                if let Some(format) = &self.format {
                    if let Ok(timestamp) = Utc.datetime_from_str(timestamp, &format) {
                        let rest_of_line: &str = &line[length..];
                        return Some((self.filename.as_str(), timestamp, rest_of_line.to_string()));
                    }
                }
            }
        }

        // The line does not have a valid timestamp.
        None
    }
}

/// Attempts to guess the datetime format and its size from the input string.
fn guess_datetime_format(line: &str) -> Option<(String, usize)> {
    let formats = vec![
        ("%FT%T%.f", 29),     // eg, "2018-05-09T12:00:09.123123123"
        ("%FT%T%.f", 26),     // eg, "2018-05-09T12:00:09.123123"
        ("%FT%T%.f", 23),     // eg, "2018-05-09T12:00:09.123"
        ("%FT%T", 19),        // eg, "2018-05-09T12:00:09.123"
        ("%F %T%.f", 29),     // eg, "2018-05-09 12:00:09.123123123"
        ("%F %T%.f", 26),     // eg, "2018-05-09 12:00:09.123123"
        ("%F %T%.f", 23),     // eg, "2018-05-09 12:00:09.123"
        ("%F %T", 19),        // eg, "2018-05-09 12:00:09"
        ("%Y%m%d%H%M%S", 14), // eg, "20180509120009"
        ("%c", 24),           // eg, "Sat May 12 11:39:55 2018"
    ];

    for (time_format, len) in formats {
        let potential_ts = line.chars().take(len).collect::<String>();
        let ts = Utc.datetime_from_str(&potential_ts, time_format);
        if ts.is_ok() {
            return Some((time_format.to_string(), len));
        }
    }
    None
}
