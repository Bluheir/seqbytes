# seqbytes
The seqbytes crate provides the traits `ESeqByteReader` and `SeqByteReader` used for reading bytes sequentially. The `SeqByteReader` trait convert the bytes into the 
specified generic type, denoted `U`, which must implement `SizedNumber`. The trait `SizedNumber` represents a type which can be converted to and from bytes, with a
fixed size in bytes. 

The trait `ESeqByteReader` is used for reading bytes sequentially, converting to a type with a specific endianness. The type converted to must implement `EndianNumber`,
which represents a type which can be converted to and from bytes with a specific endianness.

## Implementations
The traits `ESeqByteReader` and `SeqByteReader` are implemented by default on types implementing `Read` + `Seek`.

## Example
```rust
use seqbytes::prelude::*;
use std::io::Cursor;

let a = vec![69, 96, 255, 255, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
let mut cursor = Cursor::new(a);

let num: i32 = cursor.shift().unwrap();
let s = &*cursor.shift_string(5).unwrap();

assert_eq!(num, -40891);
assert_eq!(*s, *"hello");
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
seqbytes = "0.1.0"
```

## Documentation
* [Docs.rs](https://docs.rs/seqbytes/0.1.0/)

## License

seqbytes is distributed under the Apache v2.0 License.

See [LICENSE.md](LICENSE.md).

