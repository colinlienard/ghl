# `ghl`

A small CLI to create pull requests linked to Linear in seconds.

> [!NOTE]  
> This is made to be used at my company and is adapted to our rules.  
> It is also only compatible with Macos.

## Install (or update)

Run the following:

```bash
curl -o ghl -L https://github.com/colinlienard/ghl/releases/latest/download/ghl && chmod +x ghl && mv ghl ~/.local/bin/
```

## Usage

### Configuration

You first need to create a new GitHub token [here](https://github.com/settings/tokens/new), and select the **_repo_** scope.

Then, run the following and paste your token. You can also set the default pull requests description:

```bash
ghl config
```

### Creating a pull request

```bash
ghl create
```

This will ask you the Linear git branch name of an issue, and the name of the pull request that will be created. Then it will:

1. Create a new branch.
2. Create an empty commit.
3. Push to the remote repository.
4. Create a pull request.
5. Assign you the pull request.

## License

[MIT](./LICENSE) © Colin Lienard
