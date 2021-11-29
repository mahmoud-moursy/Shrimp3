# ShrimpLang 3.0

A complete rework of ShrimpLang 2. This release is focused on speed, an improved codebase and better debugging features.

Oh, and also, `$` to access variables is being removed! (Yay!)

### Docs

Libraries are here! Check out the [internet library](internet.md)!

**Shrimp3 is not yet turing-complete! (No flow control)**

### TODOs:

- Array manipulation
- While loops
### FIXMEs:
- Fix bug where the rest of the args will not be collected and sent to the program

### Basic syntax

`;` is no longer required for a new line.

Comments must be enclosed in `#`, e.g `# A comment`

A new syntax feature has been added, the arrow assigner (`->`).
It lets you assign the output of a function to a variable.

`decl`, `return` and `use` are the newly-introduced keywords.

Some other minor changes have been made, here is a taste of the syntax

```
# `args` is a required parameter in every
# program.
@main(args) {
	# Prints arguments given to the program #
	println(args)
}
```