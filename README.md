# fargo

    fargo v0.1.0
    Fargo is a prototype Fuchsia-specific wrapper around Cargo

    USAGE:
        fargo [FLAGS] [OPTIONS] [SUBCOMMAND]

    FLAGS:
            --debug-os    Use debug user.bootfs and ssh keys
        -h, --help        Prints help information
        -V, --version     Prints version information
        -v, --verbose     Print verbose output while performing commands

    OPTIONS:
        -N, --device-name <device-name>    Name of device to target, needed if there are multiple devices visible on the network

    SUBCOMMANDS:
        autotest             Auto build and test in Fuchsia device or emulator
        build                Build binary targeting Fuchsia device or emulator
        build-tests          Build tests for Fuchsia device or emulator
        cargo                Run a cargo command for Fuchsia. Use -- to indicate that all following arguments should be passed to
                             cargo.
        configure            Run a configure script for the cross compilation environment
        create-facade        Create an in-tree facade crate for a FIDL interface.
        enable-networking    Enable networking for a running emulator
        help                 Prints this message or the help of the given subcommand(s)
        list-devices         List visible Fuchsia devices
        load-driver          Build driver and load it on Fuchsia device or emulator.
        pkg-config           Run pkg-config for the cross compilation environment
        restart              Stop all Fuchsia emulators and start a new one
        run                  Run binary on Fuchsia device or emulator
        ssh                  Open a shell on Fuchsia device or emulator
        start                Start a Fuchsia emulator
        stop                 Stop all Fuchsia emulators
        test                 Run unit tests on Fuchsia device or emulator

The `fargo-test` directory contains something one can use to test-drive.

## Getting started

Since at the moment fargo requires the FUCHSIA\_ROOT environmental variable be
set to the path to a Fuchsia **release** build, the first step is to build
Fuchsia.

The [Fuchsia Getting
Started](https://fuchsia.googlesource.com/docs/+/HEAD/getting_started.md)
instruction are what you need. Since a release build is what fargo expects to
find you'll want to pass --release to fx/set. The Rust components that fargo
needs to cross compile are part of garnet, so you must be using the
garnet layer or higher.

The author most often uses the following steps to update and build Fuchsia in
preparation for using fargo

    ./scripts/fx set-layer garnet
    .jiri_root/bin/jiri update
    ./scripts/fx set x86 --release
    ./scripts/fx build-zircon
    ./scripts/fx build

Once this build is complete, clone and build fargo.

    git clone https://fuchsia.googlesource.com/fargo
    cd fargo
    cargo install --force

Fargo uses ssh to communicate between your host computer and either Qemu or a
real device to copy build results and execute them. For Qemu there is a bit of
[tricky set up](https://fuchsia.googlesource.com/magenta/+/master/docs/qemu.md#Enabling-Networking-under-QEMU-x86_64-only) to do.

Finally, you need to be using nightly (as opposed to stable) and have the `x86_64-unknown-fuchsia`
target installed. If you installed rust with [rustup](https://www.rustup.rs) you can
install the target with:

    rustup default nightly
    rustup target add x86_64-unknown-fuchsia

If you installed Rust some other way, you'll have to do some research about how to get the nightly
build and `x86_64-unknown-fuchsia` support into your installation.

### Testing if Fargo is working

Now to verify if fargo is working correctly, try starting a fuchsia machine and executing a test.

    fargo start
    cd fargo/fargo-test
    fargo test

Note that fargo start now depends on an environment using fx set. If that isn't the way you start
Fuchsia emulators, use fargo enable-networking after you've started the emulator.

If all is well, you should see a successful test pass just as if you had ran cargo test on any other
rust project.

Additionally, if you are using qemu you need to enable networking, otherwise fargo won't be able to
copy the binary onto then fuchsia machine to run the tests.

## Getting help

For problems getting the Fuchsia build to complete, the #fuchsia IRC channel on
freenode is the best bet.

For fargo itself, that IRC channel can also work of one of the more Rust-aware
folks happens to be paying attention. More reliable is the
[rust-fuchsia](https://groups.google.com/a/fuchsia.com/forum/#!aboutgroup/rust-fuchsia) Google group.

## Using crates that link with native libraries

Some crates are wrappers around libraries written in other languages. An
example of one such crate is [cairo-rs](https://crates.io/crates/cairo-rs).
Cargo has to know what libraries need to be linked to a binary using such a
crate and where to find those libraries.

Cargo uses build.rs files to locate such libraries. This provides a challenge
for Fargo, as it is unlikely that such build.rs files would know how to cross
compile their libraries for Fuchsia.

Luckily, many of the crates of interest which have native dependencies use
[pkg-config](https://docs.rs/pkg-config/0.3.9/pkg_config/) as one of the ways
to find native dependencies. Fargo provides functions to set up and use a
Fuchsia-specific pkg-config directory.

`fargo pkg-config` is a wrapper around pkg-config that sets the environment so
that only packages found in the Fuchsia-specific pkg-config directory are
visible. This is useful to test if a particular package is already installed.

`fargo configure` is a wrapper around a package's automake configure script.
It takes care of setting up environmental variables such that many automake
based packages will properly cross-compile.

See `scripts/build_cairo_support.sh` for an example of how to use these
functions to build native support.

## Fargo roadmap

The goal is to transition fargo to using something like an SDK instead.
