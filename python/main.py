import math
import random
import sys

from concurrent.futures import ProcessPoolExecutor, wait

from ray import *
from vector3 import *
from hitable import *
from camera import *
from material import *
from texture import *
from constant_medium import *

RED = 0
GREEN = 1
BLUE = 2

RENDER_MODE = True
ROWS = 500
COLS = 500
NUM_SAMPLES = 5000

#
# Single thread Timings for 100,100,100 real	7m36.600s, user	7m30.134s, sys	0m1.904s
# Thread Pool timings with 4 workers: real	8m9.428s user	8m6.958s sys	0m4.945s
# Process Pool with 4 workers: real	1m56.122s user	7m3.216s sys	0m0.540s
# Process pool nwith 8 workers: real	1m44.566s user	12m15.019s sys	0m0.716s
# Process pool with 7 workers: real	1m56.024s user	11m43.397s sys	0m1.463s

def col(ray, world, depth):
	hit, hit_record = world.hit(ray, 0.001, sys.float_info.max)
	if hit:
		emitted = hit_record.material.emit(hit_record.u, hit_record.v, hit_record.pt)
		scatter, scatterray, atten = hit_record.material.scatter(ray, hit_record)
		if scatter and depth < 50:
			new_c = emitted + (atten * col(scatterray, world, depth+1))
			# print("new_c: {0} atten: {1}".format(new_c, atten))
			return new_c
		else:
			return emitted

	return Vector3(0,0,0)

# image_array = numpy.zeros(ROWS*COLS*channels).reshape(ROWS, COLS, channels)

def scene_simple():
	objects = []
	objects.append(MovingSphere(Vector3(0,0,-1), 0, Vector3(0,0.5,-1), 1, 0.5, Lambert(Color(Vector3(0.8,0.3,0.3)))))
	checker = Checker(Color(Vector3(0.2, 0.2, 0.2)), Color(Vector3(0.7, 0.7, 0.7)))
	objects.append(Sphere(Vector3(0,-100.5,-1), 100, Lambert(checker)))
	objects.append(Sphere(Vector3(1,0,-1), 0.5, Metal(Color(Vector3(0.8, 0.6, 0.2)), 0.3)))
	# objects.append(Sphere(Vector3(-1,0,-1), 0.5, Dieletric(1.5)))
	# objects.append(Sphere(Vector3(-1,1,-1), 0.5, DiffuseLight(Color(Vector3(10, 10, 10)))))
	objects.append(XYRect(-0.5, 0.15, 0.5, 1.5, -1.0, DiffuseLight(Color(Vector3(10, 10, 10)))))

	look_from = Vector3(-2,2,-3)
	look_at = Vector3(0,0,-1)
	b = (look_from - look_at).length()
	cam = Camera(look_from, look_at, Vector3(0,1,0), 40, COLS / ROWS, 0, b, 0, 1)
	return (cam, objects)

def scene_random():
	random.seed(2084)
	objects = []
	objects.append(Sphere(Vector3(0, -1000, 0), 1000, Lambert(Vector3(0.5, 0.5, 0.5))))
	for a in range(-11, 11):
		for b in range(-11, 11):
			choose_mat = random.random()
			radius = 0.2
			center = Vector3(a+0.9*random.random(), radius, b+0.9*random.random())
			offset = Vector3(4, radius, 0)
			if (center - offset).length() > 0.9:
				mat = None
				if choose_mat < 0.8:
					lamRand = lambda: random.random()*random.random()
					mat = Lambert(Color(Vector3(lamRand(), lamRand(), lamRand())))
				elif choose_mat < 0.95:
					metRand = lambda: (1 + random.random()) * 0.5
					mat = Metal(Color(Vector3(metRand(), metRand(), metRand())), random.random() * 0.5)
				else:
					mat = Dieletric(1.5)
				objects.append(Sphere(center, radius, mat))
	objects.append(Sphere(Vector3(0,1,0), 1.0, Dieletric(1.5)))
	objects.append(Sphere(Vector3(-4, 1, 0), 1.0, Lambert(Color(Vector3(0.4, 0.2, 0.1)))))
	objects.append(Sphere(Vector3(4, 1, 0), 1.0, Metal(Color(Vector3(0.7, 0.6, 0.5)), 0.0)))

	look_from = Vector3(13, 2, 3)
	look_at = zero()
	dist_to_focus = 10
	aperture = 0.1
	cam = Camera(look_from, look_at, Vector3(0,1,0), 20, COLS / ROWS, aperture, dist_to_focus, 0, 1)
	return (cam, objects)

