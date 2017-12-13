import numpy
from random import randint
import matplotlib
from pylab import *

class Ray(object):
	def __unit__(self, origin, direction):
		self.origin = origin
		self.direction = direction
		
	@property
	def origin(self):
		return self.origin
		
	@property
	def direction(self):
		return self.direction
		
	def point_at_t(self, t):
		return self.origin + (self.direction * t)
		
def vec(values):
	return numpy.array(values, ndmin=2)
		
RED = 0
GREEN = 1
BLUE = 2

rows = 100
cols = 200
channels = 3
image_array = numpy.zeros(rows*cols*channels).reshape(rows, cols, channels)

for y in range(rows):
	flippedy = rows - y - 1
	for x in range(cols):
		image_array[flippedy, x, RED] = x / cols
		image_array[flippedy, x, GREEN] = y / rows
		image_array[flippedy, x, BLUE] = 0.2

imshow(image_array, interpolation='None')
show()
