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
