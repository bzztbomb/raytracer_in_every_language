import numpy
import random
import matplotlib
from pylab import *
import math

from ray import *
from vector3 import *
from hitable import *
from camera import *
	
def pt_in_unit_sphere():
	while True:
		p = (Vector3(random(), random(), random()) * 2) - one()
		if p.dot(p) < 1.0:
			return p
			
def col(ray, world):
	hit, hit_record = world.hit(ray, 0, sys.float_info.max)
	if hit:	
		 target = hit_record.pt + hit_record.normal + pt_in_unit_sphere()
		 return col(Ray(hit_record.pt, target-hit_record.pt), world) * 0.5
		
	unit_direction = ray.direction.unit()
	t = 0.5 * (unit_direction.y + 1.0)
	return Vector3(1,1,1)*(1-t) + Vector3(0.5, 0.7, 1) * t
	
RED = 0
GREEN = 1
BLUE = 2

rows = 100
cols = 200
num_samples = 30
channels = 3
image_array = numpy.zeros(rows*cols*channels).reshape(rows, cols, channels)

world = HitableList()
world.add(Sphere(Vector3(0,0,-1), 0.5))
world.add(Sphere(Vector3(0,-100.5,-1), 100))

cam =  Camera()

for y in range(rows):
	flippedy = rows - y - 1
	for x in range(cols):
		c = Vector3(0,0,0)
		for ns in range(num_samples):
			u = (x+random()) / cols
			v = (flippedy+random()) / rows
			r = cam.get_ray(u, v)
			c += col(r, world)
		c /= num_samples
		image_array[y, x, RED] = c.x
		image_array[y, x, GREEN] = c.y
		image_array[y, x, BLUE] = c.z

imshow(image_array, interpolation='None')
show()
