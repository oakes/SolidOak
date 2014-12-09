## Introduction

SolidOak is a simple IDE for Rust. It uses GTK+ for the UI and Neovim for the editor. The goal is to provide a simple, all-in-one solution to get started with Rust programming.

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

```Shell
brew tap TingPing/gnome
brew install TingPing/gnome/gtk+3 TingPing/gnome/vte3
brew install libtool automake cmake pkg-config
cargo build
```

### Windows is not supported

To support Windows, we need to get rgtk and neovim-rs to build for it. Additionally, we need to find a replacement for `fork()` and other Posix-specific functions being used to run Neovim in a separate process.

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
