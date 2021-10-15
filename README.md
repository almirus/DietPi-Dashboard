# DietPi-Dashboard
A web dashboard for DietPi

[![CodeFactor](https://www.codefactor.io/repository/github/ravenclaw900/dietpi-dashboard/badge/main)](https://www.codefactor.io/repository/github/ravenclaw900/dietpi-dashboard/overview/main)

## Installation
To install, use one of the [precompiled releases](#downloading) or [compile it yourself](#compiling)

### Downloading
```sh
# Get architecture
dpdashboardarch=
if (( $G_HW_ARCH == 10 )); then
    dpdashboardarch='amd64'
elif (( $G_HW_ARCH == 3 )); then
    dpdashboardarch='armv8'
elif (( $G_HW_ARCH == 2 )); then
    dpdashboardarch='armv7'
else
    dpdashboardarch='armv6'
fi
curl -L $(curl -sS 'https://api.github.com/repos/ravenclaw900/dietpi-dashboard/releases/latest' | mawk -F\" "/\"browser_download_url\": \".*dietpi-dashboard-$dpdashboardarch\"/{print \$4}")  -o dietpi-dashboard # Download latest binary for current architecture
unset dpdashboardarch # Remove architecture variable
chmod +x dietpi-dashboard # Make binary exectuable
./dietpi-dashboard # Run binary

```

### Compiling
#### Prereq:

```sh
dietpi-software install 9 16 17 # Install Node.js (webpage), Build-Essential (make and gcc), and Git (git clone), respectively
npm install -g yarn # Install Yarn package manager, for node dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Install Rust (backend)
source ~/.cargo/env # Update PATH
```

#### Compiling:

```sh
rm -rf DietPi-Dashboard # Remove possibly pre-downloaded repository
git clone https://github.com/ravenclaw900/DietPi-Dashboard # Download source code
cd DietPi-Dashboard/src/frontend # Change directories
yarn # Install dependencies
cd ../.. # Change directories
make # Compile binary for your platform
./dietpi-dashboard # Run binary
```

### Compiling for all targets (release)
#### Prereq:

Normal compilation prereq (see above)
```sh
rustup target add aarch64-unknown-linux-gnu arm-unknown-linux-gnueabihf armv7-unknown-linux-gnueabihf x86_64-unknown-linux-gnu # Add Rust standard libraries
apt install gcc-aarch64-linux-gnu libc-dev-arm64-cross gcc-arm-linux-gnueabihf libc6-dev-armhf-cross gcc-x86-64-linux-gnu libc6-dev-amd64-cross # Install cross-compiling toolchains
mkdir /opt/rpi ; git clone https://github.com/raspberrypi/tools /opt/rpi # Install Raspberry Pi cross-compiling toolchain
apt install upx # Install UPX (for compressing binaries)
```
You also need to  [compile](https://github.com/upx/upx/blob/devel/README.SRC) UPX, to compress the ARMv6/7 binaries.

#### Compiling:

```sh
rm -rf DietPi-Dashboard # Remove possibly pre-downloaded repository
git clone https://github.com/ravenclaw900/DietPi-Dashboard # Download source code
cd DietPi-Dashboard/src/frontend # Change directories
yarn # Install dependencies
cd ../.. # Change directories
make build # Compile binaries for all platforms
```
Binaries will then be avalible in the `build` directory.
