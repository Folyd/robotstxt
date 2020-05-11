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

pub use matcher::RobotsMatcher;
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

/// Extracts path (with params) and query part from URL. Removes scheme,
/// authority, and fragment. Result always starts with "/".
/// Returns "/" if the url doesn't have a path or is not valid.
pub fn get_path_params_query(url: &str) -> String {
    fn find_first_of(s: &str, pattern: &str, start_position: usize) -> Option<usize> {
        s[start_position..]
            .find(|c| pattern.contains(c))
            .map(|pos| pos + start_position)
    }
    fn find(s: &str, pattern: &str, start_position: usize) -> Option<usize> {
        s[start_position..]
            .find(pattern)
            .map(|pos| pos + start_position)
    }

    let mut search_start = 0;
    // Initial two slashes are ignored.
    if url.len() >= 2 && url.get(..2) == Some("//") {
        search_start = 2;
    }
    let early_path = find_first_of(url, "/?;", search_start);
    let mut protocol_end = find(url, "://", search_start);

    if early_path.is_some() && early_path < protocol_end {
        // If path, param or query starts before ://, :// doesn't indicate protocol.
        protocol_end = None;
    }
    if protocol_end.is_none() {
        protocol_end = Some(search_start);
    } else {
        protocol_end = protocol_end.map(|pos| pos + 3)
    }

    if let Some(path_start) = find_first_of(url, "/?;", protocol_end.unwrap()) {
        let hash_pos = find(url, "#", search_start);
        if hash_pos.is_some() && hash_pos.unwrap() < path_start {
            return "/".into();
        }

        let path_end = if hash_pos.is_none() {
            url.len()
        } else {
            hash_pos.unwrap()
        };
        if url.get(path_start..=path_start) != Some("/") {
            // Prepend a slash if the result would start e.g. with '?'.
            return format!("/{}", &url[path_start..path_end]);
        }
        return String::from(&url[path_start..path_end]);
    }

    "/".into()
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

#[cfg(test)]
mod tests {
    use super::get_path_params_query;

    #[test]
    fn test_get_path_params_query() {
        let f = get_path_params_query;
        assert_eq!("/", f(""));
        assert_eq!("/", f("http://www.example.com"));
        assert_eq!("/", f("http://www.example.com/"));
        assert_eq!("/a", f("http://www.example.com/a"));
        assert_eq!("/a/", f("http://www.example.com/a/"));
        assert_eq!(
            "/a/b?c=http://d.e/",
            f("http://www.example.com/a/b?c=http://d.e/")
        );
        assert_eq!(
            "/a/b?c=d&e=f",
            f("http://www.example.com/a/b?c=d&e=f#fragment")
        );
        assert_eq!("/", f("example.com"));
        assert_eq!("/", f("example.com/"));
        assert_eq!("/a", f("example.com/a"));
        assert_eq!("/a/", f("example.com/a/"));
        assert_eq!("/a/b?c=d&e=f", f("example.com/a/b?c=d&e=f#fragment"));
        assert_eq!("/", f("a"));
        assert_eq!("/", f("a/"));
        assert_eq!("/a", f("/a"));
        assert_eq!("/b", f("a/b"));
        assert_eq!("/?a", f("example.com?a"));
        assert_eq!("/a;b", f("example.com/a;b#c"));
        assert_eq!("/b/c", f("//a/b/c"));
    }
}
