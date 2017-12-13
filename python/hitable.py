from ray import *
from vector3 import *
from math import *
from aabb import *

import random
import sys

class HitRecord (object):
	def __init__(self, t, pt, normal, u, v, material):
		self.t = t
		self.pt = pt
		self.normal = normal
		self.material = material
		self.u = u
		self.v = v

	def __repr__(self):
		return "t: {self.t}, pt: {self.pt} normal: {self.normal}".format(self=self)

class Hitable (object):
	# returns a tuple of true/false and a hitrecord
	def hit(self, ray, tmin, tmax):
		return (False, None)

	def bounding_box(self, t0, t1):
		return (False, None)

# Scene data structures
class HitableList (Hitable):
	def __init__(self, objects):
		self.list = objects

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

class NodeBvh(Hitable):
	def __init__(self, hitablelist, time0, time1):
		# We have hit the leaves
		if len(hitablelist) < 3:
			self.left = hitablelist[0]
			self.right = hitablelist[1 if len(hitablelist) == 2 else 0]
			self.box = aabb.surrounding_box(self.left.bounding_box(time0, time1)[1], self.right.bounding_box(time0, time1)[1])
			return
		# Sort and divide the list
		axis = random.randint(0,2)
		hitablelist.sort(key = lambda x: x.bounding_box(time0, time1)[1].min.data[axis])
		middleIndex = len(hitablelist)//2

		self.left = NodeBvh(hitablelist[:middleIndex], time0, time1)
		self.right = NodeBvh(hitablelist[middleIndex:], time0, time1)
		hasBoxL, boxL = self.left.bounding_box(time0, time1)
		hasBoxR, boxR = self.right.bounding_box(time0, time1)
		if not (hasBoxL and hasBoxR):
			raise Exception("Deal with boundless objects! Like planes")
		self.box = aabb.surrounding_box(boxL, boxR)

	def hit(self, ray, tmin, tmax):
		if (not self.box.hit(ray, tmin, tmax)):
			return (False, None)
		lefthit, l = self.left.hit(ray, tmin, tmax)
		righthit, r = self.right.hit(ray, tmin, tmax)
		if (lefthit == righthit):
			if not lefthit:
				return (False, None)
			else:
				closer = l if l.t < r.t else r
				return (True, closer)
		if righthit:
			return (True, r)
		if lefthit:
			return (True, l)

	def bounding_box(self, t0, t1):
		return (True, self.box)

	def __repr__(self):
		return self.print("")

	def print(self, prefix):
		ret = "{0}bvh box: {1} {2}\n".format(prefix, self.box, hex(id(self)))
		lstring = self.left.print(prefix + "  ") if isinstance(self.left, NodeBvh) else self.left
		rstring = self.right.print(prefix + "  ") if isinstance(self.right, NodeBvh) else self.right
		ret += "{0}  left: {1}\n".format(prefix, lstring)
		ret += "{0}  right: {1}\n".format(prefix, rstring)
		return ret

# Transformers
class FlipNormals(Hitable):
	def __init__(self, hitable):
		self.hitable = hitable

	def hit(self, ray, tmin, tmax):
		hit, hr = self.hitable.hit(ray, tmin, tmax)
		if hit:
			hr.normal *= -1
			return (True, hr)
		else:
			return (False, None)

	def bounding_box(self, t0, t1):
		return self.hitable.bounding_box(t0, t1)

class Translate(Hitable):
	def __init__(self, hitable, offset):
		self.hitable = hitable
		self.offset = offset

	def hit(self, ray, tmin, tmax):
		rayMoved = Ray(ray.origin - self.offset, ray.direction, ray.time)
		hit, hr = self.hitable.hit(rayMoved, tmin, tmax)
		if (hit):
			hr.pt += self.offset
			return (True, hr)
		else:
			return (False, None)

	def bounding_box(self, sceneTime0, sceneTime1):
		hasBox, box = self.hitable.bounding_box(sceneTime0, sceneTime1)
		if (hasBox):
			box.min += self.offset
			box.max += self.offset
			return (True, box)
		else:
			return (False, None)

