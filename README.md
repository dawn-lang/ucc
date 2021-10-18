# Untyped Concatenative Calculus

The untyped concatenative calculus, implemented in Rust. A toy programming language and prototype for [Dawn](https://www.dawn-lang.org/).

## Native REPL

To build and run the native REPL:

```sh
cargo run
```

## Web REPL

To build the web REPL:

```sh
(cd ucci-web; wasm-pack build --target web)
```

To serve the web REPL using python's built-in http server:

```sh
(cd ucci-web; python3 -m http.server)
```

## License

Licensed under the [Mozilla Public License, v. 2.0](LICENSE).

## Contribution

Unless You explicitly state otherwise, any Contribution intentionally submitted
for inclusion in the Covered Software by You, as defined in the Mozilla Public
License, v. 2.0, shall be licensed as above, without any additional terms or
conditions.
