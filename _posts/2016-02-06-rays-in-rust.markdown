---
layout: post
title: "Ray tracing in one weekend"
date: 2016-02-06 01:23:45 -0800
tags: rust raytracing
---
Ray tracing in one weekend

I saw a post for [Ray tracing in one weekend]. Google for ray tracer
rust and you will find no end to projects and post. I found [tray_rust]
a really interesting concept. So knowing little about ray tracing I
thought I could spare a weekend to play with it, of course in rust.

The book is not to much $2.99, surely I have spent more on less. I first read
the whole thing without writing any code. Reading the code examples in c
plus first gave me a good idea how the whole thing will be structured.
The basic concepts where explained in an easy to digest fashion. I did
not feel bogged down with the math or highly technical visualization terms. With
that said I will not be engaging in an intellectual conversion about ray
tracing anytime soon.

First thing I did was look for crates that would help. I knew about
[nalgebra] which thankfully had a vec3. I found the ppm file format fun. I
was a little disappointed it is not rendered in browsers. In the end I
implemented the books code to write the file, however, I am using
[image from piston] . I am writing a jpeg and ppm with the same bit
of code. The docs make it pretty darn easy to understand and use.

Each chapter of the book builds up the code you are going to use. It was
a pretty easy conversion to rust with a few changes.

In the c code it was common to pass in an object and mutate it and return
boolean if the object should be used. My first big change is to make that an Option.
Which lead me to a bug in the C code. I contacted the author and asked
why they were setting the value but returning false. The value was set
but it would never be used. In rust it did not make sense to set the
value and return None.

The second departure I needed the help of irc to figure out how it
should work. Short of it is called dynamic dispatch in rust. Long
version is that every object has a material which will scatter the ray.
Implementing one was simple. Just pass the material in. Moving from one
to many was a problem. I first thought about making a enum of all the
types but that did not feel correct as I wrote it out. I also think it
would not have worked. I moved the scatter method to a trait and
implemented it for each material.

### A material:
{% highlight rust%}
pub trait Reflect{
fn reflect(&self, v: &Vec3<f32>, n: &Vec3<f32>) -> Vec3<f32> {
*v - (*n * v.dot(n) * 2.0)
}
}

pub trait Scatter{
fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)>;
}
pub struct Metal {
albedo: Vec3<f32>,
fuzz: f32,
}
impl Metal {
// move to util
pub fn new(albedo: Vec3<f32>, fuzz: f32) -> Self { ... }
}

// I dont like that scatter needs reflect. I am unsure how to force a relationship here.
impl Reflect for Metal {}
impl Scatter for Metal {
fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)> { }
}
{% endhighlight %}

Simple enough. We have some metal with with math. I left my comment in the example. Scatter is calling reflect. I can not change the signature of scatter because not all scatters need reflect. I am guessing this is the job of the compiler to make sure that my object has reflect when I call it. It is rather reassuring compared to ruby, but thats not the point. Now that we an example material lets look at where it matters for an object.

### An object:
{% highlight rust%}
pub struct Sphere {
pub center: Vec3<f32>,
pub radius: f32,
pub material: Rc<Scatter>,
}
impl Sphere {
pub fn new(center: Vec3<f32>, radius: f32, material: Rc<Scatter>) -> Self {
Sphere {
center: center,
radius: radius,
material: material,
}
}
}
{% endhighlight %}

The first thing I noticed is the Rc. Why do we need the reference counter? Well traits generally have no size so we need to wrap it in a pointer which in rust has a set size. This allows us to pass any object that implements scatter and call scatter methods. This to me is amazing. I wish I had something like this in ruby. I am sure I am missing some details about deconstructors or speed but right now I do not care. It understand you can also use a Box or Arc. I choose not to use an Arc because materials do not need to be mutated currently.


In the end my code is not producing the same images as in the book. Up until I added glass it was pretty spot on. The [full code] is on github. Hopefully I will make it spot on one day. Another personal win for me was the fact that I could write a big chunk of rust and have it compile with few or no compile issues. Of course my code lacks pointers and explicit lifetimes.

This code was ran with rustc 1.5.0 (3d7cd77e4 2015-12-04).

[full code]: https://github.com/sbeckeriv/Ray-Tracing-in-One-Weekend-in-Rust
[image from piston]: https://github.com/PistonDevelopers/image
[nalgebra]: https://crates.io/crates/nalgebra/
[Ray tracing in one weekend]: http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html
[tray_rust]: http://www.willusher.io/2016/01/02/distributed-rendering-with-rust-and-mio/
