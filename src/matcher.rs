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

/// Instead of just maintaining a Boolean indicating whether a given line has
/// matched, we maintain a count of the maximum number of characters matched by
/// that pattern.
///
/// This structure stores the information associated with a match (e.g. when a
/// Disallow is matched) as priority of the match and line matching.
///
/// The priority is initialized with a negative value to make sure that a match
/// of priority 0 is higher priority than no match at all.
struct Match {
    priority: i32,
    line: u32,
}

impl Default for Match {
    fn default() -> Self {
        Match {
            priority: Self::NO_MATCH_PRIORITY,
            line: 0,
        }
    }
}

impl Match {
    const NO_MATCH_PRIORITY: i32 = -1;
    pub fn new(priority: i32, line: u32) -> Match {
        Match { priority, line }
    }

    pub fn set(&mut self, priority: i32, line: u32) {
        self.priority = priority;
        self.line = line;
    }

    pub fn clear(&mut self) {
        self.set(Self::NO_MATCH_PRIORITY, 0);
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn higher_priority_match<'a>(a: &'a Match, b: &'a Match) -> &'a Match {
        if a.priority() > b.priority() {
            a
        } else {
            b
        }
    }
}

#[derive(Default)]
struct MatchHierarchy {
    global: Match,
    specific: Match,
}

impl MatchHierarchy {
    pub fn clear(&mut self) {
        self.global.clear();
        self.specific.clear();
    }
}

/// RobotsMatcher - matches robots.txt against URLs.
///
/// The Matcher uses a default match strategy for Allow/Disallow patterns which
/// is the official way of Google crawler to match robots.txt. It is also
/// possible to provide a custom match strategy.
///
/// The entry point for the user is to call one of the *AllowedByRobots()
/// methods that return directly if a URL is being allowed according to the
/// robots.txt and the crawl agent.
/// The RobotsMatcher can be re-used for URLs/robots.txt but is not thread-safe.
trait RobotsMatchStrategy {
    fn match_allow(&self, path: &str, pattern: &str) -> i32;

    fn match_disallow(&self, path: &str, pattern: &str) -> i32;

    /// Returns true if URI path matches the specified pattern. Pattern is anchored
    /// at the beginning of path. '$' is special only at the end of pattern.
    ///
    /// Since 'path' and 'pattern' are both externally determined (by the webmaster),
    /// we make sure to have acceptable worst-case performance.
    fn matches(path: &str, pattern: &str) -> bool {
        true
    }
}

/// Implements the default robots.txt matching strategy. The maximum number of
/// characters matched by a pattern is returned as its match priority.
struct LongestMatchRobotsMatchStrategy {}

impl RobotsMatchStrategy for LongestMatchRobotsMatchStrategy {
    fn match_allow(&self, path: &str, pattern: &str) -> i32 {
        if Self::matches(path, pattern) {
            pattern.len() as i32
        } else {
            -1
        }
    }

    fn match_disallow(&self, path: &str, pattern: &str) -> i32 {
        if Self::matches(path, pattern) {
            pattern.len() as i32
        } else {
            -1
        }
    }
}

struct RobotsMatcher<S: RobotsMatchStrategy> {
    /// Characters of 'url' matching Allow.
    allow: MatchHierarchy,
    /// Characters of 'url' matching Disallow.
    disallow: MatchHierarchy,
    /// True if processing global agent rules.
    seen_global_agent: bool,
    /// True if processing our specific agent.
    seen_specific_agent: bool,
    /// True if we ever saw a block for our agent.
    ever_seen_specific_agent: bool,
    /// True if saw any key: value pair.
    seen_separator: bool,
    /// The path we want to pattern match. Not owned and only a valid pointer
    /// during the lifetime of *AllowedByRobots calls.
    path: String,
    /// The User-Agents we are interested in. Not owned and only a valid
    /// pointer during the lifetime of *AllowedByRobots calls.
    user_agents: Vec<String>,
    match_strategy: S,
}

impl<S: RobotsMatchStrategy> RobotsMatcher<S> {
    /// Initialize next path and user-agents to check. Path must contain only the
    /// path, params, and query (if any) of the url and must start with a '/'.
    fn init_user_agents_and_path(&mut self, user_agents: Vec<String>, path: String) {
        self.path = path;
        self.user_agents = user_agents;
    }

    /// Returns true if 'url' is allowed to be fetched by any member of the
    /// "user_agents" vector. 'url' must be %-encoded according to RFC3986.
    pub fn allowed_by_robots(
        &mut self,
        robots_body: &str,
        user_agents: Vec<String>,
        url: &str,
    ) -> bool
    where
        Self: RobotsParseHandler,
    {
        // The url is not normalized (escaped, percent encoded) here because the user
        // is asked to provide it in escaped form already.
        let path = super::get_path_params_query(url);
        self.init_user_agents_and_path(user_agents, path);
        super::parse_robotstxt(robots_body, self);
        !self.disallow()
    }

