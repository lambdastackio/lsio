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
//

#[allow(unused_imports)]
use term;

// NOTE: Had to add {} block around let mut t below because it was a statement.

/// print! with a specific color.
///
#[macro_export]
macro_rules! print_color {
    ($color:expr, $($arg:tt)*) => {
        {
        let mut t = term::stderr().unwrap();
        t.fg($color).unwrap();
        print!($($arg)*);
        t.reset().unwrap();
        }
    };
}

/// print_color! with quiet option.
///
#[macro_export]
macro_rules! println_color_quiet {
    ($quiet:expr, $color:expr, $($arg:tt)*) => {
        {
            if $quiet != false {
                let mut t = term::stderr().unwrap();
                t.fg($color).unwrap();
                print!($($arg)*);
                t.reset().unwrap();
            }
        }
    };
}

/// println! with a specific color.
///
#[macro_export]
macro_rules! println_color {
    ($color:expr, $fmt:expr) => (print_color!($color, concat!($fmt, "\n")));
    ($color:expr, $fmt:expr, $($arg:tt)*) => (print_color!($color, concat!($fmt, "\n"), $($arg)*));
}

/// println_color! with a quiet option.
///
#[macro_export]
macro_rules! println_color_quiet {
    ($quiet: expr, $color:expr, $fmt:expr) => {
        if $quiet != false {
            print_color!($color, concat!($fmt, "\n"));
        }
    };
    ($quiet: expr, $color:expr, $fmt:expr, $($arg:tt)*) => {
        if $quiet != false {
            print_color!($color, concat!($fmt, "\n"), $($arg)*);
        }
    };
}

/// repeat_color - Prints out a repeat of characters in a specific color.
///
/// Currently prints characters using the color specified.
///
/// # Example
/// ```
/// repeat_color!(term::color::RED, "=", "", 80);
/// ```
#[macro_export]
macro_rules! repeat_color {
    ($color:expr, $e:expr, $text:expr, $size:expr) => {
        let overlay: String = String::from($text);
        let len = overlay.len();
        let repeat_size: u16 = ($size - len)/2;
        let repeated: String = iter::repeat($e).take($size).collect();
        println_color!($color, "{}{}{}", repeated, overlay, repeated);
    }
}

/// repeat_color! with quiet option.
#[macro_export]
macro_rules! repeat_color_quiet {
    ($quiet:expr, $color:expr, $e:expr, $text, $size:expr) => {
        if $quiet != false {
            repeat_color!($color, $e, $text, $size);
        }
    }
}

/// repeat - Prints out a repeat of characters
///
/// Currently prints characters using the default color (white).
///
/// # Example
/// ```
/// repeat!("=", "", 80);
/// ```
#[macro_export]
macro_rules! repeat {
    ($e:expr, $text:expr, $size:expr) => {
        repeat_color!(term::color::WHITE, $e, $text, $size);
    }
}
