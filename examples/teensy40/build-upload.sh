#!/bin/bash

set -ex

cargo objcopy --release -- -O ihex lcd.hex
teensy_loader_cli -vsw --mcu TEENSY40 lcd.hex

