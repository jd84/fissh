# russh

[![Build Status](https://www.travis-ci.org/jd84/russh.svg?branch=master)](https://www.travis-ci.org/jd84/russh)

`russh` is a SSH wrapper to manage and group server connections aligned with credentials.

> For security reason `russh` doesn't store any passwords.

## Installation

### Pre-build Binaries

Pre-build binaries for Linux, Windows and MacOS can be found on the release page.

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

Transfer a file to a server run `russh -t /path/to/file.txt vm-01:/path/to/dest.txt` 
Or download a file from a server `russh -t vm-01:/path/to/file.txt .`

The Parameter `-t` tells `russh` that you would transfer a file. In the feature maybe this parameter will be removed, so `russh` can auto detect between ssh and scp connections, but for now the `-t` is required.

To edit you config you can call `russh -e` and your favorite editor will start. The parameter `-e` consumes the value configured under `editor`.

## Thanks

Thanks to https://github.com/japaric/trust for CI templates!
