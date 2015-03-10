![screenshot](screenshot.png)

## Introduction

SolidOak is a simple IDE for Rust. It's just a standalone executable that will run anywhere without being installed. See the [Releases](https://github.com/oakes/SolidOak/releases) page for pre-built Linux x64 binaries (you need [Rust](http://www.rust-lang.org/install.html) and GTK+ 3.10 or above installed). It has the following features:

* An embedded copy of [Neovim](https://github.com/neovim/neovim) as its text editor
    - On first launch, it will create ~/.soak and ~/.soakrc (equivalent to ~/.vim and ~/.vimrc)
    - It starts off in "Easy Mode" (locked in insert mode) for Vim newbies, but you can toggle it off
* An easy-to-use GUI written with [rgtk](https://github.com/jeremyletang/rgtk)
    - Buttons for common editing actions and a project tree that stays in sync with Neovim
    - You can bypass the GUI and run it as a console app by passing the `-nw` flag
* An embedded copy of [Racer](https://github.com/phildawes/racer) for autocomplete
    - To enable, you must clone the Rust source code and point Racer to it:
    - `git clone https://github.com/rust-lang/rust.git ~/rust --depth 1 && export RUST_SRC_PATH=~/rust/src`
    - Now try typing `std::io::` and then type Ctrl+x Ctrl+o to see a list of completions

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

To support Windows, we would need to get rgtk and neovim-rs to build for it. Additionally, we would need to find a replacement for all the Posix-specific functions being used in `src/ffi.rs`.

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
