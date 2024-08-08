Okay, how do I want this to work? What do I want to include?

Everything should be cross platform, but initially everything will be built on linux, so linux support will be the primary focus.

As for a feature set, I think something like this would be appropriate:
- [~] Hardware information
    - [~] Monitors
        - [x] Resolution
        - [~] Model
    - [ ] Bluetooth devices
    - [ ] Printers
    - [ ] Mice
    - [ ] Keyboards
    - [ ] Audio devices
        - [ ] MIDI
        - [ ] Speakers
        - [ ] Headphones
    - [ ] Physical storage
        - [ ] Total space
        - [ ] Used space
        - [ ] List drives
    - [ ] RAM
        - [ ] Available
        - [ ] Used
    - [ ] CPU
        - [ ] Model
        - [ ] Capacity
    - [ ] GPU
        - [ ] Model
        - [ ] Capacity
    - [ ] Uptime
- [~] Software information
    - [x] Operating system
    - [x] CPU architecture
    - [ ] Terminal detection
    - [ ] Shell detection
    - [ ] Desktop Environment
    - [ ] Window Manager
- [ ] Process information
- [ ] Network information
    - [ ] LAN IP
    - [ ] Public IP
    - [ ] Other devices on the network?

For a lot of the above features, creating subprocesses probably isn't ideal. There must be a better weay to interact directly with system calls, maybe via a c library on windows or the nix package for linux things.

`neofetch` has a good set of platform specific features.

-----------
The cli should then provide options for providing some or all of the above feature set, and what format to provide that in.
