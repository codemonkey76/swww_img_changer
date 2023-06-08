# Wallpaper changer for SWWW (Solution to your Wayland Wallpaper Woes)

## Dependencies
  - a compositor that implements:
    * wlr-layer-shell (typically wlroots based compositors)
    * xdg-output
  - [lz4](https://github.com/lz4/lz4) (for compressing frames when animating)
  - [SWWW](https://github.com/Horus645/swww)


### Dependencies:

  - Up to date stable rustc compiler and cargo

To build, clone this repository and run:
```
cargo build --release
```
Then, put the binary 'target/release/swww_img_changer' in your path.

You can then add it as a cronjob to run when you like.
