### Prototype Fuzzy Matcher using Nucleo

This is a prototype of the nucleo fuzzy matcher. Most of the code was borrowed from [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher/blob/master/examples/fz.rs)

In this example we list the directory using nushell. The result shows the `Car` part highlighted in red.

```nushell
❯ ["foo-bar", "baz-brr"] | to text | d:\cartar\debug\fz foo bar
method                 | match          | score | elapsed
-----------------------|----------------|-------|--------
Fuzzy                  | foo-bar        |    88 | 83.9µs
FuzzyIndices           | foo-bar        |    88 | 20µs
FuzzyGreedy            | foo-bar        |    88 | 11.7µs
FuzzyGreedyIndices     | foo-bar        |    88 | 2.8µs
Substring              | foo-bar        |    88 | 58.4µs
SubstringIndices       | foo-bar        |    88 | 4.1µs
Prefix                 | foo-bar        |    88 | 8.1µs
PrefixIndices          | foo-bar        |    88 | 2.8µs
Fuzzy                  | foo-bar        |    80 | 2.6µs
FuzzyIndices           | foo-bar        |    80 | 3µs
FuzzyGreedy            | foo-bar        |    80 | 3µs
FuzzyGreedyIndices     | foo-bar        |    80 | 2.9µs
Substring              | foo-bar        |    80 | 3.7µs
SubstringIndices       | foo-bar        |    80 | 3.6µs
Postfix                | foo-bar        |    80 | 1.5µs
PostfixIndices         | foo-bar        |    80 | 2.8µs
Fuzzy                  | baz-brr        |    73 | 54.8µs
FuzzyIndices           | baz-brr        |    73 | 60.8µs
FuzzyGreedy            | baz-brr        |    73 | 3.8µs
FuzzyGreedyIndices     | baz-brr        |    73 | 14.8µs
```
