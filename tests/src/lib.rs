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
use std::ffi::CStr;
use std::os::raw::c_char;

use robotstxt::matcher::LongestMatchRobotsMatchStrategy;
use robotstxt::RobotsMatcher;

#[no_mangle]
pub extern "C" fn IsUserAgentAllowed(
    robotstxt: *const c_char,
    user_agent: *const c_char,
    url: *const c_char,
) -> bool {
    if let (Ok(robotstxt), Ok(user_agent), Ok(url)) = unsafe {
        (
            CStr::from_ptr(robotstxt).to_str(),
            CStr::from_ptr(user_agent).to_str(),
            CStr::from_ptr(url).to_str(),
        )
    } {
        println!("{} {} {}", robotstxt, user_agent, url);
        let user_agents = vec![user_agent.to_string()];
        let mut matcher = RobotsMatcher::<LongestMatchRobotsMatchStrategy>::default();
        matcher.allowed_by_robots(&robotstxt, user_agents, &url)
    } else {
        panic!("Invalid parameters");
    }
}
