import itertools
import os 
import sys

cwd = os.path.realpath(os.path.dirname(__file__))
exec_cmd = os.path.join(os.path.dirname(__file__), "..")
os.system(f"cd {os.path.abspath(exec_cmd)}")
inps_dir = os.path.join(os.path.dirname(__file__),"..", "inputs","tests")
filt = sys.argv[1:]

for file_test in filt[0:]:
   file_p = os.path.abspath(os.path.join(inps_dir, "fire_spread"))
   for inp in itertools.islice(os.listdir(file_p), 1):
      act_file = os.path.join(file_p, inp)
      os.system(f"cargo run --release -- -f {file_test}/{inp}")