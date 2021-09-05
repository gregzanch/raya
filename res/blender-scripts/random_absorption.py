import bpy
from random import random

default_bands = [
    63.0, 
    125.0, 
    250.0, 
    500.0, 
    1000.0, 
    2000.0, 
    4000.0, 
    8000.0
]

default_absorption = [[band, 0.05] for band in default_bands]


def random_absorption():
    return [[band, random() * 0.2] for band in default_bands]

for mat in bpy.data.materials:
    mat['absorption']=random_absorption()