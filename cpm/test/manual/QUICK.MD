# Quick Commands

## For testing

### Test continuity

#### Windows

1. cargo build && ..\..\target\debug\cpm.exe init -f
2. cargo build && ./cpm setup -a
3. ./cpm build -t "C:\Dev\vcpkg" -dg "nt/msvc"
4. ./cpm build -t "C:\Dev\vcpkg" -dg "nt/msvc" -b

#### Linux

1. cargo build && ../../target/debug/cpm init -f
2. cargo build && ./cpm.sh setup -a
3. ./cpm.sh build -dg "unix/clang"
4. ./cpm.sh build -c -dg -bi

#### OSX

1. cargo build && ../../target/debug/cpm init -f
2. cargo build && ./cpm.sh setup -a
3. ./cpm.sh build -dg "unix/clang"
4. ./cpm.sh build -c -dg -bi

### Test release

1. cargo build --release && ..\..\target\release\cpm.exe init -f