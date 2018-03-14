# bergen
A mountain inspired programming language.

## Description
Created during [Booster Conference 2018][conference] in Bergen, Norway, `bergen` is a dialect of [brainf*ck][].

Mountain ranges our used to represent the different operators. Below you can find a list.

|Mountain Range|Corresponding Command|Meaning|
|--------------|---------------------|-------|
| `/\`         | `>`                 | increment the data pointer. |
| `/\`         | `<`                 | decrement the data pointer. |
| `/\`         | `+`                 | increment the byte at the data pointer. |
| `/\`         | `-`                 | decrement the byte at the data pointer. |
| `/\`         | `[`                 | if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `]` command. |
| `/\`         | `]`                 | if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `[` command.|
| `/\`         | `.`                 | output the byte at the data pointer. |
| `/\`         | `,`                 | accept one byte of input, storing its value in the byte at the data pointer. |

[conference]: https://2018.boosterconf.no/
[brainf*ck]: https://en.wikipedia.org/wiki/Brainfuck
