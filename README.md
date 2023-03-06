## Try tuple to array
Rust library for converting tuple to array.

## Description
A generic trait for converting a tuple of different types into an array of one type.

#### Add this to your Cargo.toml
```rust,ignore
[dependencies]
try_tup_to_arr = { version = "0.1.0", git = "https://github.com/pic16f877ccs/try_tup_to_arr" }
```
#### Or using cargo
```rust,ignore
cargo add try_tup_to_arr --git "https://github.com/pic16f877ccs/try_tup_to_arr"
```
#### Examples
Usage:

```
# use try_tup_to_arr::TryTupToArr;
assert_eq!(TryTupToArr::<i32>::try_into_arr((45u8, 2023u16, -60i8,)),
Ok([45i32, 2023i32, -60i32]));
assert_eq!(TryTupToArr::<i16>::try_into_arr(("45", 2023u16, true,)),
Ok([45i16, 2023i16, 1i16]));
assert_eq!(TryTupToArr::try_into_arr((45u8, 2023u16, -53i8,))
.unwrap().iter().sum::<i32>(), 2015i32);
assert!(TryTupToArr::<i16>::try_into_arr(("6032023", 2023u16, true,)).is_err());
```

## License
GNU General Public License v3.0
