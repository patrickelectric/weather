$ cargo install -f ldproxy espflash espmonitor espup
$ espup install -t esp32s2
$ cargo install cargo-generate
$ don't forget to source the file export-esp.sh that prompts in the end
$ cargo build
$ espflash /dev/ttyUSB0 target/xtensa-esp32s2-none-elf/debug/weather_esp32_s2
