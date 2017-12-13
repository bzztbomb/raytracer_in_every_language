from vector3 import *
from ray import *
from util import *
import random

import math

class Camera (object):
	def __init__(self, look_from, look_at, vup, vfov, aspect, aperture, focus_dist, time0, time1):
		theta = vfov*math.pi/180.0
		half_height = math.tan(theta/2.0)
		half_width = aspect * half_height
		
		w = (look_from - look_at).unit()
		u = (vup.cross(w)).unit()
		v = w.cross(u).unit()
		
		#self.lower_left_corner = Vector3(-half_width,-half_height,-1)
		self.lower_left_corner = look_from - half_width * u * focus_dist- half_height * v * focus_dist - w*focus_dist
		self.horizontal = 2*half_width*u*focus_dist
		self.vertical = 2*half_height*v*focus_dist
		self.origin = look_from 
		self.lens_radius = aperture / 2
		self.u = u
		self.v = v
		self.w = w
		self.time0 = time0
		self.time1 = time1
		
	def get_ray(self, u, v):
		rd = self.lens_radius * pt_in_unit_disk()
		offset = self.u * rd.x + self.v*rd.y
		t = self.time0 + (self.time1-self.time0)*random.random()
		return Ray(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset, t)
		

		