    /// Do robots check for 'url' when there is only one user agent. 'url' must
    /// be %-encoded according to RFC3986.
    pub fn one_agent_allowed_by_robots(
        &mut self,
        robots_txt: &str,
        user_agent: &str,
        url: &str,
    ) -> bool
    where
        Self: RobotsParseHandler,
    {
        self.allowed_by_robots(robots_txt, vec![user_agent.to_string()], url)
    }

    /// Returns true if we are disallowed from crawling a matching URI.
    fn disallow(&self) -> bool {
        if self.allow.specific.priority() > 0 || self.disallow.specific.priority() > 0 {
            return self.disallow.specific.priority() > self.allow.specific.priority();
        }

        if self.ever_seen_specific_agent {
            // Matching group for user-agent but either without disallow or empty one,
            // i.e. priority == 0.
            return false;
        }

        if self.disallow.global.priority() > 0 || self.allow.global.priority() > 0 {
            return self.disallow.global.priority() > self.allow.global.priority();
        }

        false
    }

    /// Returns true if any user-agent was seen.
    fn seen_any_agent(&self) -> bool {
        self.seen_global_agent || self.seen_specific_agent
    }

    fn extract_user_agent(user_agent: &str) -> &str {
        ""
    }

    /// Verifies that the given user agent is valid to be matched against
    /// robots.txt. Valid user agent strings only contain the characters
    /// [a-zA-Z_-].
    fn is_valid_user_agent_to_obey(user_agent: &str) -> bool {
        !user_agent.is_empty() && Self::extract_user_agent(user_agent) == user_agent
    }
}

impl<S: RobotsMatchStrategy> RobotsParseHandler for &mut RobotsMatcher<S> {
    fn handle_robots_start(&mut self) {
        // This is a new robots.txt file, so we need to reset all the instance member
        // variables. We do it in the same order the instance member variables are
        // declared, so it's easier to keep track of which ones we have (or maybe
        // haven't!) done.
        self.allow.clear();
        self.disallow.clear();

        self.seen_global_agent = false;
        self.seen_specific_agent = false;
        self.ever_seen_specific_agent = false;
        self.seen_separator = false;
    }

    fn handle_robots_end(&mut self) {}

    fn handle_user_agent(&mut self, line_num: u32, user_agent: &str) {
        if self.seen_separator {
            self.seen_specific_agent = false;
            self.seen_global_agent = false;
            self.seen_separator = false;
        }

        // Google-specific optimization: a '*' followed by space and more characters
        // in a user-agent record is still regarded a global rule.
        let p = user_agent.get(..1).unwrap();
        if !user_agent.is_empty() && p == "*" && (user_agent.len() == 1 || p.is_empty()) {
            self.seen_global_agent = true;
        } else {
            let user_agent = RobotsMatcher::<S>::extract_user_agent(user_agent);
            for agent in &self.user_agents {
                if user_agent.eq_ignore_ascii_case(&agent) {
                    self.ever_seen_specific_agent = true;
                    self.seen_specific_agent = true;
                    break;
                }
            }
        }
    }

    fn handle_allow(&mut self, line_num: u32, value: &str) {
        if !self.seen_any_agent() {
            return;
        }

        self.seen_separator = true;
        let priority = self.match_strategy.match_disallow(&self.path, value);
        if priority >= 0 {
            if self.seen_specific_agent {
                if self.allow.specific.priority() < priority {
                    self.allow.specific.set(priority, line_num);
                }
            } else {
                if self.allow.global.priority() < priority {
                    self.allow.global.set(priority, line_num);
                }
            }
        } else {
            // TODO
        }
    }

    fn handle_disallow(&mut self, line_num: u32, value: &str) {
        if !self.seen_any_agent() {
            return;
        }

        self.seen_separator = true;
        let priority = self.match_strategy.match_disallow(&self.path, value);
        if priority >= 0 {
            if self.seen_specific_agent {
                if self.disallow.specific.priority() < priority {
                    self.disallow.specific.set(priority, line_num);
                }
            } else {
                if self.disallow.global.priority() < priority {
                    self.disallow.global.set(priority, line_num);
                }
            }
        }
    }

    fn handle_sitemap(&mut self, line_num: u32, value: &str) {
        self.seen_separator = true;
    }

    fn handle_unknown_action(&mut self, line_num: u32, action: &str, value: &str) {
        self.seen_separator = true;
    }
}
