#!/bin/bash

# Check that the script is being run as root
if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

# Create a new directory for the package
mkdir tensornet-package

# Copy the kernel and device drivers to the package directory
cp -r tensornet-core/kernel.rs tensornet-package/
cp -r tensornet-core/device-drivers tensornet-package/

# Copy the system libraries to the package directory
cp -r system-libraries tensornet-package/

# Copy the apps to the package directory
cp -r app1.rs tensornet-package/
cp -r app2.rs tensornet-package/
cp -r app3.rs tensornet-package/

# Copy the user guide and API reference to the package directory
cp user-guide.md tensornet-package/
cp api-reference.md tensornet-package/

# Copy the tutorials to the package directory
cp -r tutorials tensornet-package/

# Copy the examples to the package directory
cp example1.rs tensornet-package/
cp example2.rs tensornet-package/
cp example3.rs tensornet-package/

# Copy the build script to the package directory
cp build.sh tensornet-package/

# Copy the config file to the package directory
cp config.toml tensornet-package/

# Copy the build script to the package directory
cp build.rs tensornet-package/

# Copy the homepage to the package directory
cp index.html tensornet-package/

# Create a tarball of the package directory
tar -czvf tensornet-package.tar.gz tensornet-package/

# Remove the package directory
rm -r tensornet-package/
