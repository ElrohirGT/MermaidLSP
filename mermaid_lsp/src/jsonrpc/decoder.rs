use std::io::prelude::*;
use std::io::BufReader;

use log::debug;
use log::error;

use super::ClientMessage;

pub struct LSPMessages<T: std::io::Read> {
    reader: BufReader<T>,
}

impl<T: std::io::Read> LSPMessages<T> {
    pub fn new(reader: BufReader<T>) -> Self {
        LSPMessages { reader }
    }
}

#[derive(Debug)]
pub enum ParseJsonRPCMessageErrors {
    FailedToReadHeader(std::io::Error),
    IncorrectHeaderFormat,
    FailedToReadBody(std::io::Error),
    ContentLengthNotANumber(std::num::ParseIntError),
    FailedToParseBody(serde_json::Error),
}

impl<T: std::io::Read> Iterator for LSPMessages<T> {
    type Item = Result<ClientMessage, ParseJsonRPCMessageErrors>;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("[LSPMessages iterator] Next function called!");
        let mut header_bytes = vec![];

        if let Err(e) = self
            .reader
            .read_until(b'\n', &mut header_bytes)
            .map_err(ParseJsonRPCMessageErrors::FailedToReadHeader)
        {
            error!(
                "[LSPMessages iterator] Failed to read LSP message header! {:?}",
                e
            );
            return Some(Err(e));
        }
        self.reader.consume(2); // The remaining \r\n that separate the header and the body
        debug!(
            "[LSPMessages iterator] Header bytes received! {:?}",
            String::from_utf8_lossy(&header_bytes)
        );

        let last_header_part = match header_bytes
            .split(|b| *b == b':')
            .last()
            .ok_or(ParseJsonRPCMessageErrors::IncorrectHeaderFormat)
        {
            Ok(v) => v,
            Err(e) => {
                error!("[LSPMessages iterator] Invalid header format!");
                return Some(Err(e));
            }
        };

        // Ignoring [ ]<NUMBER>[\r\n]
        let body_length_bytes = &last_header_part[1..(last_header_part.len() - 2)];
        debug!(
            "[LSPMessages iterator] Body length as a string! - Original: {:?} - Cut: {:?}",
            String::from_utf8_lossy(last_header_part),
            String::from_utf8_lossy(body_length_bytes)
        );
        let body_length_string = match String::from_utf8(body_length_bytes.to_vec())
            .map_err(|_| ParseJsonRPCMessageErrors::IncorrectHeaderFormat)
        {
            Ok(v) => v,
            Err(e) => {
                error!("[LSPMessages iterator] Invalid UTF8-bytes were supplied to header!");
                return Some(Err(e));
            }
        };

        let body_length: usize = match body_length_string
            .parse()
            .map_err(ParseJsonRPCMessageErrors::ContentLengthNotANumber)
        {
            Ok(v) => v,
            Err(err) => {
                error!(
                    "[LSPMessages iterator] The content length is not a valid number! {:?}",
                    err
                );

                return Some(Err(err));
            }
        };
        debug!("[LSPMessages iterator] Body length: {}", body_length);

        let mut body_bytes = vec![0u8; body_length];
        if let Err(e) = self
            .reader
            .read_exact(&mut body_bytes)
            .map_err(ParseJsonRPCMessageErrors::FailedToReadBody)
        {
            error!(
                "[LSPMessages iterator] Failed to read message body! {:?}",
                e
            );
            return Some(Err(e));
        }

        let body = match serde_json::from_slice(&body_bytes)
            .map_err(ParseJsonRPCMessageErrors::FailedToParseBody)
        {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "[LSPMessages iterator] Failed to parse message body from bytes! {:?}",
                    e
                );

                return Some(Err(e));
            }
        };
        debug!("[LSPMessages iterator] Bytes read to a string! {:?}", body);

        Some(Ok(body))
    }
}
