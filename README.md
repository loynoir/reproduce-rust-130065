# reproduce

tested able to reproduce using docker with ubuntu image

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
...
$ bash
```

```sh
$ cargo install rustfilt
...
```

```sh
$ rm -rf build; git restore . && bash -x test.sh
...
+ /path/to/llvm-cov show ...
warning: 1 functions have mismatched data
```

```sh
$ rm -rf build; git restore . && sed -i 3d crates/feature_str_from_raw_parts_util/src/lib.rs && bash -x test.sh && git restore .
(no warning)
```
