#!/usr/bin/env python
import math

class StarShip:
	def __init__(self):
		self.modules = []
		self.total_fuel = 0

	def add_module(self, mass):
		self.modules.append(mass)

	def fuel_for_module(self, mass, recursive):
		mass = mass / 3
		mass = math.floor(mass)
		mass = mass - 2
		if mass < 0:
			return 0 # return zero on negative mass
		if recursive:
			return mass + self.fuel_for_module(mass, recursive)
		else:
			return mass

	def calculate(self, recursive):
		self.total_fuel = 0
		for module in self.modules:
			self.total_fuel += self.fuel_for_module(module, recursive)
		self.total_fuel = int(self.total_fuel)
		return self.total_fuel

starship = StarShip()

try:
	while True:
		mass = int(raw_input())
		starship.add_module(mass)
except:
	starship.calculate(False)
	print("Part 1: {}".format(starship.total_fuel))
	starship.calculate(True)
	print("Part 2: {}".format(starship.total_fuel))