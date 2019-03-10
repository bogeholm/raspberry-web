# raspberry-web 
[![Build status](https://gitlab.com/bogeholm/raspberry-web/badges/master/build.svg)](https://gitlab.com/bogeholm/raspberry-web/) 
[![Build status](https://api.travis-ci.com/bogeholm/raspberry-web.svg?branch=master
)](https://travis-ci.com/bogeholm/raspberry-web) 
[![Codecov](https://codecov.io/gl/bogeholm/raspberry-web/branch/master/graph/badge.svg)](https://codecov.io/gl/bogeholm/raspberry-web) 
[![crates.io](https://img.shields.io/crates/v/raspberry-web.svg)](https://crates.io/crates/raspberry-web) 
[![docs.rs](https://docs.rs/raspberry-web/badge.svg)](https://docs.rs/crate/raspberry-web/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


## Control GPIO output on Raspberry Pi through a REST service 
Run the server and send a GET request to http://localhost:2323/set/level/2/high:
```json
{
    "gpio_id": 2,
    "in_use": 1,
    "gpio_mode": "output",
    "gpio_level": "high",
    "last_change": "2019-02-18 21:19:31.239669"
}
```

## Installation
Prerequisites on Raspbian (apart from [Rust](https://www.rust-lang.org/tools/install)):
```bash
$ sudo apt-get update && sudo apt-get install build-essential libsqlite3-dev
$ cargo install cargo-deb
```

### Install using [`cargo deb`](https://github.com/mmstick/cargo-deb)
```bash
$ git clone https://gitlab.com/bogeholm/raspberry-web && cd raspberry_web
$ cargo build --release
$ cargo deb
$ cargo deb --install
```
Your user will need write permission to `/usr/local/raspberry-web` unless you only want to run the program as root:
```
sudo chown -R $(whoami) /usr/local/raspberry-web
```

## Usage
Installation using `cargo deb` places a binary in `/usr/local/bin` and a configuration file at `/usr/local/raspberry-web/configuration.toml`, which should be edited before use.

Say you want to be able to swicth GPIO pins 1,2 and 3 on and off, start with GPIO pins 1 and 2 off (level low) and pin 3 on (level high), you would put the following under the section `[gpioconfig]`
```
[gpioconfig]
gpios_in_use = [1, 2, 3]
gpios_mode_output = [1, 2, 3]
gpios_level_low = [1, 2]
gpios_level_low = [3]
```

Now you can run the server from the command line:
```bash
rasbberry-web
```

You can specify an alternate config file
```bash
rasbberry-web --config-file=/path/to/my/awesome/config.toml
```

You could also use [systemd](https://wiki.debian.org/systemd) (starting this way will read `/usr/local/rasbberry-web/configuration.toml`):
```bash
sudo systemctl start raspberry-web.service
```


## Repositories
Currently the [GitLab](https://gitlab.com/bogeholm/raspberry-web) repo is mirrored on [GitHub](https://github.com/bogeholm/raspberry-web)