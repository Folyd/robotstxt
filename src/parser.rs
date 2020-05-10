// Copyright 2020 Folyd
// Copyright 1999 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#![allow(unused_variables, dead_code)]

use crate::RobotsParseHandler;

#[derive(Eq, PartialEq)]
pub enum ParseKeyType {
    /// Generic highlevel fields.
    UserAgent,
    Sitemap,

    /// Fields within a user-agent.
    Allow,
    Disallow,

    /// Unrecognized field; kept as-is. High number so that additions to the
    /// enumeration above does not change the serialization.
    Unknown = 128,
}

/// A robots.txt has lines of key/value pairs. A ParsedRobotsKey represents
/// a key. This class can parse a text-representation (including common typos)
/// and represent them as an enumeration which allows for faster processing
/// afterwards.
/// For unparsable keys, the original string representation is kept.
pub struct ParsedRobotsKey {
    type_: ParseKeyType,
    key_text: String,
    /// Allow for typos such as DISALOW in robots.txt.
    allow_typo: bool,
}

impl Default for ParsedRobotsKey {
    fn default() -> Self {
        ParsedRobotsKey {
            type_: ParseKeyType::Unknown,
            allow_typo: true,
            key_text: String::new(),
        }
    }
}

impl ParsedRobotsKey {
    /// Parse given key text. Does not copy the text, so the text_key must stay
    /// valid for the object's life-time or the next Parse() call.
    fn parse(&mut self, key: &str) {
        if self.validate_key(key, &["user-agent"], Some(&["useragent", "user agent"])) {
            self.type_ = ParseKeyType::UserAgent;
        } else if self.validate_key(key, &["allow"], None) {
            self.type_ = ParseKeyType::Allow;
        } else if self.validate_key(
            key,
            &["disallow"],
            Some(&["dissallow", "dissalow", "disalow", "diasllow", "disallaw"]),
        ) {
            self.type_ = ParseKeyType::Disallow;
        } else if self.validate_key(key, &["sitemap", "site-map"], None) {
            self.type_ = ParseKeyType::Sitemap;
        } else {
            self.type_ = ParseKeyType::Unknown;
            self.key_text = key.to_string();
        }
    }

    /// Returns the type of key.
    fn get_type(&self) -> &ParseKeyType {
        &self.type_
    }

    /// If this is an unknown key, get the text.
    fn get_unknown_text(&self) -> String {
        assert!(self.type_ == ParseKeyType::Unknown && self.key_text.is_empty());
        self.key_text.to_string()
    }

    fn validate_key(&self, key: &str, targets: &[&str], typo_targets: Option<&[&str]>) -> bool {
        let key = key.to_lowercase();
        let check = |target: &&str| key.starts_with(&target.to_lowercase());
        targets.iter().any(check)
            || (typo_targets.is_some()
                && self.allow_typo
                && typo_targets.unwrap().iter().any(check))
    }
}

pub struct RobotsTxtParser<'a, Handler: RobotsParseHandler> {
    robots_body: &'a str,
    handler: &'a mut Handler,
}

impl<'a, Handler: RobotsParseHandler> RobotsTxtParser<'a, Handler> {
    pub fn new(robots_body: &'a str, handler: &'a mut Handler) -> Self {
        RobotsTxtParser {
            robots_body,
            handler,
        }
    }

    pub fn parse(&mut self) {
        self.handler.handle_robots_start();

        self.handler.handle_robots_end();
    }

    /// Attempts to parse a line of robots.txt into a key/value pair.
    ///
    /// On success, the parsed key and value, and true, are returned. If parsing is
    /// unsuccessful, parseKeyAndValue returns two empty strings and false.
    pub fn parse_key_value(line: &str) -> (&str, &str, bool) {
        let mut line = line;
        // Remove comments from the current robots.txt line.
        if let Some(comment) = line.find('#') {
            line = &line[..comment].trim();
        }

        // Rules must match the following pattern:
        //   <key>[ \t]*:[ \t]*<value>
        let mut sep = line.find(':');
        if sep.is_none() {
            // Google-specific optimization: some people forget the colon, so we need to
            // accept whitespace in its stead.
            let white = " \t";

            sep = line.find(|c| white.contains(c));
            if let Some(sep) = sep {
                let val = &line[sep..].trim();
                // since we dropped trailing whitespace above.
                assert!(val.len() > 0);

                if val.find(|c| white.contains(c)).is_some() {
                    // We only accept whitespace as a separator if there are exactly two
                    // sequences of non-whitespace characters.  If we get here, there were
                    // more than 2 such sequences since we stripped trailing whitespace
                    // above.
                    return ("", "", false);
                }
            }
        }

        if let Some(sep) = sep {
            // Key starts at beginning of line.
            let key = &line[..sep];
            if key.len() == 0 {
                return ("", "", false);
            }

            // Value starts after the separator.
            let value = &line[(sep + 1)..];
            (key.trim(), value.trim(), true)
        } else {
            // Couldn't find a separator.
            ("", "", false)
        }
    }

    pub fn need_escape_value_for_key(key: &ParsedRobotsKey) -> bool {
        match key.get_type() {
            ParseKeyType::UserAgent | ParseKeyType::Sitemap => false,
            _ => true,
        }
    }

