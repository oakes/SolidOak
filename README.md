## Introduction

SolidOak is a simple IDE for Rust.

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
brew install gtk+3 vte3
brew install libtool automake cmake pkg-config
cargo build
```

### Windows is not supported

This is not a near-term goal. To support Windows, we would need to get both the rgtk and neovim-rs dependencies to build. Additionally, we would need to find a replacement for `fork()`, which is a Posix-specific function being used to run neovim in a separate process.

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
