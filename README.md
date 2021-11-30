# ShrimpLang 3.0

A complete rework of ShrimpLang 2. This release is focused on speed, an improved codebase and better debugging features.

Oh, and also, `$` to access variables is being removed! (Yay!)

### Docs-ish (they are not good.)

##### - Libraries are here! Check out the [internet library](internet.md)!

### TODOs:
- Grow Shrimp!
### FIXMEs:
- Fix bug where the rest of the args will not be collected and sent to the program (low priority)

### Basic syntax

`;` is no longer required for a new line.

Comments must be enclosed in `#`, e.g `# A comment #`

A new syntax feature has been added, the arrow assigner (`->`).
It lets you assign the output of a function to a variable.

`for`,`while`, `decl`, `return` and `use` are the newly-introduced keywords.

`if` has been converted into a keyword.

Some other minor changes have been made, here is a taste of the syntax

```
# `args` is a required parameter in every
# program.
@main(args) {
	# Prints arguments given to the program #
	println(args)
}
```

```
@main(args) {
	# The below code works too! #
	println(args) println(args)
}
```

### Documentation

#### `push(Array {value}+) -> Array`
Pushes `n` values onto an Array type and returns that array. Does *not* modify the initial array.

#### `pop(Array) -> Array`
Removes the last value of an array and returns the array without that value.
Does *not* modify the actual array.

#### `index(Array Number) -> {value}`
Indexes an array. Panics if the index is out of bounds.

#### `index_v(Array Number) -> {value}`
Indexes an array. Returns `Void` if the index is out of bounds.
- Note: If there is a variable with `Void` in the array, it may be impossible to tell
if an error had occurred or if `Void` had been returned correctly.

#### `println({value}*)`
Prints out the specified values, and then prints a new line.

#### `print({value}*)`
Prints out the specified values.

#### `hello_world()`
Equivalent to `println("Hello world!")`, but uses less characters. Extremely practical and fundamental to the design of the language.

#### `eq({value}(2+))`
Checks if all the values specified are equal. Must specify at least two values.
`eq(1)` == invalid,
`eq(1 1)` == valid,
`eq(1 1 1)` == valid

#### `cmp(Number(2+))`
Checks if the first value is larger than all the values specified.

if x = 4,

`eq(` `cmp(x 1 2 2 3)` `true` `)`

but if x = 2

`eq(` `cmp(x 1 2 2 3)` `false` `)`

#### `range(Number Number) -> Array`
An exclusive range function..
`range(0 3)` -> `[ 0 1 2 ]`

#### `add(Number*)`
Adds together all the values provided to it. Returns `0` if no arguments are provided.

#### `sub(Number(1+))`
Subtracts all values from the first number specified.

#### `div(Number(1+))`
Divides the first number by all values provided.

#### `mult(Number(1+))`
Multiplies the first number by all values provided.

#### `pow(Number(1+))`
Raises the first number to the power of all values provided.

#### `xor(Number(1+))`
Applies a XOR gate to the number using all of the provided values.

### Keywords

#### `decl`
```
@main(args) {
	decl x "Hello world"
	println(x)
}
```

#### `if`
```
@main(args) {
	if true {
		println("This should always execute")
	}
	if eq(1 1) {
		println("This should also always execute")
	}
	if false {
		println("This should never execute")
	}
}
```

#### `for`
```
@main(args) {
	range(0 1000) -> my_range
	for my_range => num {
		# Prints the current value on each iteration #
		# There are 1000 iterations because there are 1000 values #
		println(num)
	}
}
```

#### `while`
```
@main(args) {
	while true {
		# Self explanatory. #
		println("Infinite loop!")
	}
}
```

#### `return`
```
@main(args) {
	my_function() -> x
	# Prints "Hello there!" #
	println(x)
}

@my_function() {
	return "Hello there!"
}
```

#### `del`
```
@main(args) {
	decl x "Hi"
	del x
	# This will error out because
	# `x` has been deleted.
	println(x)
}
```

#### `use`
```
@main(args) {
	# Imports the (built-in) internet library #
	use internet
	internet_get("https://sh.rustup.rs") -> rustup_script
	# Prints the shell script for Rustup's installer #
	println(rustup_script)
}
```