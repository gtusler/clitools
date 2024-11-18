# Installer

The aim with this package is to install all the cli tools contained within the repo, globally, somewhere on $PATH, for convenience sake.

There are some interesting things here, which I haven't covered before.

## The plan

For the sake of future-proofing, this package assumes that the binaries are already built and shell completion scripts are already built.

Make a global dir to move the binaries to, or choose an appropriate place based on the OS.
Detect the current shell and add the binaries to `$PATH`, then load the shell completion scripts.

Run initial setup for each package that requires it, for example the `timers` package requires a global database where it stores timer info.
