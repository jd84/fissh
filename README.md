# russh

[![Build Status](https://www.travis-ci.org/jd84/russh.svg?branch=master)](https://www.travis-ci.org/jd84/russh)

`russh` is a SSH wrapper to manage and group server connections aligned with credentials.

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

Example **config.yml**

```yaml
version: 1

editor: nano

credentials:
    -   
        User: admin_user
        IdentityFile: ~/.ssh/private_key
    -
        User: pi

groups:
    -
        Name: home-network
        Hosts:
            -
                Name: pollux
                HostName: pollux.home-network.local
                Port: 22
                Users: [admin_user]
            -
                Name: pihole
                HostName: pirategate.home-network.local
                Port: 22
                Users: [pi, admin_user]
    -
        Name: work-network
        Hosts:
            -
                Name: vm-01
                HostName: vm-01.srv.work-network.com
                Port: 22
                Users: [admin_user]
```

To list and show your configured servers, just run `russh -l`

```bash
$ russh -l
home-network

	pollux (pollux.home-network.local)
	pihole (pirategate.home-network.local)

work-network

	vm-01 (vm-01.srv.work-network.com)
```

To open a ssh connection to you server run `russh vm-01`.

To transfer files from or to a server `russh` supports scp.

Transfer a file to a server run `russh /path/to/file.txt vm-01:/path/to/dest.txt` 
Or download a file from a server `russh vm-01:/path/to/file.txt .`

## Thanks

Thanks to https://github.com/japaric/trust for CI templates!
Thanks to https://crates.io/crates/slab
