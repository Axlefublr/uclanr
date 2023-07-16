# Uclanr

Your mediocre brainstorming helper

## Why does this exist?

I love the unix style of binary names: they either have a cool history behind them, or they're a cute pun.

And see, I'm pretty bad at coming up with those on the spot when creating a new project, so I wanted to have a word generator to aid my inspiration.

Your usual word generator will give you 5 two-letter words before it gives you anything useful though, so I decided to make it give you only the more uncommon words.

With those, I now can generate brainstorming material at will!

## What does it do?

It prints a random words to stdout. Very simple.

If you specify a number after the command (`uclanr 5`), it will print that many random words, separated by spaces

The words this program uses are ranked by popularity: from 1001st most common word to the 10000th.

From there, I removed all words that are 4 letters long and below, leaving us with words that are at least 5 letters long.

And in the end, we have 7165 possible words. Hyperspecific, I know.

## How do I install this?

I haven't figured out how to publish packages yet, and idk how github releases work; so far I have this barebones solution:

Have git and rust installed

```
cargo install --git https://github.com/Axlefublr/uclanr.git
uclanr 5
```
