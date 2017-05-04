// Copyright 2017 LambdaStack All rights reserved.
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

use std;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::net;
use std::path::Path;
use std::result;
use std::str::FromStr;

use toml;
use url::Url;

use error::{Error, Result};

/// Defines the default ConfigFile operation of ```from_file``` and ```from_toml```
///
/// ```from_toml``` should be implemented in the calling project for a Config struct that
/// you define. For example:
///
/// #[derive(Clone, Debug, PartialEq, Eq)]
/// pub struct Config {
///     pub endpoint: Option<String>,
///     pub proxy: Option<String>,
/// }
///
/// impl ConfigFile for Config {
///     type Error = Error;
///
///     fn from_toml(toml: toml::Value) -> Result<Self> {
///         let mut cfg = Config::default();
///
///         let endpoint = match toml.lookup("options.endpoint") {
///             Some(ep) => Some(ep.as_str().unwrap().to_string()),
///             None => None,
///         };
///
///         let proxy = match toml.lookup("options.proxy") {
///             Some(p) => Some(p.as_str().unwrap().to_string()),
///             None => None,
///         };
///
///         cfg.endpoint = endpoint;
///         cfg.proxy = proxy;
///
///         Ok(cfg)
///     }
/// }
///
/// impl Default for Config {
///     fn default() -> Self {
///         Config {
///             endpoint: None,
///             proxy: None,
///         }
///     }
/// }
///
pub trait ConfigFile: Sized {
    type Error: std::error::Error + From<Error>;

    fn from_file<T: AsRef<Path>>(filepath: T) -> result::Result<Self, Self::Error> {
        let mut file = match File::open(filepath.as_ref()) {
            Ok(f) => f,
            Err(e) => return Err(Self::Error::from(Error::FileIO(e))),
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Ok(_) => (),
            Err(e) => return Err(Self::Error::from(Error::FileIO(e))),
        }
        match raw.parse() {
            Ok(toml) => Self::from_toml(toml),
            Err(e) => {
                let msg = format_errors(&e);
                Err(Self::Error::from(Error::FileSyntax(msg)))
            }
        }
    }

    fn from_toml(toml: toml::Value) -> result::Result<Self, Self::Error>;
}

/// ParseInto allows for many different types to be converted for toml::Value types.
///
pub trait ParseInto<T> {
    fn parse_into(&self, field: &'static str, out: &mut T) -> Result<bool>;
}

impl ParseInto<Vec<net::SocketAddrV4>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<net::SocketAddrV4>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(slice) = val.as_slice() {
                let mut buf = vec![];
                for entry in slice.iter() {
                    if let Some(v) = entry.as_str() {
                        match net::SocketAddrV4::from_str(v) {
                            Ok(addr) => buf.push(addr),
                            Err(_) => return Err(Error::InvalidSocketAddrV4(field)),
                        }
                    } else {
                        return Err(Error::InvalidSocketAddrV4(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                // error, expected array
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<net::SocketAddrV4> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut net::SocketAddrV4) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                match net::SocketAddrV4::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::InvalidSocketAddrV4(field)),
                }
            } else {
                Err(Error::InvalidSocketAddrV4(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<net::Ipv4Addr> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut net::Ipv4Addr) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                match net::Ipv4Addr::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::InvalidIpv4Addr(field)),
                }
            } else {
                Err(Error::InvalidIpv4Addr(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Url> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Url) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                *out = Url::parse(v).unwrap();
                Ok(true)
            } else {
                Err(Error::InvalidUrl(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Option<Url>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Option<Url>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                *out = Some(Url::parse(v).unwrap());
                Ok(true)
            } else {
                Err(Error::InvalidUrl(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<String> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut String) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                *out = v.to_string();
                Ok(true)
            } else {
                Err(Error::InvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Option<String>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Option<String>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                *out = Some(v.to_string());
                Ok(true)
            } else {
                Err(Error::InvalidString(field))
            }
        } else {
            *out = None;
            Ok(true)
        }
    }
}

impl ParseInto<usize> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut usize) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as usize;
                Ok(true)
            } else {
                Err(Error::InvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u16> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u16) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u16;
                Ok(true)
            } else {
                Err(Error::InvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u32> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u32) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u32;
                Ok(true)
            } else {
                Err(Error::InvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u64> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u64) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u64;
                Ok(true)
            } else {
                Err(Error::InvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u16>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u16>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u16);
                    } else {
                        return Err(Error::InvalidArray(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::InvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u32>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u32>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u32);
                    } else {
                        return Err(Error::InvalidArray(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::InvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u64>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u64>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u64);
                    } else {
                        return Err(Error::InvalidArray(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::InvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<BTreeMap<String, String>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut BTreeMap<String, String>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            let buf: BTreeMap<String, String> = val.as_table()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
                .collect();
            *out = buf;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<BTreeMap<String, String>>> for toml::Value {
    fn parse_into(&self,
                  field: &'static str,
                  out: &mut Vec<BTreeMap<String, String>>)
                  -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for m in v.iter() {
                    let map: BTreeMap<String, String> = m.as_table()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
                        .collect();
                    buf.push(map);
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::InvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

fn format_errors(errors: &Vec<toml::ParserError>) -> String {
    let mut msg = String::new();
    for err in errors {
        msg.push_str(&format!("\terror: {}\n", err.desc));
    }
    msg
}
