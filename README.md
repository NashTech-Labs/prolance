# Prolance: Protocol Surveylance tool in Rust
<p align="left">
  <a href="https://travis-ci.org/pankajchaudhary5/dialog-box">
    <img alt="Build Status" src="https://travis-ci.org/PankajChaudhary5/dialog-box.svg?branch=master">
  </a>
  <img alt="MIT licensed" src="https://img.shields.io/badge/license-MIT%2FApache-blue.svg">
  <img alt="Stability stable" src="https://img.shields.io/badge/stability-stable-green.svg">
</p>

[`Prolance`](https://blog.knoldus.com/prolance-protocol-surveillance-tool-in-rust/) is a tool that can monitor the running protocols on the server. So with the help of this tool, we can keep track of all the protocols. As of now, we have implemented this tool for two protocols those are Dynamic Host Configuration Protocol (DHCP) and Active Directory (AD).

## Features of Prolance
* Monitor the running activities of the protocols.
* Filter out the activity logs as per our monitoring requirements.
* The logs should stream continuously on the Kafka topic. 
* The logs should stream in the compressed form.
* The user can schedule this process according to his requirements.

You can checkout the [`blog`](https://blog.knoldus.com/prolance-protocol-surveillance-tool-in-rust/) for deep dive in this project.


## Setting up your environment
As you have seen above this project is now developed for active directory and DHCP so we need to setup Microsoft server 2012/2016.

### Rustup.rs

Building this project requires [rustup](https://rustup.rs/), version 1.20.0 or more recent.
If you have an older version, run `rustup self update`.

To install on Windows, download and run [`rustup-init.exe`](https://win.rustup.rs/)
then follow the onscreen instructions.

To install on other systems, run:

```
curl https://sh.rustup.rs -sSf | sh
```

This will also download the current stable version of Rust, which this project won’t use.
To skip that step, run instead:

```
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none
```
## Building

### Normal Build

```
git clone https://github.com/knoldus/kode-combat-2019-procespy.git
cd kode-combat-2019-procespy
cargo build
```

## Contributing
We thrive for the best and want you to contribute towards a better Project. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for giving your valuable feedbacks and contributions.

## License

Amethyst is free and open source software distributed under the terms of both the [MIT License][lm] and the [Apache License 2.0][la].

[lm]: MIT-LICENSE.md
[la]: APACHE-LICENSE.md

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
