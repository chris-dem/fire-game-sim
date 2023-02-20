# import itertools
import os 
import sys

cwd = os.path.realpath(os.path.dirname(__file__))
exec_cmd = os.path.join(os.path.dirname(__file__), "..")
inps_dir = os.path.join(os.path.dirname(__file__),"..", "inputs","tests")
filt = sys.argv[1:]
clear_dir = os.path.realpath(os.path.join(exec_cmd, "helper", "clear_out.py"))
output_cwd = os.path.join(exec_cmd, "output")

os.system(f"cd {os.path.abspath(exec_cmd)}")
for file_test in filt[0:]:
   file_p = os.path.abspath(os.path.join(inps_dir, "fire_spread"))
   for inp in os.listdir(file_p):
      act_file = os.path.join(file_p, inp)
      # RUN
      cline =f"cargo run --release -- -f {file_test}/{inp}" 
      if os.system(cline):
         raise RuntimeError(f"program {cline} failed")
      full_paths = list(map(lambda  e : os.path.abspath(os.path.join(output_cwd,e)), os.listdir(output_cwd)))
      new_file = min(filter(lambda f : os.path.isdir(f),full_paths))
      (new_file_name,ext) = os.path.splitext(os.path.abspath(os.path.join(output_cwd,inp)))
      new_file_name = f"{new_file_name}_out"
      if new_file_name in full_paths:
         os.system(f"rm -r {new_file_name}")
      os.rename(new_file, new_file_name)
os.system(f"rm -r {output_cwd}/*.log")
      