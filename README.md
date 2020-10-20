# russh

[![Build Status](https://www.travis-ci.org/jd84/russh.svg?branch=master)](https://www.travis-ci.org/jd84/russh)

`russh` is a SSH and SCP wrapper to manage and group server connections aligned with credentials.

> For security reason `russh` doesn't store any passwords.

## Installation

### One-Line Installation

**Linux**

```bash
curl -LSfs https://japaric.github.io/trust/install.sh | \
  sudo sh -s -- --git jd84/russh --target x86_64-unknown-linux-gnu --to /usr/local/bin/
```

**macOS**

```bash
curl -LSfs https://japaric.github.io/trust/install.sh | \
  sh -s -- --git jd84/russh --target x86_64-apple-darwin --to ~/bin/
```

### Pre-build Binaries

Pre-build binaries for Linux and MacOS can be found on the release page.

## Quick Start

Once `russh` is installed on your computer. Try running `russh --version` to make sure that is installed correctly, create a file named `russh.yml` in `~/.ssh/`.

Example **russh.toml**

```toml
[identities]
  [identities.root]
    user = "root"
    key = "~/.ssh/id_rsa"

  [identities.pi]
    user = "pi"

[groups]
  [groups.webserver]
    [[groups.webserver.servers]]
      name = "web-01"
      hostname = "192.168.100.5"
      user = "root"
      port = 22
      description = "Wordpress Webserver"

    [[groups.database.servers]]
      name = "db-01"
      hostname = "db-01.localdomain.local"
      user = "pi"
      port = 22
      description = "RaspberryPi Database"
```

To list and show your configured servers, just run `russh -l`

To open a ssh connection to you server run `russh web-01`.

To transfer files from or to a server `russh` supports scp.

Transfer a file to a server run `russh /path/to/file.txt web-01:/path/to/dest.txt`
Or download a file from a server `russh web-01:/path/to/file.txt .`

## Thanks

Thanks to https://github.com/japaric/trust for CI templates!
