# rusty-marcher  [![Build Status](https://travis-ci.org/blefaudeux/rusty-marcher.svg?branch=master)](https://travis-ci.org/blefaudeux/rusty-marcher)

[not being worked on anymore]
Toy ray marcher inspired by [tinyraytracer](https://github.com/ssloy/tinyraytracer/wiki), with a couple of changes or additions: written in Rust (first project for me, so probably very naive), and taking some multispectral components into account (well, R/G/B indices for everything from reflexion/refraction/diffusion/lighting).

Only dependencies are the awesome Rayon and tiny obj loader, on purpose (programming exercise, again..), plus the GTK toolkit if you want to build the naive UI.

![Current state of affairs](https://github.com/blefaudeux/rusty-marcher/blob/master/test_data/screen.png?raw=true)

How to run: `cargo run` to get the demo scene rendered.
