# Control GPIO output on Raspberry Pi through a REST service 
![build status][build]

[build]: https://gitlab.com/bogeholm/raspberry-web/badges/master/build.svg "Build status (master)"

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
Prerequisites on Raspbian (apart from [Rust](https://www.rust-lang.org/tools/install) :smiley:):
```bash
$ sudo apt-get update && sudo apt-get install build-essential gcc-arm-linux-gnueabihf libsqlite3-dev
& cargo install cargo-deb
```

Installation:
```bash
$ git clone https://gitlab.com/bogeholm/raspberry-web && cd raspberry web
$ cargo deb --install
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


Now you can be run the server from the command line:
```bash
rasbberry-web
```
You can specify an alternate config file
```bash
rasbberry-web --config-file=/path/to/my/awesome/config.toml
```
Or your can use [systemd](https://wiki.debian.org/systemd) (starting this way will read `/usr/local/rasbberry-web/configuration.toml`)
```bash
sudo service start raspberry-web.service
```
