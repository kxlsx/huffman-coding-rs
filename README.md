# **huffman-coding-rs**

This repository hosts my best efforts to create an implementation of the [Huffman coding algorithm](https://en.wikipedia.org/wiki/Huffman_coding), in rust.
It's split into two crates:

## [**huff_coding**][lib]

[![Crate][lib_crate_img]][lib_crate]

A library enabling one to create Huffman trees with any alphabet type they choose.

## [**huff**][bin]
  
  [![Crate][bin_crate_img]][bin_crate]
  
  Basic compression/decompression binary built on the library
  mentioned above.

## Contributing

I accept any changes, but i doubt anyone cares.

## Learn more about the Huffman Coding algorithm

Cool articles/videos about the *Huffman Coding* algorithm I found and learned from while working on this

- Articles
  - [Wikipedia](https://en.wikipedia.org/wiki/Huffman_coding)
  - [tutorialspoint](https://www.tutorialspoint.com/huffman-coding)
  - [Programiz](https://www.programiz.com/dsa/huffman-coding)
  - [GeeksforGeeks](https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/)
  - [Stack Exchange thread on *Huffman Tree* sizes](https://cs.stackexchange.com/questions/75542/maximum-size-of-huffman-codes-for-an-alphabet-containing-256-letters)
  - [*"Maximal codeword lengths in Huffman codes"* by Y.S.Abu-Mostafa and R.J.McEliece](https://www.sciencedirect.com/science/article/pii/S089812210000119X)
- Videos
  - [Tom Scott's The Basics](https://www.youtube.com/watch?v=JsTptu56GM8)
  - [Computerphile on *Huffman Trees*](https://www.youtube.com/watch?v=umTbivyJoiI)
  - [Computerphile on Compression](https://www.youtube.com/watch?v=Lto-ajuqW3w)
  - [Abdul Bari's video](https://www.youtube.com/watch?v=co4_ahEDCho)

[bin]:https://github.com/kxlsx/huffman-coding-rs/tree/master/huff
[bin_crate]:https://crates.io/crates/huff
[bin_crate_img]:https://img.shields.io/crates/v/huff.svg?logo=rust
[lib]:https://github.com/kxlsx/huffman-coding-rs/tree/master/huff_coding
[lib_crate]:https://crates.io/crates/huff_coding
[lib_crate_img]:https://img.shields.io/crates/v/huff_coding.svg?logo=rust
