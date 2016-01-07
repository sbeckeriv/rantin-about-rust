---
layout: post
title: "chapter 1 done"
date: 2016-01-02 23:23:50 -0800
tags: rust
---
Chapter 1 Done!

As I wrap up Chapter 1 of [Algorithms, 4th Edition] I figured I would
fill in a blog post with some useful resources and my favorite snippets.

## Resources

* Links to the files in [Algorithms, 4th Edition] are useful. Downloading the large files for testing
* [Learning Rust With Entirely Too Many Linked Lists] for a resource of
examples and clarification
* \#rust-beginners 'an IRC channel that loves answering questions at any
depth'. Always helpful people!
* https://play.rust-lang.org/ For sharing examples. I just wish I could
access my old playgrounds as a list.
* [The rust book], [Rust by example] and [the docs]
* google

## Distractions

* [Rust Easy Hackathon By DoSelect] I started it but did not complete
it because I was deep in my book. I did copy the questions for later.
* [Xorting golf code] A friend linked this golf code question. It got
diving in deep until I had some code [to show based on the ruby example]. I would like to see it done rusty.
* [notty] has me excited for a new console!

## Code bits
{% highlight rust %}
// Like ruby's array 3 states [None, true, false] from xorting code
let mut empty: Vec<Option<bool>> = vec![None; 32];

// Like ruby's bit_length method
let bit = format!("{:b}", num).len() - 1;

//string to number
string.parse::<usize>().unwrap();

// Simple iterator post treat stdin like an iterator all in my code
let reader = stdin_lines::StdinLines::new();

// time things. from different parts of my code
let spent = timer::record(|| {
let mut x = read_ints(file_string);
x.sort();
println!("{:?}",count(x));
});
println!("{:?}", spent);
{% endhighlight %}

This code was ran with rustc 1.5.0 (3d7cd77e4 2015-12-04).

[notty]: https://github.com/withoutboats/notty
[the docs]: https://doc.rust-lang.org/stable/std/
[Algorithms, 4th Edition]: http://algs4.cs.princeton.edu/home/
[Learning Rust With Entirely Too Many Linked Lists]: http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html
[The rust book]: https://doc.rust-lang.org/nightly/book/
[Rust by example]: http://rustbyexample.com/
[Rust Easy Hackathon By DoSelect]: https://doselect.com/hackathon/doselect-rust-easy-hackathon
[Xorting golf code]: http://codegolf.stackexchange.com/questions/68109/xorting-an-array
[to show based on the ruby example]: https://github.com/sbeckeriv/xorting-rust