def scene_twospheres():
	objects = []
	checker = Lambert(Checker(Color(Vector3(0.2, 0.2, 0.2)), Color(Vector3(0.7, 0.7, 0.1))))
	mat = Lambert(NoiseTexture(4))
	objects.append(Sphere(Vector3(0,-1000,0), 1000, checker))
	objects.append(Sphere(Vector3(0,2,0), 2, mat))
	look_from = Vector3(0, 3, 6)
	look_at = zero()
	dist_to_focus = 10
	aperture = 0.0
	cam = Camera(look_from, look_at, Vector3(0,1,0), 40, COLS / ROWS, aperture, dist_to_focus, 0, 1)
	return (cam, objects)

def scene_globes():
	objects = []
	mat = Lambert(ImageTexture("map.png"))
	objects.append(Sphere(Vector3(0,-1000,0), 1000, mat))
	objects.append(Sphere(Vector3(0,2,0), 2, mat))
	look_from = Vector3(13, 2, 3)
	look_at = zero()
	dist_to_focus = 10
	aperture = 0.0
	cam = Camera(look_from, look_at, Vector3(0,1,0), 40, COLS / ROWS, aperture, dist_to_focus, 0, 1)
	return (cam, objects)

def scene_cornell():
	red = Lambert(Color(Vector3(0.65, 0.05, 0.05)))
	white = Lambert(Color(Vector3(0.73, 0.73, 0.73)))
	green = Lambert(Color(Vector3(0.12, 0.45, 0.15)))
	light = DiffuseLight(Color(Vector3(15, 15, 15)))
	objects = []
	objects.append(FlipNormals(YZRect(0, 0, 555, 555, 555, green)))
	objects.append(YZRect(0, 0, 555, 555, 0, red))
	objects.append(XZRect(213, 227, 343, 332, 554, light))
	objects.append(FlipNormals(XZRect(0, 0, 555, 555, 555, white)))
	objects.append(XZRect(0, 0, 555, 555, 1, white))
	objects.append(FlipNormals(XYRect(0, 0, 555, 555, 555, white)))
	objects.append(Translate(RotateY(Box(Vector3(0,0,0), Vector3(165,165,165), white), -18), Vector3(130, 0, 65)))
	objects.append(Translate(RotateY(Box(Vector3(0,0,0), Vector3(165,330,165), white), 15), Vector3(265, 0, 295)))

	look_from = Vector3(278, 278, -800)
	look_at = Vector3(278, 278, 0)
	dist_to_focus = 10
	aperture = 0.0
	vfov = 40
	cam = Camera(look_from, look_at, Vector3(0,1,0), vfov, COLS / ROWS, aperture, dist_to_focus, 0, 1)

	return (cam, objects)

def scene_cornell_smoke():
	red = Lambert(Color(Vector3(0.65, 0.05, 0.05)))
	white = Lambert(Color(Vector3(0.73, 0.73, 0.73)))
	green = Lambert(Color(Vector3(0.12, 0.45, 0.15)))
	light = DiffuseLight(Color(Vector3(7, 7, 7)))
	objects = []
	objects.append(FlipNormals(YZRect(0, 0, 555, 555, 555, green)))
	objects.append(YZRect(0, 0, 555, 555, 0, red))
	objects.append(XZRect(113, 127, 443, 432, 554, light))
	objects.append(FlipNormals(XZRect(0, 0, 555, 555, 555, white)))
	objects.append(XZRect(0, 0, 555, 555, 1, white))
	objects.append(FlipNormals(XYRect(0, 0, 555, 555, 555, white)))
	b1 = Translate(RotateY(Box(Vector3(0,0,0), Vector3(165,165,165), white), -18), Vector3(130, 0, 65))
	b2 = Translate(RotateY(Box(Vector3(0,0,0), Vector3(165,330,165), white), 15), Vector3(265, 0, 295))
	objects.append(ConstantMedium(b1, 0.01, Color(Vector3(1.0, 1.0, 1.0))))
	objects.append(ConstantMedium(b2, 0.01, Color(Vector3(0.0, 0.0, 0.0))))

	look_from = Vector3(278, 278, -800)
	look_at = Vector3(278, 278, 0)
	dist_to_focus = 10
	aperture = 0.0
	vfov = 40
	cam = Camera(look_from, look_at, Vector3(0,1,0), vfov, COLS / ROWS, aperture, dist_to_focus, 0, 1)

	return (cam, objects)

