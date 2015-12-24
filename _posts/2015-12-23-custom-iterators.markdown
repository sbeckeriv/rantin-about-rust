---
layout: post
title: "Custom iterator"
date: 2015-12-23 23:23:45 -0800
tags: rust iterators iter IntoIter
---
Custom iterator

I found a need to implement a custom iterator. Now that I know the
difference between the three iterators I need to figure out how to
implement one our from scratch. I keep reading that for an iterator all
I need a next function. Simple enough.

### Just a next:
This code is not correct.
{% highlight rust %}

pub struct MaxStack<T> {
    max: usize,
    data: Vec<T>,
}

impl<'a, T> Iterator for MaxStack<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
			// something?
    }
}
{% endhighlight %}

What goes in the next method? I have setup my lifetimes correctly but
how to do I get the current index? It was not that clear to me from the
docs. You need to create an iterator object and implement iterator for that.

I was tripped up on the error

>  error: the parameter type `T` may not live long
> enough [E0309]

I learned that the first lifetime in the MaxStackIterator applies to the
structure. This does not apply to the generic.

### An Interator object
{% highlight rust %}
pub struct MaxStack<T> {
    max: usize,
    data: Vec<T>,
}

pub struct MaxStackIterator<'a, T: 'a> {
    index: usize,
    object: &'a MaxStack<T>,
}

impl<'a, T> Iterator for MaxStackIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      match self.object.data.get(self.index) {
          Some(item) => {
              self.current_pos = self.current_pos + 1;
              Some(item)
          }
          None => None,
      }
    }
}
{% endhighlight %}

This code just loops over the underlying vec. [My code] does reverse
the output. You can apply any logic you want in the next function. Now that I
have my iterator I need to apply it. I am borrowing the item so
I need to keep the lifetime syntax going for the MaxStack's
IntoIterator.

### The last connection
{% highlight rust %}
impl<'a, T> IntoIterator for &'a MaxStack<T> {
    type Item = &'a T;
    type IntoIter = MaxStackIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MaxStackIterator {
            current_pos: 0,
            object: self,
        }

    }
}
{% endhighlight %}

I could have added a new function to the MaxStackIterator but I did not
think it was needed since I only use it once. If you want one that takes
ownership or returns a mutable item you will need to write an iterator
for each.

[My code]: https://github.com/sbeckeriv/rust-algorithms/blob/master/chapter-1/3/max_stack/src/stackable.rs

This code was run with rustc 1.5.0 (3d7cd77e4 2015-12-04).

