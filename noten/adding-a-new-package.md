There are a few steps.

```sh
mkdir pkgs/package_name
cd pkgs/package_name
cargo init
```

Cargo.toml
```toml
[dependencies]
package_name.workspace = true

[workspace]
members = [
    "...",
    "pkgs/package_name",
]

[workspace.dependencies]
package_name = { version = "0.1.0", path = "pkgs/package_name" }
```

pkgs/package_name/Cargo.toml
```toml
[dependencies]
clap.workspace = true
```

scripts/build.sh
```sh
cargo build --release --package package_name

cp target/release/package_name ./bin/

echo "Generating completion file for package_name";
./bin/package_name --generate-completion zsh > completion/package_name
```

scripts/build.bat
```sh
cargo build --release --package package_name

cp target/release/package_name bin
```

scripts/setup.ts
```ts
const packages = [
    "...",
    "package_name",
];
```

scripts/clippy.sh
```sh
cargo clippy --package package_name
```

Once you've done all of those things, you want to do:
```sh
just build
just setup
```
