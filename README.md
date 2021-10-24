# ABar

ABar is an interface for generating dynamic "blocks"-style status strings.

When using minimalistic status bars, you'll generally define a number of modules
and then stick them together somehow. For instance, your bar might look a bit
like this:

```
>>> VOLUME | BATTERY | CURRENT_TIME <<<
```

This can come with a performance hit, however, as normally you'd have to update
everything all at once. Every time you want to refresh the battery, you'd also
have to refresh the volume, etc. This might seem fine, but if you ever want to
rely upon something slow (such as an HTTP request) your entire bar will slow to
a crawl. ABar solves this problem by giving each module its own unique update
cycle, monitored by a small thread pool.


## What's different from other blocks-style implementations?

ABar takes inspiration from the tools you might see over at
[suckless.org](https://suckless.org) in that it's designed to be hackable.
There's no configuration file, and instead the idea is that you modify your bar
by directly toying with the source code. This results in the potential for an
*incredibly* fast, powerful status bar that feels truly your own.

[dwmblocks](https://github.com/torrinfail/dwmblocks) is a project with a similar
goal. It's fantastic, but ABar differs in a few ways:

1. ABar is both a library *and* a binary. This means that you can either modify
   a fork of this repository, or create an entirely new project using the
   interface available on [crates.io](https://crates.io).

2. It's in Rust. While Rust is definitely harder to work with than C, you also
   have access to the `cargo` package manager. Cargo is fantastic, and can allow
   a newcomer to do some *very* advanced things in *raw* rust (skipping shell
   scripts entirely). For many people, this alone can make abar the fastest
   status bar available.

3. In general, dwmblocks focuses more on integration with shell scripts, while
   abar focuses more on the customisability offered by direct access to the
   source code.


## Installation

Running `cargo install --path .` from the source directory will install the
binary, while automatically pulling any dependencies you've defined in
`./Cargo.toml`


## Usage

The main "configuration" happens in `./src/config.rs`, which contains some
examples to guide you through the basics. That being said, I strongly recommend
"looking under the hood" and finding something to tinker with.

More specific info can be found in the [documentation](https://docs.rs/abar).
