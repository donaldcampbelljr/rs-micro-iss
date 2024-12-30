# rsMicroISS

### Use an ESP32 Microcontroller to Track the ISS!

### Steps

1. Change WIFI SSID and PW Before Building and Flashing
2. Move to root directory and run `cargo build`
3. Then run `espflash flash target/xtensa-esp32-espidf/debug/rs-micro-iss`

### Requirements
I used an ESP32 development board: [ESP32-WROVER-E](https://www.espressif.com/sites/default/files/documentation/esp32-wrover-e_esp32-wrover-ie_datasheet_en.pdf)
![image](https://github.com/user-attachments/assets/b1c6f81e-b4f5-4edb-a4bb-ac236618c8ff)

## Future TODO
Add interfacing with small screen to output coordinates on map similar to TUI-based: https://github.com/donaldcampbelljr/rsISS