    fn parse_and_emit_line(&mut self, current_line: u32, line: &str) {
        match Self::parse_key_value(line) {
            (_, _, false) => {}
            (string_key, value, true) => {
                let mut key = ParsedRobotsKey::default();
                key.parse(string_key);
                if Self::need_escape_value_for_key(&key) {
                    let value = escape_pattern(value);
                    self.emit(current_line, &key, &value);
                } else {
                    self.emit(current_line, &key, value);
                }
            }
        }
    }

    fn emit(&mut self, line: u32, key: &ParsedRobotsKey, value: &str) {
        match key.get_type() {
            ParseKeyType::UserAgent => self.handler.handle_user_agent(line, value),
            ParseKeyType::Sitemap => self.handler.handle_sitemap(line, value),
            ParseKeyType::Allow => self.handler.handle_allow(line, value),
            ParseKeyType::Disallow => self.handler.handle_disallow(line, value),
            ParseKeyType::Unknown => {
                self.handler
                    .handle_unknown_action(line, &key.get_unknown_text(), value)
            }
        }
    }
}

const HEX_DIGITS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Canonicalize the allowed/disallowed path patterns.
/// UTF-8 multibyte sequences (and other out-of-range ASCII values) are percent-encoded,
/// and any existing percent-encoded values have their hex values normalised to uppercase.
///
/// For example:
///     /SanJoséSellers ==> /Sanjos%C3%A9Sellers
///     %aa ==> %AA
/// If the given path pattern is already adequately escaped,
/// the original string is returned unchanged.
pub fn escape_pattern(path: &str) -> String {
    let mut num_to_escape = 0;
    let mut need_capitalize = false;

    // First, scan the buffer to see if changes are needed. Most don't.
    let mut i = 0;
    let mut chars = path.chars();
    loop {
        match chars.nth(i) {
            // (a) % escape sequence.
            Some(c) if c == '%' => match (chars.nth(i + 1), chars.nth(i + 2)) {
                (Some(c1), Some(c2)) if c1.is_digit(16) && c2.is_digit(16) => {
                    if c1.is_ascii_lowercase() || c2.is_ascii_lowercase() {
                        need_capitalize = true;
                    }
                    i += 2;
                }
                _ => {}
            },
            Some(c) if c as i32 >= 0x80 => {
                // (b) needs escaping.
                num_to_escape += 1;
            }
            o => {
                // (c) Already escaped and escape-characters normalized (eg. %2f -> %2F).
                if o.is_none() {
                    break;
                }
            }
        }
        i += 1;
    }
    // Return if no changes needed.
    if num_to_escape == 0 && !need_capitalize {
        return path.to_string();
    }

    i = 0;
    let mut dest = String::with_capacity(num_to_escape * 2 + path.len() + 1);
    chars = path.chars();
    loop {
        match chars.nth(i) {
            Some(c) if c == '%' => {
                // (a) Normalize %-escaped sequence (eg. %2f -> %2F).
                match (chars.nth(i + 1), chars.nth(i + 2)) {
                    (Some(c1), Some(c2)) if c1.is_digit(16) && c2.is_digit(16) => {
                        dest.push(c);
                        dest.push(c1.to_ascii_uppercase());
                        dest.push(c2.to_ascii_uppercase());
                        i += 2;
                    }
                    _ => {}
                }
            }
            Some(c) if c as i32 >= 0x80 => {
                // (b) %-escape octets whose highest bit is set. These are outside the ASCII range.
                dest.push('%');
                dest.push(HEX_DIGITS[(c as usize >> 4) & 0xf]);
                dest.push(HEX_DIGITS[c as usize & 0xf]);
            }
            Some(c) => {
                // (c) Normal character, no modification needed.
                dest.push(c);
            }
            None => {
                break;
            }
        }
        i += 1;
    }
    dest
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::RobotsParseHandler;

    struct FooHandler;

    impl RobotsParseHandler for FooHandler {
        fn handle_robots_start(&mut self) {
            unimplemented!()
        }

        fn handle_robots_end(&mut self) {
            unimplemented!()
        }

        fn handle_user_agent(&mut self, line_num: u32, user_agent: &str) {
            unimplemented!()
        }

        fn handle_allow(&mut self, line_num: u32, value: &str) {
            unimplemented!()
        }

        fn handle_disallow(&mut self, line_num: u32, value: &str) {
            unimplemented!()
        }

        fn handle_sitemap(&mut self, line_num: u32, value: &str) {
            unimplemented!()
        }

        fn handle_unknown_action(&mut self, line_num: u32, action: &str, value: &str) {
            unimplemented!()
        }
    }

    #[test]
    fn test_parse_key_value<'a>() {
        type Target<'a> = RobotsTxtParser<'a, FooHandler>;
        let negative = ("", "", false);
        let positive = ("User-agent", "Googlebot", true);

        assert_eq!(negative, Target::parse_key_value("# "));
        assert_eq!(negative, Target::parse_key_value("# User-agent: Googlebot"));

        assert_eq!(positive, Target::parse_key_value("User-agent: Googlebot"));
        assert_eq!(positive, Target::parse_key_value("User-agent  Googlebot"));
        assert_eq!(positive, Target::parse_key_value("User-agent \t Googlebot"));
        assert_eq!(positive, Target::parse_key_value("User-agent\tGooglebot"));
        assert_eq!(
            positive,
            Target::parse_key_value("User-agent: Googlebot # 123")
        );
        assert_eq!(
            positive,
            Target::parse_key_value("User-agent\tGooglebot # 123")
        );
    }
}
