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

use std::io::{self, Write};

use term;

/// confirmation prompt in a specific color.
///
/// Allows you to specify the prompt answer to be on the same line as the prompt or on the line
/// below it. Returns true or false.
///
pub fn confirm(prompt: &str, same_line: bool, color: term::color::Color) -> bool {
    if same_line {
        print_color!(color, "{} ", prompt);
    } else {
        println_color!(color, "{} ", prompt);
    }

    // Force a flush of the buffer.
    io::stdout().flush().unwrap();
    let mut input = String::new();

    // Get terminal and set color
    let mut t = term::stderr().unwrap();
    t.fg(color).unwrap();

    let mut ret_code: bool = false;

    io::stdin().read_line(&mut input).ok();
    match &*input.trim().to_lowercase() {
        "y" | "yes" => {
            ret_code = true;
        },
        _ => {},
    }

    // Set the color back to normal.
    t.reset().unwrap();

    ret_code
}
