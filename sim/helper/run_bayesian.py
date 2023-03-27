import os

for i in range(50):
    os.system("cargo run --release --no-default-features --features bayesian")
