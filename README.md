# Web Directory Brute Forcing

A command-line interface (CLI) tool written in `Rust` that enables the exploration of web server directories to identify potential vulnerabilities for an attack.

> What is web directory brute forcing ?

Directory Brute Forcing (also known as Directory Bursting or Directory Bursting) is the process of trying to find hidden or unprotected directories and files on a computer.
Find hidden or unprotected directories and files on a web server using a server using a tool or script.
It is often used in web application security tests to identify potential vulnerabilities. vulnerabilities.
You will create a tool that will accept a list of words and attempt to discover the directories and files accessible on the target web server.

`Translated with DeepL.com`

## Author

Full name: ANDRIANARISOA Hajatiana Tantely
Email: hei.hajatiana@gmail.com
REF: STD21038

## Install and run locally

There is actually no Build executable for this CLI, so you have to build it from source

- Ensure you have rust Cargo installed on your machine, if not you can download it [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- In order to build binary, ensure you have compatible target for your machine in rustup
- Running it:
  passing '--' to tell that the args are for the cli instead of cargo
  ```shell
  cargo run -- -b <base_url> -p <word list file path>
  ```
- Build:

  ```shell
  cargo build --release
  ```

- You can also build and install with the using the following command:

  ```shell
  cargo install --path <cloned repo path>
  ```
