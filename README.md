# perg
Grep implementation in rust.

## Feature

- [x] search in single file.
- [ ] search in folder.
- [ ] search in folder recursively.
- [ ] parallel search.

## Benchmarck 

Output of bash
```bash
time grep federico Cargo.toml
authors = ["Federico Guerinoni <guerinoni.federico@gmail.com>"]

real    0m0,005s
user    0m0,000s
sys     0m0,005s



time perg federico ./Cargo.toml
authors = ["Federico Guerinoni <guerinoni.federico@gmail.com>"]

real    0m0,004s
user    0m0,001s
sys     0m0,004s
```

Output of zsh
```zsh
grep --color=auto --exclude-dir={.bzr,CVS,.git,.hg,.svn,.idea,.tox} federico   0,00s user 0,00s system 86% cpu 0,004 total

perg federico ./Cargo.toml  0,00s user 0,00s system 83% cpu 0,004 total
```