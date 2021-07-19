# linkbox-cli
A command line interface for [linkbox](https://github.com/ashisbored/linkbox-backend)

## Features
- Link creation, viewing and deletion

## Usage
1. Download a binary of the tool from [the releases](https://github.com/ashisbored/linkbox-cli/releases) and place it somewhere on PATH.
2. Log in to your instance using `linkbox-cli login <instance url>`. For example, to log into an instance hosted at `https://linkbox.example.com`, you would use `linkbox login https://linkbox.example.com`
3. Create a new link using `linkbox-cli create <link> <note>`. For example, `linkbox-cli create https://example.com "Example website"`.
4. List links using `linkbox-cli list`.
5. Show a specific link with `linkbox-cli get <id>`.
6. Delete a link with `linkbox-cli delete <id>`.
7. You can log out (delete the saved token) with `linkbox-cli logout`.
