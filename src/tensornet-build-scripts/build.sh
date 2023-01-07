#!/bin/bash

# Compile the kernel
rustc --crate-type=staticlib src/tensornet-core/kernel.rs -o bin/kernel.o

# Compile the device drivers
rustc --crate-type=staticlib src/tensornet-core/device-drivers/keyboard.rs -o bin/keyboard.o
rustc --crate-type=staticlib src/tensornet-core/device-drivers/display.rs -o bin/display.o
rustc --crate-type=staticlib src/tensornet-core/device-drivers/disk.rs -o bin/disk.o

# Compile the system libraries
rustc --crate-type=staticlib src/tensornet-core/system-libraries/stdio.rs -o bin/stdio.o
rustc --crate-type=staticlib src/tensornet-core/system-libraries/string.rs -o bin/string.o

# Compile the apps
rustc src/app1.rs -o bin/app1
rustc src/app2.rs -o bin/app2
rustc src/app3.rs -o bin/app3

# Link everything together
ld -T linker.ld bin/kernel.o bin/keyboard.o bin/display.o bin/disk.o bin/stdio.o bin/string.o bin/app1 bin/app2 bin/app3 -o tensornet.bin
