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

## Example

Container opened with:

```sh
sandbox alpine latest 8080
```

HTTP request:

```http request
POST http://127.0.0.1:8080/sh

uname
```

Expected response:

```
Linux
```