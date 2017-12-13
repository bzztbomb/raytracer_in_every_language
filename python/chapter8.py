import random
import math
import sys

from ray import *
from vector3 import *
from hitable import *
from camera import *
from material import *

def col(ray, world, depth):
	hit, hit_record = world.hit(ray, 0.001, sys.float_info.max)
	if hit:
		scatter, scatterray, atten = hit_record.material.scatter(ray, hit_record)
		if scatter and depth < 50:
			return atten * col(scatterray, world, depth+1)
		else:
			return zero()

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

world = HitableList()
objects = [Sphere(Vector3(0,0,-1), 0.5, Lambert(Vector3(0.1,0.2,0.5)))) ,Sphere(Vector3(0,-100.5,-1), 100, Lambert(Vector3(0.5,0.95,0.5)))) ,Sphere(Vector3(1,0,-1), 0.5, Metal(Vector3(0.8, 0.6, 0.2), 0.3))) ,Sphere(Vector3(-1,0,-1), 0.5, Dieletric(1.2))) ,Sphere(Vector3(-1,0,-1), -0.45, Dieletric(1.2)))]

cam =  Camera(Vector3(-2,2,1), Vector3(0,0,-1), Vector3(0,1,0), 20, cols / rows)


print("P3\n%d %d\n%d" % (cols, rows, 255))

for y in range(rows):
	flippedy = rows - y - 1
	for x in range(cols):
		c = Vector3(0,0,0)
		for ns in range(num_samples):
			u = (x+random.random()) / cols
			v = (flippedy+random.random()) / rows
			r = cam.get_ray(u, v)
			c += col(r, world, 0)
		c /= num_samples
		# if (c.x == 0 and c.y == 0 and c.z == 0):
		# 	print ("x: %f y: %f" % (x, flippedy))

		print("%d %d %d" % (math.sqrt(c.x)*255, math.sqrt(c.y)*255, math.sqrt(c.z)*255))

