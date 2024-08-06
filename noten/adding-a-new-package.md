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
```

scripts/build.bat
```sh
cargo build --release --package package_name
```

scripts/setup.ts
```ts
const packages = [
    "...",
    "package_name",
];
```

Once you've done all of those things, you want to do:
```sh
./scripts/build.sh
./scripts/setup.ts
```
