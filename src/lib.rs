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

mod matcher;
mod parser;

/// Handler for directives found in robots.txt.
pub trait RobotsParseHandler {
    fn handle_robots_start(&mut self);
    fn handle_robots_end(&mut self);
    fn handle_user_agent(&mut self, line_num: u32, value: &str);
    fn handle_allow(&mut self, line_num: u32, value: &str);
    fn handle_disallow(&mut self, line_num: u32, value: &str);
    fn handle_sitemap(&mut self, line_num: u32, value: &str);
    /// Any other unrecognized name/value pairs.
    fn handle_unknown_action(&mut self, line_num: u32, action: &str, value: &str);
}
