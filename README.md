# Uclanr

Your hyperspecific brainstorming-focused word generator.

## Why does this exist?

I love the unix style of binary names: they either have a cool history behind them, or they're a cute pun.

And see, I'm pretty bad at coming up with those on the spot when creating a new project, so I wanted to have a word generator to aid my inspiration.

Your usual word generator will give you 5 two-letter words before it gives you anything useful though, so I decided to make it give you only the more uncommon words.

With those, I now can generate brainstorming material at will!

## What does it do?

It prints a random words to stdout. Very simple.

If you specify a number after the command (`uclanr 5`), it will print that many random words, separated by spaces.

The words this program uses are ranked by popularity: from 1001st most common word to the 10000th.

From there, I removed all words that are 4 letters long and below, leaving us with words that are at least 5 letters long.

And in the end, we have 7165 possible words. Hyperspecific, I know.

## Features

Run `uclanr --help` to see the help menu.

By default, the random words are joined by a space.
You can change that using `--joiner` or `-j`.
It doesn't necessarily have to be a single character.

If you specify `\n` in `--joiner`, it's treated as a newline character; `\t` is a tab character.
Use `--raw` or `-r` to disable this functionality.

Use `--case` or `-c` to specify the case of the words. Supported values are `caps` for Capitalized words, `title` for Title case words and `lower` for lower case letters which is a default.


## How do I install this?

```
cargo install uclanr
```

## Usage

```
$ uclanr --help
A word generator that specializes in useful words

Usage: uclanr.exe [OPTIONS] [AMOUNT]

Arguments:
  [AMOUNT]
          Amount of random words to print [default: 1]

Options:
  -j, --joiner <JOINER>
          The string that joines the random words, if there are more than 1 [default: " "]
  -c, --case <CASE>
          Sets the case of every word [default: lower] [possible values: caps, title, lower]
  -r, --raw
          Disable interpreting \n as a newline and \t as a tab
  -h, --help
          Print help
```

## Contribution

`git pull` the dev branch before any changes, please
