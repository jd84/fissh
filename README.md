# fissh

`fissh` is a SSH wrapper to manage and group server connections aligned with credentials.

> For security reason `fissh` doesn't store any passwords.

## Installation

### Pre-build Binaries

Pre-build binaries for Linux, Windows and MacOS can be found on the release page.

## Quick Start

Once `fissh` is installed on your computer. Try running `fissh --version` to make sure that is installed correctly, create a file named `fissh.yml` in `~/.ssh/`.

Example **config.yml**

```yaml
version: 1

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

To list and show your configured servers, just run `fissh -l`

```bash
$ fissh -l
home-network

	pollux (pollux.home-network.local)
	pihole (pirategate.home-network.local)

work-network

	vm-01 (vm-01.srv.work-network.com)
```

To open a ssh connection to you server run `fissh vm-01`.
