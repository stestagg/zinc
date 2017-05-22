target remote :3333
load firmware.hex
mon reset
display/i $pc
display/x $r0
display/x $r1
display/x $r2
display/x $r3
display/x $r4
display/x $r5
display/x $r6

