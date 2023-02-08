import os 
import sys

cwd = os.path.realpath(os.path.dirname(__file__))
inps_dir = os.path.join(os.path.dirname(__file__),"..", "inputs","tests")
filt = sys.argv[1:]

for file_test in filt:
   file_p = os.path.join(inps_dir, file_test)
   print(os.listdir(file_p))
