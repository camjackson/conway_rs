[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

# Conway's Game of Life in Rust!

This is my first attempt at making something real in Rust, built using various Rust nightlies in the month or so
leading up to the first stable release. 

It has a 128x96 grid, wrapping on both axes, and rendered at 60fps using OpenGL, with [glium](http://github.com/tomaka/glium).

Run it with `cargo run [seed_name]`. Valid seeds are `random` and `gosper_glider`. The default is `random`.

I've tested it on OSX and Windows. It may or may not work on Linux.

From a random seed:

![random image](random.png)

From the glider seed:

![glider image](gliders.png)

The code looks OK to me, but I'm still learning the basics of Rust, so I'd be keen to get any feedback (or even pull
requests!) to help me improve the code :)
