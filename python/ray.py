class Ray(object):
	def __init__(self, origin, direction):
		self.origin = origin
		self.direction = direction

	def point_at_t(self, t):
		return self.origin + (self.direction * t)

	def __repr__(self):
		return 'origin: {self.origin}\ndirection: {self.direction}'.format(self=self)

