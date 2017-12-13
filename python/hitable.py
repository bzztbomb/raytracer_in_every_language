from ray import *
from vector3 import *
from math import *

class HitRecord (object):

	def __init__(self, t, pt, normal, material):
		self.t = t
		self.pt = pt
		self.normal = normal
		self.material = material

	def __repr__(self):
		return 't: {self.t}\npoint: {self.pt}\nnormal: {self.normal}'.format(self=self)

class Hitable (object):
	# returns a tuple of true/false and a hitrecord
	def hit(self, ray, tmin, tmax):
		return (False, None)

class HitableList (Hitable):
	def __init__(self):
		self.list = []

	def add(self, hitable):
		self.list.append(hitable)

	def hit(self, ray, tmin, tmax):
		hit_anything = False
		closest = tmax
		rec = None
		for o in self.list:
			hit, hitrecord = o.hit(ray, tmin, closest)
			if hit:
				hit_anything = True
				rec = hitrecord
				closest = hitrecord.t
		return (hit_anything, rec)

class Sphere (Hitable):
	def __init__(self, center, radius, material):
		self.center = center
		self.radius = radius
		self.material = material

	def hit(self, ray, tmin, tmax):
		# print("hit called")
		oc = ray.origin - self.center
		a = ray.direction.dot(ray.direction)
		b = ray.direction.dot(oc)
		c = oc.dot(oc) - self.radius*self.radius
		discr = b*b - a*c
		if discr > 0:
			sq = math.sqrt(discr)
			t = (-b - sq) / a
			if t > tmin and t < tmax:
				pt = ray.point_at_t(t)
				normal = (pt - self.center) / self.radius
				# print("case0")
				# print("t: " + str(t))
				# print("pt: " + str(pt))
				# print("normal: " + str(normal))
				# print("radius" + str(self.radius)
				return (True, HitRecord(t, pt, normal, self.material))
			t = (-b + sq) / a
			if t > tmin and t < tmax:
				# print("case1")
				# print("t: " + str(t))
				pt = ray.point_at_t(t)
				normal = (pt - self.center) / self.radius
				return (True, HitRecord(t, pt, normal, self.material))
		return (False, None)





