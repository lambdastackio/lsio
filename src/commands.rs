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

use std::process::Command;
use std::process::Output;
use std::io::Result;

/// run_args - pass in the cmd as low_level &str and args as a String slice
///
/// The function allows you to add all of the options to a given command to
/// # Example
///
/// ```
/// let mut args: Vec<String> = Vec::new();
/// args.push("aux".to_string());
///
/// run_args("ps", &args);
/// ```
pub fn run_args(cmd: &str, args: &[String], shell: bool) -> Result<(Output)> {
    let output;

    if args.len() > 0 {
        if !shell {
            output = try!(Command::new(cmd).args(args.iter()).output());
        } else {
            let mut arg_string = String::new();
            arg_string = arg_string + cmd + " ";
            for s in args {
                arg_string = arg_string + s + " ";
            }
            output = try!(run_cli(arg_string));
        }
    } else {
        // output = try!(Command::new(cmd).output());
        output = try!(run_cli(cmd.to_string()));
    }

    Ok(output)
}

/// run_cli - pass in a String of a normal command line
///
/// The function will split the options into words to supply to the low_level std::process::Command
/// which returns Result<(Output)>
/// # Example
///
/// ```
/// run_cli("ps aux");
/// ```

// NOTE: Add Into so a "" can also be passed in...
pub fn run_cli(cmd_line: String) -> Result<(Output)> {
    let output = try!(Command::new("sh").arg("-c").arg(&cmd_line).output());
    Ok(output)

    // Example of word splitting...
    // let output;
    // let mut words = cmd_line.split_whitespace();
    // let cmd = words.next();
    //
    // match cmd {
    // Some(cmd) => {
    // let args = words.collect::<Vec<&str>>();
    // if args.len() > 0 {
    // output = try!(Command::new(cmd).args(&args).output());
    // } else {
    // output = try!(Command::new(cmd).output());
    // }
    // }
    // None => {
    // Let error bubble up
    // output = try!(Command::new(&cmd_line).output());
    // }
    // }
    //
    // Ok(output)
    //
}
