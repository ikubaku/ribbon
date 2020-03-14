#!/bin/bash

set -m
openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg &> "openocd_$(date +%y%m%d%H%M_%s).log" &
OPENOCD_PID=$!
set +m
gdb -x startup.gdb "$@"
kill $OPENOCD_PID
