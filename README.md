# Orion-Gameboy
Emulator for the original GameBoy written in Rust. There are many other emulators like this one, some even written in Rust, but the primary goal for this project is to further develop my Rust skills, while working on a project that is interesting to me related to retro-computing.

My goal is to create as accurate of an emulator as possible, while still maintaining strong performance. The majority of the edge-cases, quirks and bugs inherent to the original hardware are well-researched and documented at this point, and I wish to replicate them as much as possible.

## Roadmap

- **CPU:** Currently all CPU opcodes except STOP and HALT are implemented and cycle-accurate
- **GPU:** First steps and the skeleton of the implementation
- **Display:** Once GPU behavior is implemented, an actual graphical display for the Gameboy's screen can be implemented
- **Sound Card:** Not yet implemented
- **Input:** Not yet implemented

## References
The following are references or resources I found helpful while working on this project.

- [Chart of GameBoy Opcodes](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [GameBoy Opcode Summary](http://www.devrs.com/gb/files/opcodes.html)
- [GameBoy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
- [GameBoy Programming Manual](https://ia803208.us.archive.org/9/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
- [mooneye-gb, Reference Emulator](https://github.com/Gekkio/mooneye-gb)
- [Pandocs](https://gbdev.io/pandocs/)
