from vector3 import *
from util import *
from ray import *
import random

class Material (object):
	def scatter(self, ray_in, hit_record):
		return (False, zero(), zero())

	def emit(self, u, v, p):
		return zero()

class Lambert (Material):
	def __init__(self, albedo):
		self.albedo = albedo

	def scatter(self, ray_in, hit_record):
		target = hit_record.pt + hit_record.normal + pt_in_unit_sphere()
		scattered = Ray(hit_record.pt, target - hit_record.pt, ray_in.time)
		return (True, scattered, self.albedo.value(hit_record.u, hit_record.v, hit_record.pt))

class Metal(Material):
	def __init__(self, albedo, fuzz):
		self.albedo = albedo
		self.fuzz = fuzz

	def scatter(self, ray_in, hit_record):
		r = reflect(ray_in.direction.unit(), hit_record.normal)
		ray = Ray(hit_record.pt, r + pt_in_unit_sphere() * self.fuzz, ray_in.time)
		return (ray.direction.dot(hit_record.normal) > 0, ray, self.albedo.value(hit_record.u, hit_record.v, hit_record.pt))

def schlick(cosine, ref_index):
	r0 = (1-ref_index) / (1+ref_index)
	r0 = r0*r0
	return r0 + (1-r0)*math.pow(1-cosine, 0.5)

class Dieletric(Material):
	def __init__(self, ref_index):
		self.ref_index = ref_index

	def scatter(self, ray_in, hit_record):
		outward_normal = None
		reflected = reflect(ray_in.direction, hit_record.normal)
		ni_over_nt = None
		attenuation = Vector3(1,1,1)
		cosine = None
		bob = ray_in.direction.unit().dot(hit_record.normal)
		if ray_in.direction.dot(hit_record.normal) > 0:
			outward_normal = -hit_record.normal
			ni_over_nt = self.ref_index
			cosine = ray_in.direction.dot(hit_record.normal) / ray_in.direction.length()
			g = 1 - self.ref_index*self.ref_index*(1-cosine*cosine)
			cosine = math.sqrt(g) if g > 0 else 0
		else:
			outward_normal = hit_record.normal
			ni_over_nt = 1 / self.ref_index
			cosine = -ray_in.direction.unit().dot(hit_record.normal.unit())
		refracted, rdir = refract(ray_in.direction, outward_normal, ni_over_nt)
		reflect_prob = schlick(cosine, self.ref_index) if refracted else 1.0
		if random.random() > reflect_prob:
			return (True, Ray(hit_record.pt, rdir, ray_in.time), attenuation)
		else:
			return (True, Ray(hit_record.pt, reflected, ray_in.time), attenuation)

class DiffuseLight(Material):
	def __init__(self, tex):
		self.texture = tex

	def scatter(self, ray_in, hit_record):
		return (False, zero(), zero())

	def emit(self, u, v, p):
		return self.texture.value(u, v, p)
