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

use std::ffi::CStr;
use std::mem;
use std::net::{IpAddr, UdpSocket};
use std::str;

use libc;
use errno::errno;
use error::{Error, Result};

static GOOGLE_DNS: &'static str = "8.8.8.8:53";

/// Pass in the IP of your DNS if you're unable to reach the Internet or leave it empty.
///
pub fn ip(dns: &str) -> Result<IpAddr> {
    let socket = try!(UdpSocket::bind("0.0.0.0:0"));
    let ip = format!("{}:53", dns);
    let _ = try!(socket.connect(if dns.is_empty() {GOOGLE_DNS} else {&ip}));
    let addr = try!(socket.local_addr());
    Ok(addr.ip())
}

extern {
    pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

pub fn hostname() -> Result<String> {
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_slice().as_mut_ptr();
    let err = unsafe {
        gethostname(ptr as *mut libc::c_char, len as libc::size_t)
    };
    match err {
        0 => {
            let slice = unsafe {
                CStr::from_ptr(ptr as *const libc::c_char)
            };
            let s = try!(slice.to_str());
            Ok(s.to_string())
        }
        _ => {
            Err(Error::IPFailed)
        }
    }
}

/*
pub fn hostname() -> Result<String> {
    let output = try!(Command::new("sh")
        .arg("-c")
        .arg("hostname")
        .output());
    match output.status.success() {
        true => {
            debug!("Hostname address is {}",
                   String::from_utf8_lossy(&output.stdout));
            let hostname = try!(String::from_utf8(output.stdout).or(Err(Error::Sys)));
            Ok(hostname)
        }
        false => {
            debug!("Hostname address command returned: OUT: {} ERR: {}",
                   String::from_utf8_lossy(&output.stdout),
                   String::from_utf8_lossy(&output.stderr));
            Err(Error::Sys)
        }
    }
}
*/

#[derive(Debug)]
pub struct Uname {
    pub sys_name: String,
    pub node_name: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

#[cfg(not(windows))]
pub fn uname() -> Result<Uname> {
    unsafe { uname_libc() }
}

#[cfg(not(windows))]
unsafe fn uname_libc() -> Result<Uname> {
    let mut utsname: libc::utsname = mem::uninitialized();
    let rv = libc::uname(&mut utsname);
    if rv < 0 {
        let errno = errno();
        let code = errno.0 as i32;
        return Err(Error::UnameFailed(format!("Error {} when calling uname: {}", code, errno)));
    }
    Ok(Uname {
        sys_name: CStr::from_ptr(utsname.sysname.as_ptr()).to_string_lossy().into_owned(),
        node_name: CStr::from_ptr(utsname.nodename.as_ptr()).to_string_lossy().into_owned(),
        release: CStr::from_ptr(utsname.release.as_ptr()).to_string_lossy().into_owned(),
        version: CStr::from_ptr(utsname.version.as_ptr()).to_string_lossy().into_owned(),
        machine: CStr::from_ptr(utsname.machine.as_ptr()).to_string_lossy().into_owned(),
    })
}

#[cfg(windows)]
pub fn uname() -> Result<Uname> {
    Ok(Uname {
        sys_name: String::from("Windows"),
        node_name: String::from(""),
        release: String::from(""),
        version: String::from("Microsoft Windows 10 Enterprise Insider Preview"),
        machine: String::from("x86_64"),
    })
}