def scene_final():
	white = Lambert(Color(Vector3(0.73, 0.73, 0.73)))
	ground = Lambert(Color(Vector3(0.48, 0.83, 0.53)))
	boxobjects = []
	numBoxesPerSide = 20
	for i in range(numBoxesPerSide):
		for j in range(numBoxesPerSide):
			w = 100
			x0 = -1000 + i*w
			z0 = -1000 + j*w
			y0 = 0
			x1 = x0 + w
			y1 = 100 * (random.random()+0.01)
			z1 = z0 + w
			boxobjects.append(Box(Vector3(x0,y0,z0), Vector3(x1, y1, z1), ground))
	ground = NodeBvh(boxobjects, 0, 1)
	objects = [ground]
	light = DiffuseLight(Color(Vector3(7,7,7)))
	objects.append(XZRect(123, 147, 423, 412, 554, light))
	center = Vector3(400, 400, 200)
	objects.append(MovingSphere(center, 0, center + Vector3(30, 0, 0), 1, 50, Lambert(Color(Vector3(0.7, 0.3, 0.1)))))
	objects.append(Sphere(Vector3(260, 150, 45), 50, Dieletric(1.5)))
	objects.append(Sphere(Vector3(0, 150, 145), 50, Metal(Color(Vector3(0.8, 0.8, 0.9)), 10.0)))
	boundary = Sphere(Vector3(360, 150, 145), 70,  Dieletric(1.5))
	objects.append(boundary)
	objects.append(ConstantMedium(boundary, 0.2, Color(Vector3(0.2, 0.4, 0.9))))
	boundary2 = Sphere(Vector3(0, 0, 0), 5000, Dieletric(1.5))
	objects.append(ConstantMedium(boundary2, 0.0001, Color(Vector3(1.0, 1.0, 1.0))))
	mat = Lambert(ImageTexture("map.png"))
	objects.append(Sphere(Vector3(400, 200, 400), 100, mat))
	objects.append(Sphere(Vector3(220,280,300), 80, Lambert(NoiseTexture(0.1))))
	boxobjects2 = []
	numBoxes = 1000
	for j in range(numBoxes):
		boxobjects2.append(Sphere(Vector3(165*random.random(), 165*random.random(), 165*random.random()), 10, white))
	objects.append(Translate(RotateY(NodeBvh(boxobjects2, 0, 1), 15), Vector3(-100, 270, 395)))

	look_from = Vector3(478, 278, -600)
	look_at = Vector3(278, 278, 0)
	dist_to_focus = 10
	aperture = 0.0
	vfov = 40
	cam = Camera(look_from, look_at, Vector3(0,1,0), vfov, COLS / ROWS, aperture, dist_to_focus, 0, 1)

	return (cam, objects)

def clamp(c):
	r = math.sqrt(c.x)
	g = math.sqrt(c.y)
	b = math.sqrt(c.z)
	m = max([r,g,b])
	if (m > 1.0):
		r /= m
		g /= m
		b /= m
	return Vector3(r,g,b)

def render(cam, world, y_start, y_finish, index):
	result = []
	for y in range(y_start, y_finish):
		flippedy = ROWS - y - 1
		for x in range(COLS):
			c = Vector3(0,0,0)
			for ns in range(NUM_SAMPLES):
				u = (x+random.random()) / COLS
				v = (flippedy+random.random()) / ROWS
				r = cam.get_ray(u, v)
				c += col(r, world, 0)
			c /= NUM_SAMPLES
			output = clamp(c)

			if (c.data[0] < 0 or c.data[1] < 0 or c.data[2] < 0):
				raise Exception("{0} {1}".format(x, y))
			output = output * 255
			result.append("%d %d %d" % (output.x, output.y, output.z))
	return (index, result)

if __name__ == "__main__":
	gcam, objects = scene_cornell()
	gworld = HitableList(objects)

	# Debug a single pixel
	# print(col(cam.get_ray(50 / COLS, (ROWS - 90) / ROWS), world, 0))
	# exit(-1)

	chunks = 7
	pool = ProcessPoolExecutor(chunks)

	chunk_length = ROWS // chunks
	curr_start = 0
	curr_end = chunk_length

	futures = []
	i = 0
	for chunk in range(chunks):
		futures.append(pool.submit(render, gcam, gworld, curr_start, curr_end, i))
		curr_start += chunk_length
		curr_end += chunk_length
		i += 1
		if (curr_end > ROWS):
			curr_end = ROWS

	chunks_results = wait(futures)
	if RENDER_MODE:
		print("P3\n%d %d\n%d" % (COLS, ROWS, 255))
		results = []
		for chunk in chunks_results:
			for c in chunk:
				results.append(c.result())
				# for line in c.result():
				# 	print(line)
		results.sort(key=lambda x: x[0])
		for i in results:
			for j in i[1]:
				print(j)