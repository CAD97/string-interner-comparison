# string interner comparsion

## Usage

### To collect samples

```
cargo sample <lib> 2> events.ldjson
```

Possible values for `<lib>` are:

- `std`: Baseline, collecting into `Vec<String>`.
- `interner`: Testing the [`string-interner`](https://crates.io/crates/string-interner) crate with default backend.
    - `interner_bucket`: To use the `BucketBackend` of the `string-interner` (default)
    - `interner_string`: To use the `StringBackend` of the `string-interner`
    - `interner_buffer`: To use the `BufferBackend` of the `string-interner`
- `lasso`: Testing the [`lasso`](https://crates.io/crates/lasso) crate.
- `larlpop`: Testing the [`lalrpop-intern`](https://crates.io/crates/lalrpop-intern) crate.
- `intaglio`: Testing the [`intaglio`](https://crates.io/crates/intaglio) crate.
    - `intanglio_dyn`: Use this to prevent `intanglio` to optimize for `&'static str`.
- `cargo`: Testing [`cargo`](https://crates.io/crates/cargo)'s internal string interner.
- `strena`: Testing [`strena`](https://github.com/CAD97/strena) research project.
    - `strena_new`: To explicity test `strena` using `Interner::new`.
    - `strena_with_capacity`: To explicity test `strena` with known capacity for best possible memory consumption.

### To collect allocation event stats

```
cargo report events.ldjson
```

### To plot allocation against words

```
python ./plot.py
```

Or alternatively (mostly on Unix) just execute it.

This will require editing the script to get ideal results.
Requires `numpy` and `matplotlib` to be installed.

This is messy, hacky, exploratative code, published solely so others can reproduce measurement results.

## License

Licensed under any of

- [MIT](http://opensource.org/licenses/MIT),
- [APACHE-2.0](http://www.apache.org/licenses/LICENSE-2.0), or
- [UNLICENSE](https://choosealicense.com/licenses/unlicense/)

at your option.
