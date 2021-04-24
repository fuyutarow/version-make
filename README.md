# version-make

A CLI tool for versioning according to [SemVer](https://semver.org/)

Support config file
- Cargo.toml ([Cargo](https://github.com/rust-lang/cargo))
- package.json ([npm](https://github.com/npm/cli), [yarn](https://github.com/yarnpkg/yarn))
- pyproject.toml ([Poetry](https://github.com/python-poetry/poetry))
- [manifest.json](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json)


## Installation
Support OS
- Homebrew (mac)
- Linuxbrew (Linux, WSL)

Install
```
brew install --HEAD fuyutarow/tap/version-make
```

Clean uninstall
```
brew uninstall fuyutarow/tap/version-make
brew untap fuyutarow/tap
```


## Usage
semver: X.Y.Z-a+b (Major.Minor.Patch-pre+build)

Examples
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
