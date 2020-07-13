# string interner comparsion

Usage:

```
# To collect samples
cargo run -q -- sample --lib <lib> 2> events.ldjson

# To collect allocation event stats
cargo run -q -- report events.ldjson

# To plot allocation against words
py ./plot.py
# This will require editing the script to get ideal results.
# Requires numpy and matplotlib to be installed.
```

This is messy, hacky, exploratative code, published solely so others can reproduce measurement results.

## License

Licensed under any of

- [MIT](http://opensource.org/licenses/MIT),
- [APACHE-2.0](http://www.apache.org/licenses/LICENSE-2.0), or
- [UNLICENSE](https://choosealicense.com/licenses/unlicense/)

at your option.
