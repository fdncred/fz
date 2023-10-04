### Prototype Fuzzy Matcher using Nucleo

This is a prototype of the nucleo fuzzy matcher. Most of the code was borrowed from [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher/blob/master/examples/fz.rs)

In this example we list the directory using nushell. The result shows the `Car` part highlighted in red.

```nushell
ls | to text | fz car
name: Cargo.lock
name: Cargo.toml
```
