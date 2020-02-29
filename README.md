# UpVER

Manager for [SemVer](https://semver.org/)

Support
- Cargo.toml (Cargo)
- package.json (npm, yarn)


## Installation
```
brew install --HEAD fuyutarow/upver/upver
```

clean uninstall
```
brew uninstall fuyutarow/upver/upver
brew untap fuyutarow/upver
```


## Usage
semver: X.Y.Z (Major.Minor.Patch)

- Increment Major version: X+1.y.z
  ```
  upver up -x Cargo.toml
  ```
- Increment Patch version: x.y.Z+1 and replace new file
  ```
  upver up -z -r Cargo.toml
  ```


## Development
```
cargo make hot
```
