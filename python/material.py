from vector3 import *
from util import *
from ray import *
import random

class Material (object):
	def scatter(self, ray_in, hit_record):
		return (False, zero(), zero())

class Lambert (Material):
	def __init__(self, albedo):
		self.albedo = albedo

	def scatter(self, ray_in, hit_record):
		target = hit_record.pt + hit_record.normal + pt_in_unit_sphere()
		scattered = Ray(hit_record.pt, target - hit_record.pt)
		return (True, scattered, self.albedo)

class Metal(Material):
	def __init__(self, albedo, fuzz):
		self.albedo = albedo
		self.fuzz = fuzz

	def scatter(self, ray_in, hit_record):
		r = reflect(ray_in.direction.unit(), hit_record.normal)
		ray = Ray(hit_record.pt, r + pt_in_unit_sphere() * self.fuzz)
		return (ray.direction.dot(hit_record.normal) > 0, ray, self.albedo)

def schlick(cosine, ref_index):
	r0 = (1-ref_index) / (1+ref_index)
	r0 = r0*r0
	return r0 + (1-r0)*math.pow(1-cosine, 0.5)

class Dieletric(Material):
	def __init__(self, ref_index):
		self.ref_index = ref_index

	def scatter(self, ray_in, hit_record):
		# print("in scatter!")
		# print(ray_in)
		# print(hit_record)
		outward_normal = None
		ni_over_nt = None
		cosine = None
		if ray_in.direction.dot(hit_record.normal) > 0:
			outward_normal = -hit_record.normal
			# print(outward_normal)
			ni_over_nt = self.ref_index
			# print(ni_over_nt)
			cosine = ray_in.direction.dot(hit_record.normal) / ray_in.direction.length()
			g = 1.0 - self.ref_index*self.ref_index*(1.0-cosine*cosine)
			cosine = math.sqrt(g) if g > 0 else -99
			# print(g)
			# cosine = math.sqrt(g)
		else:
			outward_normal = hit_record.normal
			ni_over_nt = 1 / self.ref_index
			cosine = -ray_in.direction.dot(hit_record.normal) / ray_in.direction.length()
		refracted, rdir = refract(ray_in.direction, outward_normal, ni_over_nt)
		if (refracted and cosine == -99):
			raise Exception("FUCK")
		reflect_prob = schlick(cosine, self.ref_index) if refracted else 1.0
		attenuation = Vector3(1,1,1)
		if random.random() > reflect_prob:
			# 		print("refracted: " + rdir)
			return (True, Ray(hit_record.pt, rdir), attenuation)
		else:
			reflected = reflect(ray_in.direction, hit_record.normal)
			# print("reflected: " + str(reflected))
			return (True, Ray(hit_record.pt, reflected), attenuation)


