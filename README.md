![screenshot](screenshot.png)

## Introduction

SolidOak is a simple IDE for Rust. See [the website](https://sekao.net/solidoak/) for binary releases. It has the following features:

* An embedded copy of [Neovim](https://github.com/neovim/neovim) as its text editor
    - On first launch, it will create ~/.soak and ~/.soakrc (equivalent to ~/.vim and ~/.vimrc)
    - It starts off in "Easy Mode" (locked in insert mode) for Vim newbies, but you can toggle it off
* An easy-to-use GUI written with [gtk-rs](https://github.com/gtk-rs/gtk)
    - Buttons for common editing actions and a project tree that stays in sync with Neovim
    - You can bypass the GUI and run it as a console app by passing the `-nw` flag

## Build Instructions

Note: If neovim fails to build, try [cloning it directly](https://github.com/oakes/neovim) and running `make libnvim` to get more specific errors.

### Linux (apt-get)

```Shell
apt-get install libgtk-3-dev libglib2.0-dev libcairo2-dev libvte-2.91-dev
apt-get install libtool-bin autoconf automake cmake libncurses5-dev g++ pkg-config unzip
cargo build --release
```

### Linux (yum)


```Shell
yum install gtk3-devel glib2-devel vte291-devel
yum install autoconf automake cmake gcc gcc-c++ libtool ncurses-devel pkgconfig
cargo build --release
```

### OS X (homebrew)

```Shell
brew install gtk+3 vte3
brew install libtool automake cmake pkg-config gettext
cargo build --release
```

### OS X (macports)

```Shell
port install gtk3 vte
port install libtool automake cmake pkgconfig gettext
cargo build --release
```

### Windows

The following instructions are a work in progress. Building does not currently work because msys2 does not contain a package for vte.

Install [MSYS2](http://www.msys2.org/) and run this in its shell:

```Shell
pacman -S mingw-w64-x86_64-gtk3
```

In cmd.exe, install Rust's GNU toolchain and build:

```Shell
rustup install stable-gnu
set RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
cargo build --release
```

## Licensing

All files that originate from this project are dedicated to the public domain. I would love pull requests, and will assume that they are also dedicated to the public domain.
