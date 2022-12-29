# Reckless Magics

[![Rust](https://github.com/codedeliveryservice/RecklessMagics/actions/workflows/rust.yml/badge.svg)](https://github.com/codedeliveryservice/RecklessMagics/actions/workflows/rust.yml)

Implementation of the magic number generation algorithm used for [Fancy Magic Bitboards][fancy-bitboards].

## Implementation

The generator is based on a modified [Tord Romstad's proposal][proposal], originally written for [Plain Magic Bitboards][plain-bitboards].

[Fancy Magic Bitboards][fancy-bitboards] is an improved version with individual table sizes for each square, which significantly reduces the overall size by eliminating empty elements (841 KiB against 2304 KiB).

Each generated magic entry, in addition to the `magic` number used as a hash factor, also contains an `offset` used to refer to a particular table of the specified square. To speed up the lookup, the `mask` of the relevant occupancy bits and the `offset` pre-calculated as `64 - relevant_bit_count` are also stored as part of the entry.

For example, the output of the first 3 entries for rooks is:

```rust
MagicEntry { mask: 0x000101010101017E, magic: 0x1080018022704002, shift: 52, offset: 0 },
MagicEntry { mask: 0x000202020202027C, magic: 0x8B4000A005900840, shift: 53, offset: 4096 },
MagicEntry { mask: 0x000404040404047A, magic: 0x09000A4100200012, shift: 53, offset: 6144 },
```

With this information, very few operations are required to get a magic index for quick lookup:

```rust
let entry = MagicEntry { ... };
// Mask occupancies to get relevant bits
let mut hash = occupancies & entry.mask;
// Get the magic index for an individual table
hash = hash.wrapping_mul(entry.magic) >> entry.shift;
// Get the resulting table index
let index = hash as usize + entry.offset;
```

## License

This project is licensed with the [MIT license](LICENSE).

[proposal]: https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms
[fancy-bitboards]: https://www.chessprogramming.org/Magic_Bitboards#Fancy
[plain-bitboards]: https://www.chessprogramming.org/Magic_Bitboards#Plane
