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

#![allow(unused_variables)]

use url::Url;

use parser::RobotsTxtParser;

pub mod matcher;
pub mod parser;

/// Handler for directives found in robots.txt.
pub trait RobotsParseHandler {
    fn handle_robots_start(&mut self);
    fn handle_robots_end(&mut self);
    fn handle_user_agent(&mut self, line_num: u32, user_agent: &str);
    fn handle_allow(&mut self, line_num: u32, value: &str);
    fn handle_disallow(&mut self, line_num: u32, value: &str);
    fn handle_sitemap(&mut self, line_num: u32, value: &str);
    /// Any other unrecognized name/value pairs.
    fn handle_unknown_action(&mut self, line_num: u32, action: &str, value: &str);
}

/// get_path_params_query is not in anonymous namespace to allow testing.
///
/// Extracts path (with params) and query part from URL. Removes scheme,
/// authority, and fragment. Result always starts with "/".
/// Returns "/" if the url doesn't have a path or is not valid.
pub fn get_path_params_query(url: &str) -> String {
    Url::parse(url).map_or("/".into(), |url| url.path().to_string())
}

/// Parses body of a robots.txt and emits parse callbacks. This will accept
/// typical typos found in robots.txt, such as 'disalow'.
///
/// Note, this function will accept all kind of input but will skip
/// everything that does not look like a robots directive.
pub fn parse_robotstxt(robots_body: &str, parse_callback: &mut impl RobotsParseHandler) {
    let mut parser = RobotsTxtParser::new(robots_body, parse_callback);
    parser.parse();
}
