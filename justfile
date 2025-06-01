default:
    just --list

# Build all the projects
build:
    ./scripts/build.sh

# Run clippy on all the projects
clippy:
    ./scripts/clippy.sh

# Formatting
fmt:
    cargo fmt

# Link binaries
setup:
    ./scripts/setup.ts
