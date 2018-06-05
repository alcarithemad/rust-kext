# Rust + Kext + MacOS

## Build

You'll need:
* XCode 9.x, in the default location.
* Rust nightly circa 2018-05-31.
* `cargo-xbuild`

1. `cargo xbuild --target x86_64-apple-darwin-kext.json`
1. TODO: make a kext bundle
1. `kextlibs` the dependencies and update Info.plist
1. sign it?
1. ???
1. profit!

