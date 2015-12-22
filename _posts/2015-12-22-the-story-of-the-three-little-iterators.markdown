---
layout: post
title: "Story of the three little iterators"
date: 2015-12-22 23:23:45 -0800
tags: rust iterators iter IntoIter
---
Story of the three little iterators.

My learning example.

I was implementing a simple "databag" from the algorithms book. I
decided to use a vec as the base storage object. I thought I could
simple use the vec's implementation of iterator and it would be simple.

### The first little iterator took ownership:
{% highlight rust %}
pub struct Ziploc<T> {
    data: Vec<T>,
}

impl<T> IntoIterator for Ziploc<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
{% endhighlight %}

This code is fine if you never need to use the object after the loop. Iterating by value will be [moved]. The documentation on the error gives a few suggestions on how to work around this.  Or we can use the iterate by reference.

### The second little iterator took a slice:
{% highlight rust %}
impl<'a, T> IntoIterator for &'a Ziploc<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
{% endhighlight %}

>View the underlying data as a subslice of the original data.
>This has the same lifetime as the original slice, and so the iterator can continue to be used while this exists.

This prevents the object from moving allowing us to keep using it. What
I would normally want from my iterator.

Fun side note you can you implement more then one iterator for a struct.
An example can be found at the bottom. Also I learned that the main struct does
not need lifetime specifier if you are using them in the impl.


### The third little iterator liked to change:
{% highlight rust %}
impl<'a, T> IntoIterator for &'a mut Ziploc<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
//main
    let mut z = Ziploc { data: vec![1,2,3] };
    for i in &mut z {
      *i = 1;
    }
{% endhighlight %}

A mutable iterator is the last of the 3. True be told I had to read the
[array code] to figure it out. Now for that hard part. When do you use
which? Searching the rust code base in github is nice because it
includes the rfc. I found [RFC: conventions for ownership variants]. It
is nice to know where things come from.

Full files and github history [here]

[RFC: conventions for ownership variants]: https://github.com/rust-lang/rfcs/pull/199/files
[array code]: https://github.com/rust-lang/rust/blob/e819d8aa3cd2319fa57e7336e167069ef7002d6a/src/libcore/array.rs#L160
[playground]: http://is.gd/tb0yLS
[here]: https://github.com/sbeckeriv/rust-algorithms/tree/master/chapter-1/3/simple_bag/src
[moved]: https://doc.rust-lang.org/error-index.html#E0382

This code was run with rustc 1.5.0 (3d7cd77e4 2015-12-04).

{% highlight rust %}
//example from mbrubeck on irc
pub struct Ziploc<T> {
    data: Vec<T>,
}
impl<T> Ziploc<T> {
    pub fn new() -> Ziploc<T> {
        Ziploc { data: Vec::new() }
    }
}
impl<'a, T> IntoIterator for &'a Ziploc<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
impl<T> IntoIterator for Ziploc<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
fn main() {
    let z = Ziploc { data: vec![1,2,3] };
    // iterate by reference
    for i in &z {
        println!("{}", *i);
    }
    // iterate by value
    for i in z {
        println!("{}", i);
    }
    // can not use z again
}
{% endhighlight %}
