from vector3 import *

class aabb(object):
	def __init__(self, min, max):
		self.min = min
		self.max = max

	def __repr__(self):
		return "min: {self.min}    max: {self.max}".format(self=self)

	def hit(self, ray, tmin, tmax):
		for a in range(3):
			if ray.direction[a] == 0:
				continue
			invD = 1.0 / ray.direction[a]
			t0 = (self.min[a] - ray.origin[a]) * invD
			t1 = (self.max[a] - ray.origin[a]) * invD
			if (invD < 0):
				t0, t1 = t1, t0
			tmin = t0 if t0 > tmin else tmin
			tmax = t1 if t1 < tmax else tmax
			if tmax <= tmin:
				return False
		return True

	@staticmethod
	def surrounding_box(a,b):
		bmin = Vector3(min(a.min.x, b.min.x), min(a.min.y, b.min.y), min(a.min.z, b.min.z))
		bmax = Vector3(max(a.max.x, b.max.x), max(a.max.y, b.max.y), max(a.max.z, b.max.z))
		return aabb(bmin, bmax)

