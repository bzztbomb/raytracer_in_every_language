import math

class Vector3(object):
	def __init__(self, x, y, z):
		self.data = [x, y, z]

	def __repr__(self):
		return '{self.x}, {self.y}, {self.z}'.format(self=self)

	def __mul__(self, o):
		if isinstance(o, Vector3):
			return Vector3(self.x*o.x,self.y*o.y,self.z*o.z)
		else:
			return Vector3(self.x * o, self.y * o, self.z * o)

	def __rmul__(self, o):
		return self*o

	def __truediv__(self, o):
		return Vector3(self.x / o, self.y / o, self.z / o)

	def __add__(self, o):
		return Vector3(self.x + o.x, self.y + o.y, self.z + o.z)

	def __sub__(self, o):
		return Vector3(self.x - o.x, self.y - o.y, self.z - o.z)

	def __neg__(self):
		return self * -1

	@property
	def x(self):
		return self.data[0]

	@property
	def y(self):
		return self.data[1]

	@property
	def z(self):
		return self.data[2]

	def length(self):
		return math.sqrt(self.x*self.x+self.y*self.y+self.z*self.z)

	def unit(self):
		l = self.length()
		return Vector3(self.x/l, self.y/l, self.z/l)

	def dot(self, o):
		return self.x*o.x + self.y*o.y + self.z*o.z

	def cross(self, o):
		return Vector3(self.y*o.z - self.z*o.y, -(self.x*o.z-self.z*o.x), self.x*o.y-self.y*o.x)

def one():
	return Vector3(1,1,1)

def zero():
	return Vector3(0,0,0)

def reflect(v, n):
	return v - (n*v.dot(n)*2)

def refract(v, n, ni_over_nt):
	uv = v.unit()
	dt = uv.dot(n)
	discr = 1 - ni_over_nt*ni_over_nt * (1-dt*dt)
	if discr > 0:
		return (True, (uv - n * dt) * ni_over_nt - n * math.sqrt(discr))
	else:
		return (False, None)



