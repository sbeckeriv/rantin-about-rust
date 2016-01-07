---
layout: post
title: "Personal Crates"
date: 2016-01-06 23:23:45 -0800
tags: rust crate cargo
---
Personal Crates

I have found myself copying around bits of code from example to example.
I have my simple timer and stdin_lines for example. Useful for my little
code bits but not anything I would ever want to unleash on to the world.
However, it does seem like something cargo should be handling for me.

Thank fully crates can be directly loaded from git! The [docs] are
pretty darn clear. I guess I could use an absolute path as well but my
github stats need some help.


1. Create a bare github repo
2. Use 'cargo new crate-name' it defaults to a library
3. Copy your already build mods to /src
4. add 'pub mod mod-file-name' to /lib.rs
5. commit and push to git. (2 steps)

### When building a new project:
{% highlight rust%}
/*
Add to Cargo.toml
[dependencies]
my-helpers = { git = "https://github.com/sbeckeriv/really_bad_idea" }

src/main.rs
*/
extern crate my_helpers;
use my_helpers::{timer, stdin_lines};
{% endhighlight %}

Start braggin about all your crates!

This code was ran with rustc 1.5.0 (3d7cd77e4 2015-12-04).

[docs]: http://doc.crates.io/manifest.html#the-dependencies-section
[algorithms helpers]: https://github.com/sbeckeriv/rust-algorithms-helpers
