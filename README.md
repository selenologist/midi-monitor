# midi-monitor

A very basic midi monitor written in Rust using `midir` and `ghakuf`. Attach an output to it with `aconnect` etc and it will display received MIDI messages.

# Portability

Does not work on Windows as it creates a Virtual Input Port and waits for other programs to connect to it rather than connecting directly to an output. As Windows doesn't support this, this program cannot work. It will fail to compile.
