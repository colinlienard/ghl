# `ghl`

A small CLI to create pull requests linked to Linear in seconds.

> [!NOTE]
>
> - Following the [Conventional Commits](https://www.conventionalcommits.org) specification.
> - Only compatible with MacOS.

## Install

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
ghl pr
```

This will ask you the Linear git branch name of an issue, and the name of the pull request that will be created. Then it will:

1. Create a new branch.
2. Create an empty commit.
3. Push to the remote repository.
4. Create a pull request.
5. Assign you the pull request.

### Other commands

| Command         | Description                                                      |
| --------------- | ---------------------------------------------------------------- |
| `push`          | Create a new conventional commit, create a new branch, and push. |
| `commit`, `-c`  | Create a new conventional commit.                                |
| `version`, `-v` | Display the current and the latest version.                      |
| `update`, `-up` | Update the binary to the latest version.                         |

## License

[MIT](./LICENSE) © Colin Lienard
