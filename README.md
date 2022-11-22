# Password Generator

This is a command line tool to generate random passwords. Passwords can be generated as:

1. An [XKCD](https://xkcd.com/936/)-style memorable password
2. A random string of ASCII characters
3. A pin number

## Usage

To install:
```
$ git clone git@github.com:jephdo/password.git
$ cd password
$ cargo build --release
$ cp target/release/password $HOME/.cargo/bin/
$ password pin
9246
```

To generate a memorable password:
* `-n`, `--num-words` the number of words to randomly choose. Defaults to `3`
* `-s`, `--separator` optional separator character to place in between words
* `-c`, `--capitalize` randomly uppercase one of the words in the string
```
$ password xkcd
causemightdesktop
$ password xkcd -n 6
ratsattentionbrasstagsmaterialcloser
$ password xkcd -n 4 -s '^' -c
footwear^challenging^MATTER^lost
```

For random passwords:
* `-l`, `--length` the number of characters in the password
  
```
$ password random 
gbDRm7D0vDXF
$ password random -l 30
StjI15pJKWbDR1oJSlo3RHLJit7pED
```

For pin numbers:
* `-l`, `--length` the number of digits in the pin number

```
$ password pin
7768
$ password pin -l 8
84282811
```