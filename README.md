# 🎲 passphrase

Generate strong **diceware passphrases** and **random passwords** — with an **honest entropy report** so you know exactly how strong the result is. Written in Rust, OS-seeded CSPRNG, single static binary.

```bash
passphrase                        # 6-word passphrase, e.g. "iron-lake-frog-mint-..."
passphrase words -c 8 -C          # 8 capitalized words
passphrase pass -l 24             # 24-char password (upper+digits+symbols)
passphrase pass -l 32 --no-symbols
```

`pass` flags: `-l/--len` (1..=1024), `--no-upper`, `--no-digits`, `--no-symbols` (lowercase is always included). `words` takes `-c/--count` (1..=64), `-s/--sep`, `-C/--capitalize`.

The secret is printed to **stdout**; the entropy line goes to **stderr**, so `passphrase | pbcopy` copies only the secret.

```
$ passphrase words -c 6
grid-fuel-oath-lamp-vine-dusk
  ~48 bits of entropy (reasonable)
```

## Why entropy matters
Strength is computed from the actual pool: `bits = count · log2(N)`. Words come from an embedded list of ~250 words (~8 bits/word); passwords from the selected character classes. No hand-wavy "strong password" claims — just the math.

## Dev
```bash
cargo test              # pure logic, seeded-RNG determinism
cargo build --release   # ./target/release/passphrase
```

Core generation (`passphrase`, `password`, `entropy_bits`, `charset`) lives in `src/lib.rs` and is unit-tested with a seeded RNG.

MIT.
