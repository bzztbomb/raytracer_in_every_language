import math

from vector3 import *
from ray import *

class Camera (object):
	def __init__(self, look_from, look_at, vup, vfov, aspect):
		theta = vfov*math.pi/180.0
		half_height = math.tan(theta/2.0)
		half_width = aspect * half_height

		w = (look_from - look_at).unit()
		u = (vup.cross(w)).unit()
		v = w.cross(u).unit()

		self.lower_left_corner = look_from - half_width*u - half_height*v - w
		self.horizontal = 2*half_width*u
		self.vertical = 2*half_height*v
		self.origin = look_from

	def __repr__(self):
		return 'origin: {self.origin}\nlower_left:{self.lower_left_corner}\nhorz:{self.horizontal}\nvert:{self.vertical}'.format(self=self)

	def get_ray(self, u, v):
		return Ray(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)



