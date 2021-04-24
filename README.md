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


### `version-make show`

Print the current version of the configuration file as standard output.
```
$ version-make show samples/pacakge.json
0.1.1-hot+20210425
```

Output only X.Y.Z part of the current version to the standard output

```
$ version-make show --core samples/pacakge.json
0.1.1-hot+20210425
```


### `version-make up`

Standard output of the configuration file for major version increments. X+1.y.z
```
$ version-make up -x samples/package.json
{
  "name": "node-project",
  "version": "0.1.1-hot+20210425",
  "private": true
}
```

Increment Patch version: x.y.Z+1 and replace new file.
```
$ version-make up -zr samples/package.json
```

Set pre and build: x.y.z -> x.y.z-alpha+beta.
```
$ version-make up --pre preview --build rc2 samples/package.json
{
  "name": "node-project",
  "version": "0.1.2-preview+rc2",
  "private": true
}
```