#![allow(non_snake_case)]
use crate::utils::Result;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use lazy_regex::regex;
use encoding_rs::Encoding;

#[derive(Debug)]
pub struct Parser {
    url: String,
    username: Option<String>,
    password: Option<String>,
    raw: String,
}

impl Parser {
    pub fn new(text: &str) -> Self {
        let url = Self::get_url(text).unwrap_or_else(|| "NotFound".to_string());
        let (username, password) = Self::get_userpass(text, &url);

        Self {
            url,
            username,
            password,
            raw: text.to_string(),
        }
    }

    fn get_url(text: &str) -> Option<String> {
        let re = if Self::is_port(text) {
            if Self::is_http(text) {
                regex!(r"(?P<url>https?://[^\:]+:\d{2,5}[^\:\s\|]+)")
            } else {
                regex!(r"(?P<url>(?:[a-zA-Z]+\.)+(?:com|net|org|edu|gov|int|mil|biz|info|name|pro|aero|coop|museum|[a-z]{2})\:\d{2,5}[^\|\s\r\n\:]+)")
            }
        } else if Self::is_http(text) {
            regex!(r"(?P<url>https?://[^\:\s\|]+)")
        } else {
            regex!(r"(?P<url>(?:[a-zA-Z]+\.)+(?:com|net|org|edu|gov|int|mil|biz|info|name|pro|aero|coop|museum|[a-z]{2})[^\|\s\r\n\:]+)")
        };

        re.captures(text).and_then(|caps| caps.name("url").map(|m| m.as_str().to_string()))
    }

    fn get_userpass(text: &str, url: &str) -> (Option<String>, Option<String>) {
        if Self::is_mail_address(url) {
            if let Some(caps) = regex!(r"(?P<user>[^\s\|\:]+)(?:\s|\||:){1}(?P<pass>\S+)").captures(text) {
                return (
                    caps.name("user").map(|m| m.as_str().to_string()),
                    caps.name("pass").map(|m| m.as_str().to_string()),
                );
            }
        } else {
            let userpass = if text.starts_with(url) {
                &text[url.len() + 1..]
            } else {
                &text[..text.len() - url.len() - 2]
            };
            if let Some(caps) = regex!(r"(?P<user>[^\s\|\:]+)(?:\s|\||:){1}(?P<pass>\S+)").captures(userpass) {
                return (
                    caps.name("user").map(|m| m.as_str().to_string()),
                    caps.name("pass").map(|m| m.as_str().to_string()),
                );
            }
        }
        (None, None)
    }

    fn is_port(text: &str) -> bool {
        regex!(r"\:\d{2,5}(?:/|\s|\t|$|\|)").is_match(text)
    }

    fn is_http(text: &str) -> bool {
        regex!(r"https?://").is_match(text)
    }

    fn is_mail_address(s: &str) -> bool {
        regex!(r"\S+@\S+").is_match(s)
    }
}

pub fn search_keyword(dataleak_line: &str) -> (bool, Option<String>) {
    let path = Path::new("./monitored_wordlist.txt");
    let momo_list = vec!["momo.vn", "mservice.com.vn", "cvs.vn", "mservice.io", "momocdn.net", "momoapp.vn"];

    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            log::warn!("Please create monitored_wordlist.txt before running!");
            std::process::exit(1);
        }
    };

    let monitored_keyword_list: Vec<&str> = content.lines().collect();
    println!("{:#?}", monitored_keyword_list);

    for monitored_key in monitored_keyword_list {
        if dataleak_line.contains(monitored_key) {
            if momo_list.contains(&monitored_key) {
                println!("[*] Keyword detected: {}", monitored_key);
                return (true, Some("Momo".to_string()));
            }
            return (true, Some("Unknown".to_string()));
        }
    }

    (false, None)
}

pub fn verify_send(file_path: &Path) -> Result<()> {
    let encodings = ["utf-8", "latin-1"];
    for encoding in encodings.iter() {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        match Encoding::for_label(encoding.as_bytes()) {
            Some(enc) => {
                for line in reader.lines() {
                    let line = line?;
                    let (_, _, is_ascii) = enc.decode(line.as_bytes());
                    let decoded_line = if is_ascii {
                        line
                    } else {
                        String::from_utf8_lossy(&enc.decode(line.as_bytes()).0).into_owned()
                    };

                    let (found_dataleak, company_name) = search_keyword(&decoded_line);
                    if found_dataleak && company_name.is_some() {
                        let parse = Parser::new(&decoded_line.trim());
                        // Do something here
                    }
                }
                return Ok(());
            }
            None => continue
        }
    }

    // Err("Unable to decode file with supported encodings".into())
    Ok(())
}
