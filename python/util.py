import random
from vector3 import *

def pt_in_unit_sphere():
	while True:
		p = (Vector3(random.random(), random.random(), random.random()) * 2) - one()
		if p.dot(p) < 1.0:
			return p
			
def pt_in_unit_disk():
	while True:
		p = (Vector3(random.random(), random.random(), 0) * 2) - Vector3(1,1,0)
		if p.dot(p) < 1.0:
			return p
			
			
