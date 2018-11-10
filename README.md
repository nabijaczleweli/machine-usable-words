# machine-usable-words [![TravisCI Build Status](https://travis-ci.org/nabijaczleweli/machine-usable-words.svg?branch=master)](https://travis-ci.org/nabijaczleweli/machine-usable-words) [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/gbur6cwwihsqf5e1/branch/master?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/machine-usable-words/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
Sets of English words, sorted and segregated for machine consumption.

## [Manpage](https://rawcdn.githack.com/nabijaczleweli/machine-usable-words/man/machine-usable-words-generator.1.html)

## What?

A list of adjectives, nouns, and adverbs, pulled down from the internet, and segregated into Rust-includable source files as well as raw plaintext lists.

## Why?

This all started when I implemented gfycat-like ThreeLetterWord identifiers as order IDs for a billing system, which pulled the words down in the build script.
Suddenly, this same system was very useful in [other projects](https://github.com/nabijaczleweli/bloguen/blob/d8ff0d9843c3771917e4c632990bec628ea4914d/build.rs)
  and duplicating the logic across more and more `build.rs`es turned out messy, especially as tests started failing as the websites got updated and the wordsets changed.

## How?

### Generation

See the `generator/` folder for the very dirty implementation and the [manpage](https://rawcdn.githack.com/nabijaczleweli/machine-usable-words/man/machine-usable-words-generator.1.html) for direxions on using it.

### Usage

The `rust/` folder contains `{adjectives,nouns,adverbs}.rs`, each with respective word type set included as a `static` `&[&str]`. `words.rs` is simply an aggregate of all those files.

The `raw/` folder contains analogous files, but without any language-specific clutter.
