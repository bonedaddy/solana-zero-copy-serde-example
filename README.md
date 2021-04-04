This is minimal example of `rkyv` zero copy input/modify/read for Solana BPF programs.

Uncomment `init_borsh` to get `thread 'solana-bank-forks-client' has overflowed its stack`

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```