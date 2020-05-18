#include "rust_robotstxt.h"
#include <string>
#include <iostream>

using namespace std;

int main() {
    const char *robotstxt =
            "user-agent: FooBot\n"
            "disallow: /\n";
    bool result = IsUserAgentAllowed(robotstxt, "FooBot", "");
    cout << result << endl;
}