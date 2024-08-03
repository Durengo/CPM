# Quick Commands

## For testing

### Test continuity

#### Windows

1. Make sure to boot into WSL and execute commands from there
2. cargo build && ../../target/debug/cpm init -f
3. cargo build && ./cpm setup -a
4. ./cpm.sh cross-build -dg "rpi4/umake" --processor "aarch64" --sysroot "/opt/rpi-sysroot" -b

Also test regular build so that it does not cache things:

1. ./cpm.sh build -dg "unix/clang"

#### Linux

X

#### OSX

X

### Test release

X