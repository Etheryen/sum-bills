# sum-bills

Sums all your bills if you keep them in a file in this format:

```
Bills start:
3€ shopping
2.35€ soda
-9.25€ borrowed from mark
```

Usage:
`cargo run -- STRING FILE`
STRING: line after which to sum all bills
FILE: has a section after STRING with lines that begin with numbers
