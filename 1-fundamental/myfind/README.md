# Myfind

A rust practice to find a file in a certain directory.

How to use?

```bash
cargo build --release
```

then type `./target/release/myfind -h` for helps.

Here are some examples:

```bash
./target/release/myfind ./ main.rs
```

or

```bash
./target/release/myfind ./ main.rs -v
```

or

```bash
./target/release/myfind ./ main.rs -v -o out.txt
```

