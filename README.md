A completely useless LLVM pass. It's for LLVM 18. This is the Rust version of
[sampsyo/llvm-pass-skeleton](https://github.com/sampsyo/llvm-pass-skeleton).

See the `mutate` branch for a slightly less skeletal example.

1. Build:
```shell
export LLVM_SYS_180_PREFIX=$(/bin/sh prefix.sh)
cargo build || (cargo clean && cargo build)
```

2. Generate IR (replace `test.c` with any other C/C++ file you want to test):
```
$LLVM_SYS_180_PREFIX/bin/clang -S -emit-llvm -o out.ll test.c
```

3. Run:
   - **macOS**:
     ```shell
     $LLVM_SYS_180_PREFIX/bin/opt --load-pass-plugin="target/debug/libskeleton_pass.dylib" --passes=skeleton-pass -disable-output out.ll
     ```
   - **Linux**:
     ```shell
     $LLVM_SYS_180_PREFIX/bin/opt --load-pass-plugin="target/debug/libskeleton_pass.so" --passes=skeleton-pass -disable-output out.ll
     ```

It can be useful to put these commands in a `Makefile`, `Justfile`, or shell
script.

This pass is verified under continuous integration to work on macOS and Linux.
