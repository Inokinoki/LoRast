# LoRast

A LoRa toolset written in Rust

## TODO

Wait for iced Pick List release...

## LoRa Airtime Calculator Input

### LoRa Modem settings

Spreading factor
12
7 - 12

Bandwidth
125
kHz125 kHz default for LoRaWAN. 250 kHz also supported.

Code rate
1
4 / (CR + 4) = 4/5.
4/5 default for LoRaWAN

### Frame configuration

Payload length
51
bytes

Preamble length
8 symbols
Default for frame = 8, beacon = 10.

Explicit header
Yes
Default on for LoRaWAN

CRC
Yes
Default on for LoRaWAN

Low data rate optimization
Yes
Enabled for bandwidth 125 kHz and Spreading factor >= 11

## LoRa Airtime Calculator Output

Preamble length
401.41 ms

Symbol length
32.77 ms

Symbols in frame
63

Time on air
2,465.79 ms

## LoRa Frame Encoder/Decoder

TODO
