import numpy
from random import randint
import matplotlib
from pylab import *
import math

from ray import *
from vector3 import *
from hitable import *
	
def col(ray, world):
	hit, hit_record = world.hit(ray, 0, sys.float_info.max)
	if hit:	
		return (hit_record.normal + Vector3(1,1,1)) * 0.5
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

world = HitableList()
world.add(Sphere(Vector3(0,0,-1), 0.5))
world.add(Sphere(Vector3(0,-100.5,-1), 100))

for y in range(rows):
	flippedy = rows - y - 1
	v = flippedy / rows
	for x in range(cols):
		u = x / cols
		r = Ray(origin, lower_left_corner + horizontal * u + vertical * v)
		c = col(r, world)
		image_array[y, x, RED] = c.x
		image_array[y, x, GREEN] = c.y
		image_array[y, x, BLUE] = c.z

imshow(image_array, interpolation='None')
show()
