target remote :3333

monitor arm semihosting enable
monitor reset halt
load
monitor reset halt