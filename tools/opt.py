import os
import subprocess


files = os.listdir()

for file in files:
    if file.endswith(".gltf"):
        subprocess.run(
            [
                "CompressonatorCLI ",
                "-meshopt",
                file,
                file,
            ]
        )
        with open(file, "r") as f:
            data = f.read()
            data = data.replace('"animations":null,', "")

        with open(file, "w") as f:
            f.write(data)
