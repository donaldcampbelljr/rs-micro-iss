# rsMicroISS

### Use an ESP32 Microcontroller to Track the ISS!

### Requirements
I used an ESP32 development board: `ESP32-WROVER-E`

### Steps

1. Change WIFI SSID and PW Before Building and Flashing
2. Move to root directory and run `cargo build`
3. Then run `espflash flash target/xtensa-esp32-espidf/debug/rs-micro-iss`


## Future TODO
Add interfacing with small screen to output coordinates on map similar to TUI-based: https://github.com/donaldcampbelljr/rsISS