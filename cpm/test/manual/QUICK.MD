# Quick Commands

## For testing

### Test continuity

1. cargo build && ..\..\target\debug\cpm.exe init -f
2. ./cpm build -t "C:\Dev\vcpkg" -dg "nt/msvc"
3. ./cpm build -t "C:\Dev\vcpkg" -dg "nt/msvc" -b

### Test release

1. cargo build --release && ..\..\target\release\cpm.exe init -f