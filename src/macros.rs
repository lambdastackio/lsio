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

/// Sizes and zero fills a vector of bytes for a buffer.
///
#[macro_export]
macro_rules! zero_fill_buffer {
    ($buffer:expr, $len:expr) => {
        $buffer = Vec::with_capacity($len as usize);
        for i in 0..$len {
            $buffer.push(0);
        }
    };
}

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
            if !$quiet {
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
        if !$quiet {
            print_color!($color, concat!($fmt, "\n"));
        }
    };
    ($quiet: expr, $color:expr, $fmt:expr, $($arg:tt)*) => {
        if !$quiet {
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
        let repeat_size: usize = ($size - len)/2;
        let repeated: String = iter::repeat($e).take(repeat_size).collect();
        // Fill is required since dealing with blocks instead of pixels so some may have an extra $e
        let fill_size: usize = $size - ((repeat_size * 2) + len);
        let fill: String = iter::repeat($e).take(fill_size).collect();
        println_color!($color, "{}{}{}{}", repeated, overlay, repeated, fill);
    }
}

/// repeat_color_with_ends - Prints out a repeat of characters in a specific color and centers
/// any text passed in along with printing left end and right end characters.
///
/// Currently prints characters using the color specified.
///
/// # Example
/// ```
/// repeat_color!(term::color::RED, "=", "", 80);
/// ```
#[macro_export]
macro_rules! repeat_color_with_ends {
    ($color:expr, $e:expr, $text:expr, $lend:expr, $rend:expr, $size:expr) => {
        let overlay: String = String::from($text);
        let left_end: String = String::from($lend);
        let right_end: String = String::from($rend);
        let len = overlay.len();
        let end_len = left_end.len() + right_end.len();
        let repeat_size: usize = (($size - end_len) - len)/2;
        let repeated: String = iter::repeat($e).take(repeat_size).collect();
        // Fill is required since dealing with blocks instead of pixels so some may have an extra $e
        let fill_size: usize = $size - end_len - ((repeat_size * 2) + len);
        let fill: String = iter::repeat($e).take(fill_size).collect();
        println_color!($color, "{}{}{}{}{}{}", left_end, repeated, overlay, repeated, fill, right_end);
    }
}


/// repeat_color! with quiet option.
#[macro_export]
macro_rules! repeat_color_quiet {
    ($quiet:expr, $color:expr, $e:expr, $text, $size:expr) => {
        if !$quiet {
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
