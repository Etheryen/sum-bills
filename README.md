# sum-bills

Sums all your bills if you keep them in a file with this format:

```
Bills start:
3€ shopping
2.35€ soda
-9.25€ borrowed from mark
```

## Usage

### Go

```sh
go run cmd/main.go --header "Bills start:" --file ~/personal/finances/Bills.md
```

> Go version also prints out detected currency

### Rust

```sh
cargo run -- "Bills start:" ~/personal/finances/Bills.md
```
