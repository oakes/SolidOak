## Introduction

SolidOak is a simple IDE for Rust. It uses GTK+ for the UI and Neovim for the editor. You can think of it as an easy-to-use vim GUI with Rust support baked in. You can also run it with the "-nw" flag, which will bypass the GUI and simply run it as a Rust-flavored vim. On first launch, it will create ~/.soak and ~/.soakrc, which are exactly equivalent to ~/.vim and ~/.vimrc.

## Build Instructions

### Linux (apt-get)

```Shell
apt-get install libgtk-3-dev libglib2.0-dev libcairo2-dev libvte-2.90-dev
apt-get install libtool autoconf automake cmake libncurses5-dev g++ pkg-config unzip
cargo build
```

### Linux (yum)


```Shell
yum install gtk3-devel glib2-devel vte3-devel
yum install autoconf automake cmake gcc gcc-c++ libtool ncurses-devel pkgconfig
cargo build
```

### OS X (homebrew)

Install [XQuartz](http://xquartz.macosforge.org/landing/), then:
```Shell
brew install gtk+3 vte3 --without-x11
brew install libtool automake cmake pkg-config gettext
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig:/opt/X11/lib/pkgconfig
cargo build
```

### Windows is not supported

To support Windows, we would need to get rgtk and neovim-rs to build for it. Additionally, we would need to find a replacement for all the Posix-specific functions being used in `src/native.rs`.

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
