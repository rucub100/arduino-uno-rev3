# arduino-uno-rev3
Learn MCU programming from scratch with Arduino Uno Rev3 and Rust. This standalone educational project doesn’t rely on existing crates or dependencies.

## Resources

- [Arduino UNO Board Docs](https://docs.arduino.cc/hardware/uno-rev3/)
- [ATmega328p Datasheet](https://ww1.microchip.com/downloads/aemDocuments/documents/MCU08/ProductDocuments/DataSheets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)
- [megaAVR Family Datasheet](https://ww1.microchip.com/downloads/aemDocuments/documents/MCU08/ProductDocuments/DataSheets/ATmega48A-PA-88A-PA-168A-PA-328-P-DS-DS40002061B.pdf)

## MCU Setup and Fuse Configuration

This section provides detailed information on setting up the Microcontroller Unit (MCU) for this board, including specific fuse configurations.

### MCU Setup

- **Microcontroller**: `ATmega328P`
- **Clock Source**: External 16MHz crystal
- **Bootloader**: Arduino IDE provides a convenient way to burn the bootloader onto your microcontroller

### Fuse Configuration

Fuses are used to configure various hardware settings of the MCU. Below are the recommended fuse settings for this board:

- **Low Fuse**: `0xFF`
- **High Fuse**: `0xDE`
- **Extended Fuse**: `0xFD`

which corresponds to

- External Crystal Oscillator
- Boot Reset Vector Enabled
- Boot Flash Section Size: `256` words (`512` bytes)
- Boot Start Address: `0x3F00`
- Serial Program Downloading (SPI) Enabled
- Brown-Out Detection (BOD) Level: 1 (`VCC=2.7V`)

A new `ATmega328P` typically does not come with a bootloader, and its default settings may differ. Therefore, a newly purchased chip needs to be set up for the board.

### MISC

For more information, refer to the `boards.txt` file from the Arduino IDE. Here is a summary:

```conf
...
uno.bootloader.tool.default=avrdude
uno.bootloader.low_fuses=0xFF
uno.bootloader.high_fuses=0xDE
uno.bootloader.extended_fuses=0xFD
uno.bootloader.unlock_bits=0x3F
uno.bootloader.lock_bits=0x0F
uno.bootloader.file=optiboot/optiboot_atmega328.hex
...
```

Note: If you need to read or write the fuse or lock bit values, you would typically need to use an external hardware programmer (like a USBasp, or another Arduino configured as an “Arduino as ISP”). This is because these operations require a level of access to the microcontroller that the bootloader does not provide.

#### Reading Lock Bits

```sh
avrdude -c arduino -p m328p -P /dev/ttyACM0 -b 115200 -U lock:r:-:h
```

#### Reading Fuse Bits
```sh
avrdude -c arduino -p m328p -P /dev/ttyACM0 -b 115200 -U lfuse:r:-:h -U hfuse:r:-:h -U efuse:r:-:h

```

## Troubleshooting

### Error: no matching serial port found, use -P or set RAVEDUDE_PORT in your environment

Hint: You may want to flash the microcontroller from your host machine (outside devcontainers) to avoid this issue.

Go to the [.cargo/config.toml](.cargo/config.toml) file and add the `-P /dev/...` flag with the serial port of your Arduino Uno device for the `ravedude` command.
On Linux, it's usually named `ttyACM*` or `ttyUSB*`.
Run the following command to show new TTY devices:
```sh
dmesg | tail | grep -i tty
```
