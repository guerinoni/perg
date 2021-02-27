# perg
Grep implementation in rust.

## Feature

- [x] search in single file.
- [x] search in folder.
- [x] search in folder recursively.
- [x] parallel search.
- [ ] search in hidden folder.

## Benchmarck 

```zsh
time grep -R federico /usr
14,15s user 7,25s system 54% cpu 39,089 total

time perg federico /usr
21,04s user 4,96s system 750% cpu 3,467 total
```

```bash
time grep -R federico /usr
real    0m18,160s
user    0m14,086s
sys     0m3,385s

time perg federico /usr
real    0m6,337s
user    0m27,545s
sys     0m9,840s
```

## Contributing

Any helps or suggestions will be appreciated.