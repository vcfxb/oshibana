# Oshibana

**A _Magic: The Gathering_ collection tracker for the modern era.**

## _Author's Note_

This project is actively being rewritten in Rust as a desktop native application, and should be considered a work in progress.

It's core goals are as follows:

- Track which cards are allocated/assigned to which decks you own.
- Rich deck history -- revisions, versions, etc.
- Scryfall liveness independence -- Scryfall is the best source for card data, but that doesn't mean we have to depend
  on their website & search engine being up for ours to work.

Explicit non-goals of the project:

- Per-card price history -- check tcgplayer if you want this, or another tool. It's out of scope.
- Game simulation -- there are other tools you can use to play with the decks you build here.


## System Requirements

In order to make searching for cards all throughout the history of magic as fast as possible,
we store a compressed copy of Scryfall's card exports in memory. Because of this, it is recommended
that you expect Oshibana to use 2-3 Gigabytes of RAM while you're using it. As I spend more time
optimizing card storage going forward, this number may eventually be reduced.

## Installation

We provide a variety of ways to install/run Oshibana on your machine. Please select the one that suits you best.

Once you know what file name you're looking for, head over to the [Release Page](https://github.com/vcfxb/oshibana/releases/latest)
and find it!

#### Windows

| Architecture                                | Installation Wizard (**Recommended for most users**) | Standalone Executable                          | 
|:--------------------------------------------|------------------------------------------------------|------------------------------------------------|
| x86_64 (Intel/AMD 64-bit) (**Most Common**) | `oshibana_VERSION_x64-setup.exe`                     | `oshibana-VERSION-x86-64-pc-windows-msvc.zip`  |
| ARM64 (ARM 64-bit)                          | `oshibana_VERSION_arm64-setup.exe`                   | `oshibana-VERSION-aarch64-pc-windows-msvc.zip` |


#### Mac

| Architecture                                       | `.dmg` file (**Recommended for most users**) | Standalone Executable                          | 
|:---------------------------------------------------|----------------------------------------------|------------------------------------------------|
| x86_64 (Intel/AMD 64-bit)                          | `Oshibana_VERSION_x64.dmg`                   | `oshibana-VERSION-x86-64-apple-darwin.tar.gz`  |
| ARM64 (ARM 64-bit) (**Most Common on newer Macs**) | `Oshibana_VERSION_aarch64.dmg`               | `oshibana-VERSION-aarch64-apple-darwin.tar.gz` |

#### Linux

| Architecture              | `.deb` file                  | AppImage                            | Standalone Executable                               | 
|:--------------------------|------------------------------|-------------------------------------|-----------------------------------------------------|
| x86_64 (Intel/AMD 64-bit) | `oshibana_VERSION_amd64.deb` | `oshibana_VERSION_x86_64.AppImage`  | `oshibana-VERSION-x86-64-unknown-linux-gnu.tar.gz`  |
| ARM64 (ARM 64-bit)        | `oshibana_VERSION_arm64.deb` | `oshibana_VERSION_aarch64.AppImage` | `oshibana-VERSION-aarch64-unknown-linux-gnu.tar.gz` |

## Compiling from source

This repository should have no major dependencies outside the Rust language itself, so running `cargo build` after cloning it
should work. If you're looking for a one-liner, `cargo install --git https://github.com/vcfxb/oshibana.git --locked` should work,
but I haven't tried it. Let me know if you run into any issues.

## Bug Reporting / Feature Requests
Go over to the [Issues Page](https://github.com/vcfxb/oshibana/issues) and make a ticket there, I should see it! Please
try to include as much detail as possible. For bug reports, uploading your log file really helps. It should be found at
`C:\Users\your-username\AppData\Local\Oshibana\cache\logs\oshibana.log` on Windows.
