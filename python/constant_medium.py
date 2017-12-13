import sys
import math
import random

from hitable import *
from material import *
from util import *

class Isotropic(Material):
	def __init__(self, albedo):
		self.albedo = albedo

	def scatter(self, ray_in, hit_record):
		target = hit_record.pt + hit_record.normal + pt_in_unit_sphere()
		scattered = Ray(hit_record.pt, pt_in_unit_sphere(), ray_in.time)
		return (True, scattered, self.albedo.value(hit_record.u, hit_record.v, hit_record.pt))

class ConstantMedium(Hitable):
	def __init__(self, boundary, density, phaseTexture):
		self.boundary = boundary
		self.density = density
		self.material = Isotropic(phaseTexture)

	def hit(self, ray, tmin, tmax):
		hit1, rec1 = self.boundary.hit(ray, -sys.float_info.max, sys.float_info.max)
		if hit1:
			hit2, rec2 = self.boundary.hit(ray, rec1.t + 0.0001, sys.float_info.max)
			if hit2:
				rec1.t = max(rec1.t, tmin)
				rec2.t = min(rec2.t, tmax)
				if rec1.t < 0:
					return (False, None)
				distInsideBoundary = (rec2.t - rec1.t) * ray.direction.length()
				hitDistance = -(1.0 / self.density)*math.log(random.random())
				if hitDistance < distInsideBoundary:
					t = rec1.t + hitDistance / ray.direction.length()
					pt = ray.point_at_t(t)
					return (True, HitRecord(t, pt, Vector3(0,1,0), 0, 0, self.material))
		return (False, None)

	def bounding_box(self, sceneTime0, sceneTime1):
		return self.boundary.bounding_box(sceneTime0, sceneTime1)
