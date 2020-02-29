# UpVER

Manager for [SemVer](https://semver.org/)

Support
- Cargo.toml (Cargo)
- package.json (npm, yarn)


## Installation
```
brew install fuyutarow/upver/upver
```

## Usage
semver: X.Y.Z (Major.Minor.Patch)

- Increment Major version: X+1.y.z
  ```
  upver up -x Cargo.toml
  ```


## Development
```
cargo make hot
```
