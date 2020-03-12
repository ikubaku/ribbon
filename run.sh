#!/bin/bash

set -m
openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg &> /dev/null &
OPENOCD_PID=$!
set +m
gdb -x startup.gdb "$@"
kill $OPENOCD_PID
