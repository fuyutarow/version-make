# version-make

Manager for [SemVer](https://semver.org/)

Support
- Cargo.toml ([Cargo](https://github.com/rust-lang/cargo))
- package.json ([npm](https://github.com/npm/cli), [yarn](https://github.com/yarnpkg/yarn))
- pyproject.toml ([Poetry](https://github.com/python-poetry/poetry))


## Installation
```
brew install --HEAD fuyutarow/tap/version-make
```

clean uninstall
```
brew uninstall fuyutarow/tap/version-make
brew untap fuyutarow/tap
```


## Usage
semver: X.Y.Z (Major.Minor.Patch)

- Increment Major version: X+1.y.z
  ```
  version-make up -x Cargo.toml
  ```
- Increment Patch version: x.y.Z+1 and replace new file
  ```
  version-make up -z -r Cargo.toml
  ```
- Set pre and build: x.y.z -> x.y.z-alpha+beta
  ```
  version-make up --pre alpha --build beta Cargo.toml
  ```


## Development
```
cargo install cargo-make
cargo make hot
```
