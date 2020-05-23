# robotstxt

A native Rust port of [Google's robots.txt parser and matcher C++ library](https://github.com/google/robotstxt).

## Testings

```
$ git clone https://github.com/Folyd/robotstxt
Cloning into 'robotstxt'...
$ cd robotstxt/tests 
...
$ mkdir c-build && cd c-build
...
$ cmake
...
$ make
...
$ make test
Running tests...
Test project ~/robotstxt/tests/c-build
    Start 1: robots-test
1/1 Test #1: robots-test ......................   Passed    0.33 sec

```