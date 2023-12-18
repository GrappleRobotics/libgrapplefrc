# libgrapplefrc

libgrapplefrc is the RoboRIO library for Grapple devices.

## Installing
If you wish to use libgrapplefrc, use "Manage Vendor Libraries" in VSCode to install from URL https://storage.googleapis.com/grapple-frc-maven/libgrapplefrc2024.json.

## Layout
This project is modelled off WPILib's vendor template, with some key differences. The project is split into three components:
- `grapplefrcdriver`: The Rust low-level library that contains all the primary communications code. This also includes JNI and C bindings for the companion libraries below.
- `libgrapplefrccpp`: The FRC C++ bindings for Grapple devices, interfacing with `grapplefrcdriver`.
- `libgrapplefrcjava`: The FRC Java bindings for Grapple devices, interfacing with `grapplefrcdriver`.

## Building
After installing the appropriate toolchains (`rustup target add arm-unknown-linux-gnueabi`, `./gradlew installRoboRioToolchain`), follow the below steps:
- `./gradlew updateRustLibs -PreleaseMode` - Dumps WPI libs so the Rust library can access them. This only has to be done once per version, just make sure `grapplefrcdriver/buildlibs` is empty first.
- `cd grapplefrcdriver && python build.py linuxathena` - Build the Rust library for the `linuxathena` target. Replace `linuxathena` with another valid platform if required.
- `./gradlew build -PreleaseMode` - Build the vendor library.

For publishing, this is achieved in the GitHub workflows / actions file.
