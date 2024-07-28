# arduino-uno-rev3
Learn MCU programming from scratch with Arduino Uno Rev3 and Rust. This standalone educational project doesn’t rely on existing crates or dependencies.

## Troubleshooting

### Error: no matching serial port found, use -P or set RAVEDUDE_PORT in your environment

Hint: You may want to flash the microcontroller from your host machine (outside devcontainers) to avoid this issue.

Go to the [.cargo/config.toml](.cargo/config.toml) file and add the `-P /dev/...` flag with the serial port of your Arduino Uno device for the `ravedude` command.
On Linux, it's usually named `ttyACM*` or `ttyUSB*`.
Run the following command to show new TTY devices:
```sh
dmesg | tail | grep -i tty
```