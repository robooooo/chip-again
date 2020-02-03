# chip-again

Another CHIP-8 emulator, for the terminal, written with Rust. Note that this project is nowhere near complete yet.

### What is a CHIP-8?

Basically, a VM for simple games originally used on 8-bit computers, which (due to the simple nature of the platform, is a fairly common project). To quote those more in-the-know than me,

> CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. It was made to allow video games to be more easily programmed for these computers.

- https://en.wikipedia.org/wiki/CHIP-8

### Why the name?

Because emulators for this system are an extremely common intermediate project, I figured all the clever names were already took.

### Why should I use this over other emulators?

In short, there's probably not much reason for you to do that, since so many are avaliable, and many are bound to be more fully-featured than this project in particular. But if you have a moral obligation to using software not written in rust, and you want to play CHIP-8 games in your terminal, then there's a chance this might have the features you want. (or at least, maybe in the future...)

### What features are planned for the emulator? 

Ranked in order of priority,

1. Full emulation of the CHIP-8 instruction set.
2. Customisable and selectable display modes.
3. Ability to create snapshots of memory, load saved states.
4. Ability to define custom keymaps.

