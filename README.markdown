Ray Tracer in Every Language
============================

The goal of this project is to implement the ray tracer described in the mini books [Ray Tracing in One Weekend](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html) and [Ray Tracing the Next Week](http://in1weekend.blogspot.com/2016/01/ray-tracing-second-weekend.html) in languages I don't use all of the time but I am interested in learning more about.  I've only done Python so far, but on the horizon are:

* Go
* Ruby
* Racket
* ChezScheme

Rust
----

![Final](rust/raytrace/final.png)

Been meaning to learn Rust for a while, this was a fun project to use for it. The borrow checker isn't as bad as people say, and if you want
to not think about it too much, just make everything reference counted with Arc<>/Rc<>, heh. I was able to add threading very easily with the [Rayon](https://crates.io/crates/rayon) crate. I also took a quick detour and made a WebAssembly port of the raytracer using [RustWasm](https://rustwasm.github.io/book/game-of-life/introduction.html).  I haven't maintained it, so you may need to go back a few [commits](https://github.com/bzztbomb/raytracer_in_every_language/commit/b2ab73af3e4ec9b8078a2d197adceddfc881e918) if you want a working version. Overall, I enjoyed writing code in Rust and hope to do more in the future.

This time through I noticed a typo in the book for the final image, the metal sphere needs to have a fuzz of 1.0, not 10.0.  10.0 is basically diffuse.  Also, in contrast the python version, I'm outputting PFM file in HDR and then viewing the results in [HDRView](https://bitbucket.org/wkjarosz/hdrview) so I could play with tonemapping and get away from my hack to bring everything down to 0..1 range.

Python
------

![Cornell](python/cornell.png)
![Final](python/final.png)

This was a pretty straightforward port of the C++ code over to Python.  I've used Python a bit, but I always feel like I'm just writing C in Python.  With all of the machine learning libraries available for Python that I'm interested in I decided I wanted to try and dive deeper into it.  Python seems pretty slow if you don't use numpy to accelerate everything. I did end up adding some parallelism via forking so that the render times were only days instead of weeks. I also had to avoid NAN situations that the C++ code just dealt with by being careful with the comparisons that were being done.  The spots were in the Dielectric material and AABB hit function. But it was pretty pleasnt language to use.  I would have liked to explore generators a bit more, but I'll find another project for that.
