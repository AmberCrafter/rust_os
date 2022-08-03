Diary

[2022-08-03]
1. memory managerment: heap and allocator design

[2022-08-02]
1. memory managerment: paging

[2022-08-01]
1. modified the library structure. Inspired by apogeeoak.
ref. https://github.com/apogeeoak/os
2. finished test harness
Note. Sometimes we will set 
```
// Cargo.toml
[[test]]
name = {tests/filename}
harness = false
```
due to we need a ordering testing scenario.

[2022-07-31]
VGA Text Mode: Implement print to console

[2022-07-30]
Hello world!
1. init the environment
 - setup rust-toolchain
 - setup cargo.toml
 - setup target file
 - use no-std and no-main

2. need to modified ~/.cargo/cargo.toml
add
```
[build]
target = "x86_64-myrustos.json"

[unstable]
build-std = ["core", "compiler_builtins"]
```