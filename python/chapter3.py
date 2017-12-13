import numpy
from random import randint
import matplotlib
from pylab import *
import math

from ray import *
from vector3 import *
	
def hit_sphere(center, radius, ray):
	oc = ray.origin - center
	a = ray.direction.dot(ray.direction)
	b = 2.0 * ray.direction.dot(oc)
	c = oc.dot(oc) - radius*radius
	discr = b*b - 4*a*c
	if discr < 0:
		return -1
	else:
		return (-b - math.sqrt(discr)) / (2*a)
	
def col(ray):
	t = hit_sphere(Vector3(0,0,-1), 0.5, r)
	if t > 0:
		n = r.point_at_t(t) - Vector3(0,0,-1)
		n = n.unit()
		return (n + Vector3(1,1,1)) * 0.5
	unit_direction = ray.direction.unit()
	t = 0.5 * (unit_direction.y + 1.0)
	return Vector3(1,1,1)*(1-t) + Vector3(0.5, 0.7, 1) * t
	
RED = 0
GREEN = 1
BLUE = 2

rows = 100
cols = 200
channels = 3
image_array = numpy.zeros(rows*cols*channels).reshape(rows, cols, channels)

lower_left_corner = Vector3(-2,-1,-1)
horizontal = Vector3(4,0,0)
vertical = Vector3(0,2,0)
origin = Vector3(0,0,0)

for y in range(rows):
	flippedy = rows - y - 1
	for x in range(cols):
		u = x / cols
		v = flippedy / rows
		r = Ray(origin, lower_left_corner + horizontal * u + vertical * v)
		c = col(r)
		image_array[flippedy, x, RED] = c.x
		image_array[flippedy, x, GREEN] = c.y
		image_array[flippedy, x, BLUE] = c.z

imshow(image_array, interpolation='None')
show()
