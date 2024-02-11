6502 address space is 16 bits. Address range:

0000
...
FFFF

Need:

- RAM
- ROM
- IO

0000 to 3FFF: RAM
4000 to 7FFF: IO
8000 to FFFF: ROM

How does IO work?

- Read any address: read a character from the keyboard
- Write any address:
  - If the high bit is SET: Clear the screen
  - If the high bit is CLEAR and the byte is 0: Clear the keyboard buffer
  - Otherwise, output the byte as a character to the screen
