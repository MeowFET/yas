use std::fs;
use std::io::stdin;
use std::process;
use std::time::Duration;
use std::{thread, time};

use crate::dto::GithubTag;
use log::error;
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, USER_AGENT};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
pub use mac::*;

pub fn sleep(ms: u32) {
    let time = time::Duration::from_millis(ms as u64);
    thread::sleep(time);
}

pub fn read_file_to_string(path: String) -> String {
    let content = fs::read_to_string(path).unwrap();
    content
}

pub fn error_and_quit(msg: &str) -> ! {
    error!("{}, 按Enter退出", msg);
    let mut s: String = String::new();
    stdin().read_line(&mut s);
    process::exit(0);
}

#[cfg(not(windows))]
pub fn is_rmb_down() -> bool {
    false
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_update() -> Option<String> {
    let client = Client::new();

    let resp = client
        .get("https://api.github.com/repos/wormtql/yas/tags")
        .timeout(Duration::from_secs(5))
        .header(USER_AGENT, HeaderValue::from_static("reqwest"))
        .send()
        .ok()?
        .json::<Vec<GithubTag>>()
        .ok()?;

    let latest = if resp.len() == 0 {
        return None;
    } else {
        resp[0].name.clone()
    };
    let latest = &latest[1..];

    let latest_sem: semver::Version = semver::Version::parse(&latest).unwrap();
    let current_sem: semver::Version = semver::Version::parse(VERSION).unwrap();

    if latest_sem > current_sem {
        Some(String::from(latest))
    } else {
        None
    }
}
