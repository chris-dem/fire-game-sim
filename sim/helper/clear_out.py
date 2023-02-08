import os
import sys
import shutil

cwd = os.path.realpath(os.path.dirname(__file__))

dir_path = os.path.abspath(os.path.join(os.path.join(cwd, os.pardir, "output")))

print(os.listdir(dir_path))
# for v in os.listdir(dir_path):
#     p = os.path.join(dir_path,v)
#     if os.path.isdir(p):
#         # print(f"From dir {p}")
#         shutil.rmtree(p)
#     elif os.path.isfile(p):
#         # print(f"From file {p}")
#         os.remove(p)