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
struct ParsedRobotsKey {
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

struct RobotsTxtParser<'a, Handler: RobotsParseHandler> {
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

    pub fn get_key_and_value_from() -> bool {
        false
    }

    pub fn need_escape_value_for_key(key: &ParsedRobotsKey) -> bool {
        match key.get_type() {
            ParseKeyType::UserAgent | ParseKeyType::Sitemap => false,
            _ => true,
        }
    }
}
