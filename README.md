# dsefs-rs
A Rust based client library for DSEFS

A timeline of early development:

1. ~~Create a skeleton project with a -rs suffix~~
2. ~~Choose a REST client library and add it to the Cargo.toml~~ Trying [crest](https://github.com/pablocouto/crest) for now
3. ~~Choose a configuration file format and library~~ Using [yaml-rust](http://chyh1990.github.io/yaml-rust/)

Prerequisites:
* DSE 5.0
* dsefs=true set in dse.yaml
* assumption is that DSEFS is available on http://127.0.0.1:5598/fs/ Change the host and port in dsefs-rs.yaml, if needed