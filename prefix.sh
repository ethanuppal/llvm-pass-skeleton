#!/bin/sh

check_dir() {
    [ -d "$1" ] && echo "$1" && exit 0
}

if [ -n "$LLVM_DIR" ]; then
    check_dir "$LLVM_DIR"
fi

check_dir "/opt/homebrew/opt/llvm@18"
check_dir "/opt/homebrew/opt/llvm"

check_dir "/usr/local/opt/llvm"
check_dir "/usr/lib/llvm-18/"

if command -v llvm-config >/dev/null 2>&1; then
    llvm-config --prefix
    exit 0
fi

for path in /usr/local /usr /opt/llvm; do
    if [ -f "$path/bin/llvm-config" ]; then
        "$path/bin/llvm-config" --prefix
        exit 0
    fi
done

exit 1
