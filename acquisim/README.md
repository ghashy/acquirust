# acquisim

This is a bank simulator, which stores it's data on the RAM.
There are also [acqui](https://github.com/ghashy/acqui), written in Swift for macOS, which serves as an acquisim management client.

Currently, `acquisim` supports single-store account.

Acquisim was designed to be simple. It is able to creating/deleting accounts, opening credits, creating transactions, tracking balances, bank emission. With simple internal design, it purposed to offer real-life api interaction, just like in real acquiring services.

## Usage:

Build docker container yourself in this directory, or use pre-built image from docker hub:
```bash
docker pull ghashy/acquisim
```

You need to pass configuration file and secret file as secrets. For example, using docker-compose:
```yaml
services:
  acquisim:
    image: ghashy/acquisim:0.1
    expose:
      - "15100"
    secrets:
      - terminal-password
      - example-config
    environment:
      TERMINAL_PASSWORD_FILE: /run/secrets/terminal-password
      ACQUISIM_CONFIG_FILE: /run/secrets/example-config
secrets:
  example-config:
    file: secrets/example_config.yaml
  terminal-password:
    file: secrets/terminal_password.txt
```

After running, use [acqui](https://github.com/ghashy/acqui) for bank management, and aquisim-api for store-bank interaction.
