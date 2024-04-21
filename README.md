# anytest [![Lint](https://github.com/timfjord/anytest-cli/actions/workflows/lint.yml/badge.svg)](https://github.com/timfjord/anytest-cli/actions/workflows/lint.yml) [![Test](https://github.com/timfjord/anytest-cli/actions/workflows/test.yml/badge.svg)](https://github.com/timfjord/anytest-cli/actions/workflows/test.yml)

<!-- markdownlint-enable -->

Run any test from your terminal.

Many test frameworks allow running a single test, but they all use different formats. The majority require to build quite complex queries that are hard to type and even remember. This tool aims to provide a unified way to run a single test by specifying only the file and the line number (following how RSpec/ExUnit/ESpec does that).
The tool is heavily inspired by the awesome [vim-test](https://github.com/vim-test/vim-test) plugin.

Currently, the following test frameworks are supported (and more are coming soon):

|       Language | Test framework | Identifiers       |
| -------------: | :------------- | :---------------- |
|     **Elixir** | ESpec, ExUnit  | `espec`, `exunit` |
| **JavaScript** | Jest           | `jest`            |
|     **Python** | PyTest         | `pytest`          |
|       **Ruby** | RSpec          | `rspec`           |
|       **Rust** | Cargo          | `cargotest`       |
|        **Zig** | Zigtest        | `zigtest`         |

Feel free to [open an issue](https://github.com/timfjord/anytest-cli/issues/new) with a test framework request as those test frameworks will be added first.

## Installation

Currently, `anytest` is distributed via [crates.io](https://crates.io/crates/anytest) so Rust is required to install it.
To install Rust, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

To build `anytest` from source use:

```sh
cargo install anytest
```

## Usage

It is as simple as running `anytest <file>:<line>`:

```sh
anytest tests/test_rust.rs:10
```

The tool supports the following scopes:

- `suite` - run the whole suite
- `file` - run all tests in the file
- `line` - run the test at the specified line

When the the path is specified with the line number (e.g. `anytest tests/test_rust.rs:10`), the scope is automatically set to `line`.
When the path is specified without the line number (e.g. `anytest tests/test_rust.rs`), the scope is automatically set to `file`.

The scope can be explicitly set with the `--scope`/`-s` flag:

```sh
anytest tests/test_rust.rs:10 -s file
```

By default, `anytest` uses the current working directory as the root directory. The root directory can be explicitly set with the `--root`/`-r` flag:

```sh
anytest anytest-cli/tests/test_rust.rs:10 -r anytest-cli
```

When the root directory is specified, the path to the test can be either relative to the root (e.g. `anytest tests/test_rust.rs:10 -r anytest-cli`) or relative to the current working directory (e.g. `anytest anytest-cli/tests/test_rust.rs:10 -r anytest-cli`).

To run the tool in the dry-run mode, use the `--dry-run`/ flag:

```sh
anytest tests/test_rust.rs:10 --dry-run
```

To see the list of all available options, run:

```sh
anytest --help
```

## Usage with Zed

Even though `anytest` is a standalone tool, the main driver behind writing it was to use it with [Zed](https://zed.dev).
Currently, Zed has limited extension support, so the only way to integrate `anytest` with Zed is to use [Zed tasks](https://zed.dev/docs/tasks).

The repository contains [an example `tasks.json` file](https://github.com/timfjord/anytest-cli/blob/main/examples/tasks.json) that can be copied to the Zed configuration directory with the following command:

```sh
wget https://raw.githubusercontent.com/timfjord/anytest-cli/main/examples/tasks.json -O ~/.config/zed/tasks.json -nc
```

Please note that the command above won't overwrite the existing `tasks.json` file. Remove the `-nc` flag to overwrite the existing file.

Once the tasks are set up, use either the `task: spawn` command or add keybindings to run the tests.

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-j cmd-s": ["task::Spawn", { "task_name": "anytest: test suite" }],
      "cmd-j cmd-f": ["task::Spawn", { "task_name": "anytest: test file" }],
      "cmd-j cmd-l": ["task::Spawn", { "task_name": "anytest: test line" }]
    }
  }
]
```

## Contribution

The easiest way to add a new test framework is to find it either in [the `AnyTest` repository](https://github.com/timfjord/AnyTest/tree/main/plugin/test_frameworks) or [the `vim-test` repository](https://github.com/vim-test/vim-test/tree/master/autoload/test) and try to adapt it.
It is also required to cover the test frameworks with tests. Tests and fixtures can be also found in either [the `AnyTest` repository](https://github.com/timfjord/AnyTest/tree/main/tests/test_frameworks) or [the `vim-test` repository](https://github.com/vim-test/vim-test/tree/master/spec)

## Credits

`anytest` is heavily inspired by the [vim-test](https://github.com/vim-test/vim-test) plugin so all credits go to the authors and maintainers of this awesome Vim plugin.
