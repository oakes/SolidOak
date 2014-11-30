## Introduction

SolidOak is a simple IDE for Rust.

## Build Instructions

### Linux (apt-get)

```Shell
apt-get install libgtk-3-dev libglib2.0-dev libcairo2-dev
apt-get install libtool autoconf automake cmake libncurses5-dev g++ pkg-config unzip
cargo build
```

### Linix (yum)


```Shell
yum install gtk3-devel glib2-devel
yum install autoconf automake cmake gcc gcc-c++ libtool ncurses-devel pkgconfig
cargo build
```

### OSX (homebrew)

```Shell
brew install gtk+3
brew install libtool automake cmake pkg-config
cargo build
```

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
