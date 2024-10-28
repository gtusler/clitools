The idea with this is that we can create multiple cli tools under one roof, with a shared lib.
I have a similar thing going on with `bun`, in `~/dev/ts/tbem`.

I am using bun in this project, but only for the setup script.

## Installation
First, run:
```sh
git clone git@github.com:gtusler/clitools.git
cd clitools
./scripts/build.sh
./bin/install
```

Then, you want to configure `importTo` in `./scripts/setup.ts`, using a path that makes sense for your system. The path supplied should point to somewhere on `$PATH`, because the intention is for these tools to be easily accessible via the command line.
