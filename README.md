# SensiMouse CLI

CLI tool for setting system-wide mouse sensitivity and acceleration on macOS.

## Usage

```bash
sensi-mouse [-s <sensitivity>] [-a <acceleration>]
```

## Options

- `-s <sensitivity>`: Set mouse sensitivity, default is 190, range is 1-199
- `-a <acceleration>`: Set mouse acceleration, default is 0, range is 0-10000000, 0 means disable acceleration
- `-d`: Run as daemon, will check and re-apply mouse settings if system settings are changed or affected by other programs
- `-h`: Show help

## Example

```bash
sensi-mouse                  # set sensitivity to 190 and disable acceleration
sensi-mouse -s 180           # set sensitivity to 180 and disable acceleration
sensi-mouse -s 100 -a 50000  # set sensitivity to 100 and acceleration to 50000
sensi-mouse -s 190 -a 0 -d   # set sensitivity to 190 and disable acceleration, run as daemon, program will not quit
```

## Build from source

Dev dependencies:

- llvm
- clang
- rust

Install dependencies:

```bash
xcode-select --install
```

### Build

```bash
make
```

### Install

```bash
sudo make install
```

### Uninstall

```bash
sudo make uninstall
```

### Run at startup

MouseTune can be configured to run at startup using launchd. The following commands can be used to add or remove MouseTune from launchd:

#### Add to launchd

```bash
make launchd-add
```

#### Remove from launchd

```bash
make launchd-remove
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
