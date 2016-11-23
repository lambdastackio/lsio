// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]

use std::error;
use std::io;
use std::fmt;
use std::num;
use std::result;
use std::str;
use std::string;

pub type Result<T> = result::Result<T, Error>;

/// Core error types
#[derive(Debug)]
pub enum Error {
    /// Error reading raw contents of file.
    FileIO(io::Error),
    /// Parsing error while reading a file. For example, JSON, TOML, YAML, etc
    FileSyntax(String),
    /// Expected a valid array of values for field value.
    InvalidArray(&'static str),
    /// Expected a valid Ipv4 network address for field value.
    InvalidIpv4Addr(&'static str),
    /// Expected a valid SocketAddrV4 address pair for field value.
    InvalidSocketAddrV4(&'static str),
    /// Expected a string for field value.
    InvalidString(&'static str),
    /// Expected a URL for field value.
    InvalidUrl(&'static str),
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    /// IP Address error
    IPFailed,
    /// When an error occurs parsing an integer.
    ParseIntError(num::ParseIntError),
    /// When an error occurs converting a `String` from a UTF-8 byte vector.
    StringFromUtf8Error(string::FromUtf8Error),
    /// Occurs when a `uname` libc call returns an error.
    UnameFailed(String),
    /// When an error occurs attempting to interpret a sequence of u8 as a string.
    Utf8Error(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::FileIO(ref e) => format!("Error reading file: {}", e),
            Error::FileSyntax(ref e) => {
                format!("Syntax errors while parsing file:\n\n{}",
                        e)
            }
            Error::InvalidArray(ref f) => {
                format!("Invalid array of values, field={}", f)
            }
            Error::InvalidIpv4Addr(ref f) => {
                format!("Invalid Ipv4 address, field={}. (example: \"127.0.0.0\")",
                        f)
            }
            Error::InvalidSocketAddrV4(ref f) => {
                format!("Invalid Ipv4 network address pair, field={}. (example: \
                         \"127.0.0.0:8080\")",
                        f)
            }
            Error::InvalidString(ref f) => {
                format!("Invalid string value, field={}.", f)
            }
            Error::InvalidUrl(ref f) => {
                format!("Invalid URL value, field={}.", f)
            }
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::IO(ref err) => format!("{}", err),
            Error::IPFailed => format!("Failed to discover this hosts IP address"),
            Error::ParseIntError(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::UnameFailed(ref e) => format!("{}", e),
            Error::Utf8Error(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FileIO(_) => "Unable to read the raw contents of file",
            Error::FileSyntax(_) => "Error parsing contents of file",
            Error::InvalidArray(_) => {
                "Invalid array of values encountered while parsing file"
            }
            Error::InvalidIpv4Addr(_) => {
                "Invalid Ipv4 network address encountered while parsing file"
            }
            Error::InvalidSocketAddrV4(_) => {
                "Invalid Ipv4 network address pair encountered while parsing file"
            }
            Error::InvalidString(_) => {
                "Invalid string value encountered while parsing a file"
            }
            Error::InvalidUrl(_) => {
                "Invalid URL value encountered while parsing file"
            }
            Error::FileNotFound(_) => "File not found",
            Error::IO(ref err) => err.description(),
            Error::IPFailed => "Failed to discover this hosts IP address",
            Error::ParseIntError(_) => "Failed to parse an integer from a string!",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            Error::UnameFailed(_) => "Failed to get uname on host",
            Error::Utf8Error(_) => "Failed to interpret a sequence of bytes as a string",
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}
