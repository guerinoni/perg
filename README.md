# perg
Grep implementation in rust.

## Feature

- [x] search in single file.
- [x] search in folder.
- [x] search in folder recursively.
- [ ] parallel search.

## Benchmarck 

Output of bash
```bash
time grep federico Cargo.toml
real    0m0,005s
user    0m0,004s
sys     0m0,001s

time grep --color=auto --exclude-dir={.git,.hg,.svn} -R federico .
real	0m0,061s
user	0m0,050s
sys	    0m0,009s


time perg federico ./Cargo.toml
real    0m0,004s
user    0m0,001s
sys     0m0,004s

time perg federico .
real	0m0,057s
user	0m0,050s
sys	    0m0,008s
```