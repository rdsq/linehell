# linehell

A language where nesting is illegal and extreme verbosity is mandatory

## Core principles

- **One line = one operation**. You cannot assign and calculate at once. You cannot even assign a literal at once

- **No nested expressions**. Not even `a = 1`. First, create a string, then convert it, then assign

- **`that` ties it together**. Every operation works by putting a result into a special magical buffer called `that`. Next line reads from it. And the next. And the next

## Examples

### Hello World

```linehell
str Hello World!
print that
```

### Fibonacci sequence

```linehell
str Until which number to calculate? 
printnonl that
input
number that
set total that

str 2
number that
set i that

{
str false
bool that
set run-loop that
}
set stop-loop that

{
}
set void that

str true
bool that
set run-loop that

str 0
number that
set n1 that

str 1
number that
set n2 that

{
math n1 + n2
set next that

set n1 n2
set n2 next

print next

str 1
number that
math i + that
set i that

math i > total
if-else that stop-loop void
}
while run-loop that
```

## Status

- **Unfinished?** Maybe

- **Usable?** Technically, yes

- **Stable?** Does it matter anyways?

## Installation & Usage

It is made in **Rust**, so the whole process is pretty simple. Just make sure you have **Cargo**

Just clone this repository, run `cargo run`, that's all

You can run REPL, execute a file, it's all intuitive

To install it, you can run:

```sh
cargo install --git https://github.com/rdsq/linehell
```

## Functions

- `set <name> <value>` - set a **variable**. Just don't forget that `value` is **not** a literal

- `str <string>` - initialize a **string** literal. You can then access it from `that`

- `print <values>` - **print** something to stdout with a new line. *No literals*

- `math <num1> <operation> <num2>` - do **math**. *Again, no literals*. Supported operations: `+` `-` `*` `/` `>` `<`

- `number <from>` - convert a string to a **number**. Again, no literals

- `bool <from>` - initialize a **boolean** value from a string. *Still no literals*

- `printnonl <values>` - like **print**, but without **new line**. *LiTeRaLs again*

- `input` - get **input** from **stdin**

- `run <block>` - **run** a block. Blocks are defined with `{` and `}`, work basically as anonymous functions. Blocks don't take any arguments, *at least currently*

- `if-else <condition> <if-block> <else-block>` - **if else** logic. If `condition` is `true`, run the `if-block`. If not, `else-block`

- `equal <values>` - check if values are **equal**. *(Not literals by the way)*

- `while <condition> <block>` - run a **while loop**. As long as `condition` is true, run `block`

- `table` - initialize an empty **table**. It's basically a hashmap, but also serves the purpose of arrays

- `tableget <table> <key>` - **get** a table value. *Literals reminder*

- `tableinsert <table> <key> <value>` - **insert** a value to the given key. *(LITERALS)*

- `tableremove <table> <key>` - **remove** a value of a table by its key. *Yeah, you guessed it, literals*

- `concat <values>` - **concatenate** strings. *Do I need to write that again?*

- `newlinestr` - put the **new line character** into the buffer
