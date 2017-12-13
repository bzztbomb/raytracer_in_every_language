class Ray(object):
	def __init__(self, origin, direction, time):
		self.origin = origin
		self.direction = direction
		self.time = time

	def __repr__(self):
		return "origin: {self.origin} direction: {self.direction} time: {self.time}".format(self=self)

	def point_at_t(self, t):
		return self.origin + (self.direction * t)

