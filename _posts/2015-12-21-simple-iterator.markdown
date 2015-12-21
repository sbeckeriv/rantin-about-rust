---
layout: post
title: "Simple iterator"
date: 2015-12-21 01:23:45 -0800
tags: rust const static mutability
---
A simple iterator

An example in the algorithms book is a whitelist filter. The whitelist
file name is the first argument to the program and the list to be
filtered is streamed in to the app. The files contains just numbers.

The first thing I had to figure out is to read the streamed file. Using
io::stdin you can read all or read a single line. I quickly figured out
that the read_line method never gets to an end.

My first cut:
{% highlight rust %}
    let mut input = String::new();
    loop{
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.is_empty(){
                    break
                }
                let trimmed = input.trim();
                match trimmed.parse::<u64>() {
                    Ok(num) => {
                        if !whitelist.contains(num) {
                            println!("{:?}", trimmed);
                        }
                    }
                    Err(num) => println!("Could not parse: {:?}", trimmed),
                }
                input.clear()
            }
            Err(error) => {break}
        }
    }
{% endhighlight %}

This code does not run because of the borrow checker.  After quickly looking a fool on irc I learned that trim() does not mutate self. So when I assign trimmed from input.trim() it is borrowed as a read (I dont think that is the correct vernacular). The fix for this problem is simple just wrap the trimmed code in its own scope.

{% highlight rust %}
    let mut input = String::new();
    loop{
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.is_empty(){
                    break
                }
                {
                let mut trimmed = input.trim();
                match trimmed.parse::<u64>() {
                    Ok(num) => {
                        if !whitelist.contains(num) {
                            println!("{:?}", trimmed);
                        }
                    }
                    Err(num) => println!("Could not parse: {:?}", trimmed),
                }
                }
                input.clear()
            }
            Err(error) => {break}
        }
    }
{% endhighlight %}

It works but its a little ugly. It is just some sample code from a book but it does not have to be all bad. Someone (I forget who sorry) on irc suggested I pull the trimmed code out in to a function vs an anonymous scope. After taking a walk I had the thought of making the stdin read lines an iterator. It turned out to be easier then I thought.

{% highlight rust %}
//src/stdin_lines.rs
use std::io;
pub struct StdinLines {
    count: i32,
}

impl StdinLines {
    pub fn new() -> StdinLines {
        StdinLines { count: 0 }
    }
}

impl Iterator for StdinLines {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.count = self.count + 1;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input),
            Err(_) => None,
        }
    }
}

//src/main.rs
    let stdin_lines = stdin_lines::StdinLines::new();
    for input in stdin_lines {
        if input.is_empty() {
            break;
        }
        let trimmed = input.trim();
        match trimmed.parse::<u64>() {
            Ok(num) => {
                if !whitelist.contains(num) {
                    println!("{:?}", trimmed);
                }
            }
            Err(_) => println!("Could not parse: {:?}", trimmed),
        }
    }
{% endhighlight %}

I did not need to maintain state. I am sure if something else were to read stdin my code would miss that line. Ignoring that problem I also have the blemish of count. Count is useless, however, rust does not like an empty struct. I guess I could have aliased (not the right vernacular) None for an empty struct. I think it looks cleaner. Hopefully one day I will use an Iterator that needs to maintain state.

Full files and github history [here]

[here]: https://github.com/sbeckeriv/rust-algorithms/tree/master/chapter-1/2/whitelist

This code was ran with rustc 1.5.0 (3d7cd77e4 2015-12-04).
