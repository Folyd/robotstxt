use robotstxt::matcher::LongestMatchRobotsMatchStrategy;
use robotstxt::RobotsMatcher;

#[no_mangle]
pub extern "C" fn IsUserAgentAllowed(robotstxt: &str, user_agent: &str, url: &str) -> bool {
    let user_agents = vec![user_agent.to_string()];
    let mut matcher = RobotsMatcher::<LongestMatchRobotsMatchStrategy>::default();
    matcher.allowed_by_robots(&robotstxt, user_agents, &url)
}
