## voxelspace-rs

A Rust implementation of the [VoxelSpace](https://github.com/s-macke/VoxelSpace/tree/master) algorithm. It uses [minifb](https://crates.io/crates/minifb) to open a window that can take keyboard input and display the contents of the framebuffer; [image](https://crates.io/crates/image) to load the terrain map files, and [image-pyramid](https://crates.io/crates/image-pyramid) to create (currently unused) mipmaps of those.

Currently, this is an extremely basic demo. I've not done this kind of thing much before, so please don't expect good organization or optimization yet, sorry. I intend to maybe make a game incorporating this kind of graphics algorithm eventually; I love the aesthetic.

Right now, this demo loads one hardcoded pair of terrain map files (heightmap and colormap) I took from the repo linked above, and it renders that terrain with some hardcoded parameters you can find in `src/config.rs`, with a controllable moving camera (whose motion parameters you can find in `src/camera.rs`).

Be aware that the performance is currently unacceptably bad when compiled in debug mode. It doesn't run as well as I'd like when compiled in release mode, either; it runs great on my M3, but I want it to run well on lower-end CPUs. The logic (i.e. camera movement) and rendering are currently synchronized, btw, which isn't necessarily optimal for developing this into a video game. 

#### TODO:

* Currently, there's only one thread. I intend to learn how to do this kind of rendering on the GPU stuff, but that might take me a little while, so in the meantime, I'll experiment with making the rendering multi-threaded--that seems easy enough, and I'm curious to experience for myself how viable it is here.

* Make use of the mipmaps. I've experimented with taking sparser samples of the terrain farther out from the camera, and *wow*, does it get ugly without mapmapping!

* Desynchronize the logic from the frame rate? I'm undecided on that. There's some joy to be found in old video games when the whole thing would slow down as the boss spammed fancy projectiles or fancily exploded. On the other hand, different CPUs have different thresholds of running full-speed vs slowing down cinematically vs turning into a slideshow.

* Start turning this engine into a game? Would be fun to make the terrain destructible, give the player a ship with stuff to make craters in it.

#### BUGS:
* There's currently an obvious visual bug where the terrain appears degenerate at negative coordinates. This is because the camera coordinates are stored as floats, which are then cast to unsigned ints for calculating indices into the terrain maps, so those negative floats get flattened to zero. I haven't decided what are the most sensible numeric types to use here, but my first attempt at ignoring that question and fixing the bug (by doing float `.rem_euclid` before casting to unsigned int) caused a very large performance hit, so I'm just leaving it for now.