# A minimal app to reproduce "could not get parent of requiring context" problem

**Environment:**

- Ubuntu 22.04.5 LTS
- cargo 1.87.0 (99624be96 2025-05-06)
- rustc 1.87.0 (17067e9ac 2025-05-09)

**Problem:**

The `require` function doesn't work from sub-package. This problem relates to a project structure, not to the `mlua` package itself.

**Steps to reproduce:**

1. Try to execute:

```
cargo test
```

2. You will get an error:

```
$
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (target/debug/deps/luau_require-1432f1968da1336c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/require.rs (target/debug/deps/require-f9b7d047072efaff)

running 1 test
test test_require_small ... FAILED

failures:

---- test_require_small stdout ----
The current directory is /home/ipavlenko/Projects/Develfish/example-luau/luau_require

thread 'test_require_small' panicked at luau_require/tests/require.rs:23:72:
called `Result::unwrap()` on an `Err` value: RuntimeError("error requiring module \"./require/without_config/dependency\": could not get parent of requiring context\nstack traceback:\n\t[C]: in ?\n\t[C]: in function 'proxyrequire'\n\t__mlua_require:13: in ?\n\tluau_require/tests/require.rs:6:1: in ?")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_require_small

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test require`
```

