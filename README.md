# perg

[![CI](https://github.com/guerinoni/perg/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/guerinoni/perg/actions/workflows/CI.yml)
[![codecov](https://codecov.io/gh/guerinoni/perg/branch/main/graph/badge.svg?token=A198N28TVV)](https://codecov.io/gh/guerinoni/perg)

Grep implementation in rust.

## Features

- [x] search in single file.
- [x] show line number (-n or --line-number)
- [x] ignore case sensitive (-i or --ignore-case)
- [x] search in more files (i.e. `perg file.txt file.txt`)
- [x] search from stdin (i.e. `perg -` or `perg lol -`)
- [x] search from stdin with pipe (i.e. `cat ./Cargo.toml | perg author`)
- [x] search in a directory.
- [x] search recursive (-r or --recursive)
- [x] search recursive following symlink (-R or --dereference-recursive)
- [x] exclude some dir (--exclude-dir=folder)

## Contributing

Any helps or suggestions will be appreciated.
