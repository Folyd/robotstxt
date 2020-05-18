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