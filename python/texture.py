import math
import random
import copy
from PIL import Image

from vector3 import *

class Texture(object):
	def value(self, u, v, p):
		return Vector3(0,0,0)

class Color(Texture):
	def __init__(self, color):
		self.color = color

	def value(self, u, v, p):
		return self.color

class Checker(Texture):
	def __init__(self, odd, even):
		self.odd = odd
		self.even = even

	def value(self, u, v, p):
		a = 10
		sines = math.sin(a*p.x)*math.sin(a*p.y)*math.sin(a*p.z)
		if (sines < 0):
			return self.odd.value(u,v,p)
		else:
			return self.even.value(u,v,p)

def trilinear_interp(c, u, v, w):
	accum = 0
	for i in range(2):
		for j in range(2):
			for k in range(2):
				kernel = (i*u + (1-i)*(1-u)) * (j*v+ (1-j)*(1-v)) * (k*w + (1-k)*(1-w))
				accum += c[i][j][k] * kernel
	return accum

def perlin_interp(c, u, v, w):
	uu = u*u*(3-2*u)
	vv = v*v*(3-2*v)
	ww = w*w*(3-2*w)
	accum = 0
	for i in range(2):
		for j in range(2):
			for k in range(2):
				weight_v = Vector3(u-i, v-j, w-k)
				kernel = (i*uu + (1-i)*(1-uu)) * (j*vv+ (1-j)*(1-vv)) * (k*ww+(1-k)*(1-ww))
				accum += c[i][j][k].dot(weight_v) * kernel
	return accum

class Perlin(object):
	# TODO: MAKE ALL OF THIS STATIC
	def __init__(self):
		self.randvec = []
		self.perm_x = []
		self.perm_y = []
		self.perm_z = []
		r = lambda: (random.random()*2) - 1
		for i in range(256):
			self.randvec.append(Vector3(r(), r(), r()).unit())
			self.perm_x.append(i)
			self.perm_y.append(i)
			self.perm_z.append(i)
		random.shuffle(self.perm_x)
		random.shuffle(self.perm_y)
		random.shuffle(self.perm_z)

	def noise(self, pt):
		u = pt.x - math.floor(pt.x)
		v = pt.y - math.floor(pt.y)
		w = pt.z - math.floor(pt.z)
		i = math.floor(pt.x)
		j = math.floor(pt.y)
		k = math.floor(pt.z)
		c = [[[0,0], [0,0]], [[0,0], [0,0]]]
		for di in range(2):
			for dj in range(2):
				for dk in range(2):
					idx = self.perm_x[(i+di) & 255] ^ self.perm_y[(j+dj) & 255] ^ self.perm_z[(k+dk) & 255]
					c[di][dj][dk] = self.randvec[idx]
		return perlin_interp(c, u, v, w)

	def turb(self, pt, depth):
		accum = 0.0
		temp_p = copy.copy(pt)
		weight = 1.0
		for i in range(depth):
			accum += weight * self.noise(temp_p)
			weight *= 0.5
			temp_p *= 2
		return abs(accum)

class NoiseTexture(Texture):
	def __init__(self, scale):
		self.perlin = Perlin()
		self.scale = scale

	def value(self, u, v, p):
		# Straight noise
		# Range is -0.7..0.7, found empirically.. ;/
		# offset = (1.0 / 0.7) * 0.5
		# noise = (self.perlin.noise(self.scale * p) * offset) + offset
		# Straight Turbulence
		# noise = self.perlin.turb(self.scale * p, 7)
		# Sin modulated by turb
		noise = 0.5 * (1 + math.sin(self.scale * p.x + 5 * self.perlin.turb(self.scale * p, 7)))
		return noise * one()

def clamp(val, minimum, maximum):
	return min(maximum, max(val, minimum))

class ImageTexture(Texture):
	def __init__(self, imageFilename):
		self.image = Image.open(imageFilename)

	def value(self, u, v, p):
		i = u*self.image.size[0]
		j = (1-v)*self.image.size[1]-0.001 # Hrmmmmm
		i = clamp(i, 0, self.image.size[0])
		j = clamp(j, 0, self.image.size[1])
		pixel = self.image.getpixel((i, j))
		ret = Vector3(pixel[0], pixel[1], pixel[2])
		ret /= 255.0
		return ret
