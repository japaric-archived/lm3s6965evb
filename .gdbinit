target remote :3333
break UserHardFault
break DefaultHandler
break lm3s6965evb::main
continue
