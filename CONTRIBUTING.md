# Contributing

Contribution to this project is welcomed but we ask that you adhere to the
following guidelines where applicable.

## Conduct

This project adopts the [Rust Code of Conduct][conduct] to provide a welcoming
environment for all contributors. Any queries or concerns relating to conduct
should be directed to the maintainers of this project and not the moderation
team as they are not affiliated.

## Issues

Issues relating to bug reports, feature requests and general enquiries are all
welcomed. This project provides some common issue labels to help users identify
issues accordingly.

Please be considerate, polite, clear and concise for the sake of other
contributors. See the code of conduct for further information about acceptable
use of the issue queue.

## Pull Requests

Pull requests relating to new features or bugfixes are generally welcome but
please be aware that significant changes are unlikely to be merged without a
corresponding issue to solicit feedback.

Note that pull requests are a place to discuss and critique the code presented
and therefore not suitable for general discussion. Instead such comments should
be made in a related issue.

### Branching

This repository makes use of a trunk-based development model where changes are
made in topic branches that are branched from _master_. This methodology allows
the _master_ branch to represent the latest iteration of the project while
ensuring stability through the use of continuous integration.

We recommend that topic branches use the following naming conventions:

* `feature/feature-name`
* `feature/feature-area/feature-name`
* `bugfix/description`
* `hotfix/description`

### Merging

This project makes use of continuous integration to ensure that any changes are
up to date with the latest coding standards and passing relevant tests. As part
of the pull request flow we require that these checks pass before allowing the
the pull request to be merged.

The accepted method of merging pull requests in this repository is to use the
_squash and merge_ functionality provided by _GitHub_. This allows us to reduce
the burden on developers by not enforcing a local commit strategy. Developers
are then able to commit locally as often as they want without having to worry
about squashing or wording commit messages to ensure a clean history. It also
allows them to push as often as they need in order to test their code against
the continuous integration pipeline without later having to force-push a clean
branch.

The use of _squash and merge_ allows us to keep a clean history where each
commit represents a single unit of change, e.g. a new feature or bugfix. In
order to keep consistency we recommend that each squash commit be made with a
short commit message and a reference to the merge request as is set by default
when starting the merge process.

Note that because squash merging reduces commit history down to a single commit
per pull request it is recommended that each pull request only represent a
single small change and any large changes be broken down in to multiple smaller
ones whenever possible.

## Developing

### Install rust

See the rust [installation documentation][install-rust] or use the following
command to install the rust toolchain installer _rustup_.

```
$ curl https://sh.rustup.rs -sSf | sh
```

### Install development tools (optional)

This project makes use of continuous integration to perform automated linting
and testing. Pull requests cannot be merged until all checks have passed so it
is advisable to run the checks locally before pushing to the remote repository.

The following command installs _cargo-make_, a task runner which is used to
simplify the execution of tasks that can be performed.

```
$ cargo install cargo-make
```

With this tool you can run various commands such as `cargo make lint` and
`cargo make test` to manually invoke linting and testing tasks respectively.

The following commands can be used to setup your local development environment
to use git hooks that lint on commit and test on push. Be aware that the hooks
are dependent on the _cargo-make_ tool as described above.

```
$ cargo install rusty-hook
$ rusty-hook init
```

[conduct]: https://github.com/rust-lang/rust/blob/master/CODE_OF_CONDUCT.md
[install-rust]: https://www.rust-lang.org/tools/install
