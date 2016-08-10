/*
 Copyright 2016 LambdaStack All rights reserved.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

/////////////////////////////////////
// NOTE: This is only a test file used to test a few scenarios to the library routines.
// The code once it works is commented out and left in the file as a reference.
// DO NOT MISTAKE THIS AS PART OF THE LIBRARY!!
/////////////////////////////////////

#[macro_use]
extern crate lsio;
extern crate ansi_term;
//extern crate url;
extern crate semver;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::iter;
//use std::process::Command;
//use std::process::Output;
//use std::io::prelude::*;
//use std::io::Result;
use std::env::consts;
use ansi_term::Colour::{Red, Green};
//use url::Url;
use semver::Version;

use lsio::command::run_cli;

/// #lsio - LambdaStack.io is the primary utility of the LambdaStack CI/CD solution.
/// Building on MAC OSX - SSH2 has a compile error that can be fixed with the following:
///   brew install libssh2
///   brew install openssl
///   brew link --force openssl
/// https://github.com/alexcrichton/ssh2-rs/issues/23
///


/* Example of conditional compiles */
#[cfg(target_os = "macos")]
fn os_test() {
    println!("YES - MAC OSX!");
}

#[cfg(target_os = "linux")]
fn os_test() {
    println!("YES - Linux!");
}

// try! can't be in main so you have to create another function and then call it from main

fn main() {
    env_logger::init().unwrap();

    let n:i32;
    let show_output:bool = true;
    let mut desc = String::new();
    let mut output = String::new();

    /* Example of splitting words...
    let mut arguments = "localhost -m shell -a \"ls\"".split(" "); //.collect::<Vec<&str>>();
    let splits: Vec<&str> = arguments.collect();

    for s in splits {
        println!("{:?}", s);
    }
    */

    /* Example of run_args...
    let mut args: Vec<String> = Vec::new();
    args.push("aux".to_string());
    args.push("|".to_string());
    args.push("grep".to_string());
    args.push("Atom".to_string());

    match run_args("ps", &args, true) {
        Ok(out_put) => {
            println!("{}", String::from_utf8_lossy(&out_put.stdout).to_string());
            println!("{}", out_put.status.code().unwrap());
        },
        Err(err) => println!("{:?}", err),
    }
    */

    repeat_red!("=", 80);

    // Example - tab like
    println!("{:<4}{}", " ", "test");

    repeat!("-", 80);

    let version = Version::parse("0.1.0");
    println!("App: {} source: {} version: {} line: {}", module_path!(), file!(), version.unwrap(), line!());

    repeat!("-", 80);

    // Example logging...
    trace!(".............");
    info!("Start logging...");
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    repeat!("-", 80);

    println!("OS: {}", consts::OS);
    println!("ARCH: {}", consts::ARCH);
    println!("FAMILY: {}", consts::FAMILY);

    if cfg!(target_os = "macos") {
        println!("Yes, it's a MAC - 1!");
    }

    os_test();

    repeat!("-", 80);

    // Example of reading file to a static (which is inferred) string relative to the app. ONLY works to bring in at compile time.
    let secret_key = include_str!("secret-key.ascii");
    println!("{}", secret_key);

    repeat!("-", 80);

    // Example: match run_cli("ansible localhost -vvv -m shell -a \"ls -la\"".to_string()) {
    // Example: match run_cli("".to_string()) {
    // Example: match run_cli("/Users/chrisjones/projects/lambdastack/lsio/test.sh".to_string()) {
    match run_cli("sudo ifconfig | grep en0".to_string()) {
        Ok(out_put) => {
            n = out_put.status.code().unwrap();
            if show_output {
                if n == 0 {
                    output = String::from_utf8_lossy(&out_put.stdout).to_string();
                } else {
                    output = String::from_utf8_lossy(&out_put.stderr).to_string();
                }
            }
        },
        Err(err) => {
            n = err.raw_os_error().unwrap();
            if show_output {
                desc = err.to_string()
            }
        }
    }

    println!("{}", if output.len() > 0 {if n == 0 {Green.paint(output)} else {Red.paint(output)}} else {Red.paint(desc)});

    //repeat!("-", 80);

    //let issue_list_url = Url::parse(
    //    "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    //).unwrap();

    //println!("{:?}", issue_list_url);

    repeat!("-", 80);

    repeat_red!("=", 80);

    /* Example use of errno crate for cross platform codes
    // extern crate errno;
    // use errno::{Errno, errno, set_errno};
    let errno = errno();
    println!("{:?}", errno);
    */

    /* Example of reading input...
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    */

    /* Example Enter in password (no echo)...
    extern crate rpassword;

    use rpassword::read_password;
    :
    :
    print!("Type a password: ");
    // This will flush the buffer with 'print!' above. Without it it will not show until something
    // else flushes it like println! for example.
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    println!("The password is: {}", password);
    */

    /* Example of SSH...
    extern crate ssh2;

    use std::net::TcpStream;
    use ssh2::Session;
    :
    :
    println!("{}", "Enter SSH command: ");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let tcp = TcpStream::connect("10.0.100.20:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();
    sess.userauth_password("vagrant", "vagrant").unwrap();
    // To use this method you need to do the following:
    // ssh-agent bash
    // ssh-add ~/.ssh/id_rsa  <- (note: or whatever your ssh key is called)
    sess.userauth_agent("vagrant").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec(&line).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    println!("{}", channel.exit_status().unwrap());
    */
}
