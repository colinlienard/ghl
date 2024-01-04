# `ghl`

A small CLI to create pull requests linked to Linear in seconds.

> [!NOTE]  
> This is made to be used at my company and is adapted to our rules.
> It is also only compatible with Macos.

## Install (or update)

Run the following:

```bash
curl -o ghl -L https://github.com/colinlienard/ghl/releases/latest/download/ghl && chmod +x ghl && sudo mv ghl /usr/local/bin/
```

## Usage

### Configuration

You first need to create a new GitHub token [here](https://github.com/settings/tokens/new), and select the _repo_ scope.

Then, run the following and paste your token. You can also set the default pull requests description:

```bash
ghl config
```

### Creating a pull request

```bash
ghl create
```

## License

[MIT](./LICENSE) © Colin Lienard
