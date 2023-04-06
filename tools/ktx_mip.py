import os
import subprocess

files = os.listdir()

for file in files:
    if file.endswith(".exr"):
        subprocess.run(
            [
                "CompressonatorCLI",
                "-fd",
                "BC6H",
                "-mipsize",
                "256",
                file,
                file.replace(".exr", ".ktx2"),
            ]
        )