class RotateY(Hitable):
	def __init__(self, hitable, angle):
		self.hitable = hitable
		radians = (math.pi / 180.0) * angle
		self.sinTheta = math.sin(radians)
		self.cosTheta = math.cos(radians)
		hasBox, box = hitable.bounding_box(0, 1) # Not the best..
		m = sys.float_info.max
		minb = Vector3(m, m, m)
		maxb = -minb;
		if (hasBox):
			for i in range(2):
				for j in range(2):
					for k in range(2):
						x = i * box.max.x + (1-i) * box.min.x
						y = i * box.max.y + (1-j) * box.min.y
						z = i * box.max.z + (1-k) * box.min.z
						newx = self.cosTheta*x + self.sinTheta*z
						newz = -self.sinTheta*x + self.cosTheta*z
						tester = Vector3(newx, y, newz)
						for c in range(3):
							minb.data[c] = min(minb.data[c], tester.data[c])
							maxb.data[c] = max(maxb.data[c], tester.data[c])
			self.box = aabb(minb, maxb)
		else:
			self.box = None

	def rotateVector3(self, v):
		return Vector3(self.cosTheta * v.x - self.sinTheta * v.z, v.y, self.sinTheta * v.x + self.cosTheta * v.z)

	def inverseRotateVector3(self, v):
		return Vector3(self.cosTheta * v.x + self.sinTheta * v.z, v.y, -self.sinTheta * v.x + self.cosTheta * v.z)

	def hit(self, ray, tmin, tmax):
		origin = self.rotateVector3(ray.origin)
		direction = self.rotateVector3(ray.direction)
		rotatedRay = Ray(origin, direction, ray.time)
		hit, hr = self.hitable.hit(rotatedRay, tmin, tmax)
		if hit:
			hr.pt = self.inverseRotateVector3(hr.pt)
			hr.normal = self.inverseRotateVector3(hr.normal)
			return (True, hr)
		else:
			return (False, None)

	def bounding_box(self, sceneTime0, sceneTime1):
		return (True if self.box != None else False, self.box)


# Objects
class Sphere (Hitable):
	def __init__(self, center, radius, material):
		self.center = center
		self.radius = radius
		self.material = material

	def current_center(self, t):
		return self.center

	def hit(self, ray, tmin, tmax):
		oc = ray.origin - self.current_center(ray.time)
		a = ray.direction.dot(ray.direction)
		b = ray.direction.dot(oc)
		c = oc.dot(oc) - self.radius*self.radius
		discr = b*b - a*c
		if discr > 0:
			sq = math.sqrt(discr)
			t = (-b - sq) / a
			t = t if t > tmin and t < tmax else (-b + sq) / a
			if t > tmin and t < tmax:
				pt = ray.point_at_t(t)
				normal = (pt - self.current_center(ray.time)) / self.radius
				phi = math.atan2(normal.z, normal.x)
				theta = math.asin(normal.y)
				u = 1-(phi+math.pi) / (2.0*math.pi)
				v = (theta + math.pi*0.5) / math.pi
				return (True, HitRecord(t, pt, normal, u, v, self.material))
		return (False, None)

	def bounding_box(self, t0, t1):
		sz = one() * self.radius
		return (True, aabb(self.center - sz, self.center + sz))

class MovingSphere(Sphere):
	def __init__(self, center0, time0, center1, time1, radius, material):
		super().__init__(center0, radius, material)
		self.center0 = center0
		self.time0 = time0
		self.center1 = center1
		self.time1 = time1

	def current_center(self, t):
		return self.center0 + ((t - self.time0) / (self.time1 - self.time0))*(self.center1-self.center0)

