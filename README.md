# bergen
A mountain inspired programming language.

## Description
Created during [Booster Conference 2018][conference] in Bergen, Norway, `bergen` is a dialect of [brainf*ck][].

Mountain ranges our used to represent the different operators. Below you can find a list.

### Mountain Ranges
#### `>`
> increment the data pointer.

```
  /\
 /  \
/    \
```

#### `<`
> decrement the data pointer.

```
  /\/\
 /    \
/      \
```

#### `+`
> increment the byte at the data pointer.

```

 /\
/  \
```

#### `-`
> decrement the byte at the data pointer.

```

 /\/\
/    \
```

#### `[`
> if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `]` command.

```
  /\
 /  \/\
/      \
```

#### `]`
> if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `[` command.

```
    /\
 /\/  \
/      \
```

#### `.`
> output the byte at the data pointer.

```
/\
```

#### `,`
> accept one byte of input, storing its value in the byte at the data pointer.

```
  /\  /\
 /  \/  \
/        \
```

### States
The `bergen` parser can be in a few states while parsing input. Below we will
show the states and the allowed transitions.

![States of the `bergen` parser](https://cdn.rawgit.com/dvberkel/bergen/f7e438ef/states.png)

## Tools
The following tools are being implemented.

* `bergen`: a interpreter.
* `bergenc`: a bergen to brainf\*ck compiler.

[conference]: https://2018.boosterconf.no/
[brainf*ck]: https://en.wikipedia.org/wiki/Brainfuck
