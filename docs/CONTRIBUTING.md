# Contribution Guidelines

## Introduction

Thanks for looking at contributing to pa'i! Your help makes pa'i better for
everyone!

Following these guidelines helps to communicate that you respect the time of
the developers managing and developing this open source project. In return,
they should reciprocate that respect in addressing your issue, assessing
changes, and helping you finalize your pull requests.

pa'i is an open source project and we love to receive contributions from our
community — you! There are many ways to contribute, from writing tutorials or
blog posts, improving the documentation, submitting bug reports and feature
requests or writing code which can be incorporated into pa'i itself in the
future.

## Ground Rules

### Ensure code is tested somehow

This project is a bit odd to test. Please be sure that any new functionality
(or patches to existing functionality) is tested _somehow_. There is a test
runner in /tests that has a file named `testdata.dhall` that defines all of
the test cases that get run in CI.

### Create issues for any major changes and enhancements

Please also discuss things transparently and get community feedback. This is
a bit of a large target with a lot of moving parts. As things get more and
more stable, breaking changes are going to become more and more unacceptable.

### Be welcoming, embrace diversity

Be welcoming to newcomers and encourage diverse new contributors from all
backgrounds. This is an easy ticket to getting into low-level operating
systems programming. Make it seem like it. Also see the [code of conduct](./CODE_OF_CONDUCT.md).

## First-time Contributors

Working on your first Pull Request? You can learn how from this free
series, [How to Contribute to an Open Source Project on GitHub](https://egghead.io/series/how-to-contribute-to-an-open-source-project-on-github).

Feel free to ask for help; everyone is a beginner at first 😸

If a maintainer asks you to "rebase" your PR, they're saying that a lot of
code has changed, and that you need to update your branch so it's easier
to merge.

## Locally Building pa'i

To do a local build of pa'i in debug mode, install [Nix](https://nixos.org/nix)
and [lorri](https://github.com/target/lorri). Then run:

```console
$ lorri shell
```

This will transparently download and install the Dhall tools, the Go compiler,
the Rust compiler and all of their dependencies. This will not modify your
system configuration, but may take up to a gigabyte or two of disk space.

To use this automatically, check out [direnv](https://direnv.net).

## Security Issues

See the [security policy](/security.md).

## Formatting

- Dhall code should be formatted using `dhall format`
- Go code should be formatted using `gofmt` or `goimports`
- Rust code should be formatted using `rustfmt`

## Community

Join `#pahi` on `irc.within.website` to chat with the core team. We should be
available most of the time we're awake.