class AARect(Hitable):
	# a/b index is the index into Vector3.data
	def __init__(self, aIndex, bIndex, cIndex, a0, b0, a1, b1, c, material):
		self.aIndex = aIndex
		self.bIndex = bIndex
		self.cIndex = cIndex
		self.a0 = a0
		self.b0 = b0
		self.a1 = a1
		self.b1 = b1
		self.c = c
		self.material = material
		self.aRange = a1 - a0
		self.bRange = b1 - b0

	def hit(self, ray, tmin, tmax):
		if ray.direction.data[self.cIndex] == 0:
			# print("case 1")
			return (False, None)
		t = (self.c - ray.origin.data[self.cIndex]) / ray.direction.data[self.cIndex]
		if (t < tmin or t > tmax):
			return (False, None)
		a = ray.origin.data[self.aIndex] + t*ray.direction.data[self.aIndex]
		if (a < self.a0 or a > self.a1):
			# print("case 2 {0} {1} {2}".format(self.a0, self.a1, a))
			return (False, None)
		b = ray.origin.data[self.bIndex] + t*ray.direction.data[self.bIndex]
		if (b < self.b0 or b > self.b1):
			# print(t)
			# print("case 3 {0} {1} {2}".format(self.b0, self.b1, b))
			return (False, None)
		u = (a - self.a0) / self.aRange
		v = (b - self.b0) / self.bRange
		pt = ray.point_at_t(t)
		normal = zero()
		normal.data[self.cIndex] = 1
		# print("hit")
		return (True, HitRecord(t, pt, normal, u, v, self.material))

	def bounding_box(self, t0, t1):
		eplison = 0.0001
		bmin = zero()
		bmin.data[self.aIndex] = self.a0
		bmin.data[self.bIndex] = self.b0
		bmin.data[self.cIndex] = self.c - eplison
		bmax = zero()
		bmax.data[self.aIndex] = self.a1
		bmax.data[self.bIndex] = self.b1
		bmax.data[self.cIndex] = self.c + eplison
		return (True, aabb(bmin, bmax))

class XYRect(AARect):
	def __init__(self, x0, y0, x1, y1, k, material):
		AARect.__init__(self, 0, 1, 2, x0, y0, x1, y1, k, material)

class XZRect(AARect):
	def __init__(self, x0, z0, x1, z1, k, material):
		AARect.__init__(self, 0, 2, 1, x0, z0, x1, z1, k, material)

class YZRect(AARect):
	def __init__(self, y0, z0, y1, z1, k, material):
		AARect.__init__(self, 1, 2, 0, y0, z0, y1, z1, k, material)

class Box(Hitable):
	def __init__(self, p0, p1, material):
		self.box = aabb(p0, p1)
		objects = []
		objects.append(XYRect(p0.x, p0.y, p1.x, p1.y, p1.z, material))
		objects.append(FlipNormals(XYRect(p0.x, p0.y, p1.x, p1.y, p0.z, material)))
		objects.append(XZRect(p0.x, p0.z, p1.x, p1.z, p1.y, material))
		objects.append(FlipNormals(XZRect(p0.x, p0.z, p1.x, p1.z, p0.y, material)))
		objects.append(YZRect(p0.y, p0.z, p1.y, p1.z, p1.x, material))
		objects.append(FlipNormals(YZRect(p0.y, p0.z, p1.y, p1.z, p0.x, material)))
		self.faces = HitableList(objects)

	def hit(self, ray, tmin, tmax):
		return self.faces.hit(ray, tmin, tmax)

	def bounding_box(self, t0, t1):
		return (True, self.box)

if __name__ == "__main__":
	rect = XZRect(0, 0, 555, 555, 0, None)
	ray = Ray(Vector3(100, 4, 100), Vector3(0, -1, 0), 0)
	print(rect.hit(ray, 0, 1))

	rect = XZRect(213, 227, 343, 332, 554, None)
	ray = Ray(Vector3(278, 278, -800), Vector3(0, 2.5477916398634193, 10), 0)
	print(rect.hit(ray, 0, 1))

	rect = FlipNormals(XZRect(213, 227, 343, 332, 554, None))
	ray = Ray(Vector3(278, 278, -800), Vector3(0, 2.5477916398634193, 10), 0)
	print(rect.hit(ray, 0, 1))
