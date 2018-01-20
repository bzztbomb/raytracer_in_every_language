Ray Tracer in Every Language
============================

The goal of this project is to implement the ray tracer described in the mini books [Ray Tracing in One Weekend](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html) and [Ray Tracing the Next Week](http://in1weekend.blogspot.com/2016/01/ray-tracing-second-weekend.html) in languages I don't use all of the time but I am interested in learning more about.  I've only done Python so far, but on the horizon are:

* Ruby
* Racket
* ChezScheme
* Rust
* Go

Python
------

![Cornell](python/cornell.png)
![Final](python/final.png)

This was a pretty straightforward port of the C++ code over to Python.  I did end up adding some parallelism via forking so that the render times were only days instead of weeks.  Python seems pretty slow if you don't use numpy to accelerate everything.  I also had to avoid NAN situations that the C++ code just dealt with by being careful with the comparisons that were being done.  The spots were in the Dielectric material and AABB hit function.