# Sandbox

## Syntax

Use as:

```sh
sandbox <image> <tag> <port>
```

For example:

```sh
sandbox alpine latest 8080
```

## Control

- `POST /sh` - Executes given command in shell
  - *Body* : Raw text (shell command)
  - *Return Code*
    - 200 : Execution successfully completed
    - 201 : Execution is started, but detached
    - 500 : Internal error
