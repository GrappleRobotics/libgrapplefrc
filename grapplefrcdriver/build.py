import os
import subprocess
import sys
import zipfile

NEW_PATH = os.getenv("PATH") + ";" + os.path.expanduser("~/.gradle/toolchains/frc/2025/roborio/bin")

# TODO: Load from cargo metadata
VERSION = "2025.1.0"

def run(*cmd):
  env = os.environ.copy()
  env["PATH"] = NEW_PATH
  sp = subprocess.Popen(cmd, env=env)
  sp.wait()

TRIPLE_LOOKUP = {
  ("linuxathena"): {
    'triple': "arm-unknown-linux-gnueabi",
    'path': 'linux/athena'
  },
  ("windowsx86-64"): {
    'triple': "x86_64-pc-windows-msvc",
    'path': 'windows/x86-64'
  },
  ("windowsarm64"): {
    'triple': "aarch64-pc-windows-msvc",
    'path': 'windows/arm64'
  },
  ("osxuniversal"): {
    'triple': "x86_64-apple-darwin",
    'path': 'osx/universal'
  },
  ("linuxx86-64"): {
    'triple': "x86_64-unknown-linux-gnu",
    'path': 'linux/x86-64'
  },
  ("linuxarm64"): {
    'triple': "aarch64-unknown-linux-gnu",
    'path': 'linux/arm64'
  },
  ("linuxarm32"): {
    'triple': "arm-unknown-linux-gnueabihf",
    'path': 'linux/arm32'
  }
}

def build(platform):
  details = TRIPLE_LOOKUP.get(platform)
  triple = details['triple']

  if triple is None:
    print("No Triple found for {}".format(platform), file=sys.stderr)
    exit(1)

  run("cargo", "build", "--target={}".format(triple))
  run("cargo", "build", "--release", "--target={}".format(triple))

  # OSX builds universal by building both and then merging
  if platform == "osxuniversal":
    run("cargo", "build", "--target=aarch64-apple-darwin")
    run("cargo", "build", "--release", "--target=aarch64-apple-darwin")

    for mode in ["debug", "release"]:
      mode_dir = f"target/universal-apple-darwin/{mode}"
      try:
        os.makedirs(mode_dir)
      except FileExistsError:
        pass
      output_file = f"{mode_dir}/libgrapplefrcdriver.dylib"
      intel_file = f"target/x86_64-apple-darwin/{mode}/libgrapplefrcdriver.dylib"
      arm_file = f"target/aarch64-apple-darwin/{mode}/libgrapplefrcdriver.dylib"
      run("lipo", "-create", "-output", output_file, intel_file, arm_file)
      triple = "universal-apple-darwin"

  # Zip it up for maven
  package = "au/grapplerobotics"
  identifier = "libgrapplefrcdriver"
  classifierBase = platform
  outdir = f"target/zips/{package}/{identifier}/{VERSION}"

  try:
    os.makedirs(outdir)
  except FileExistsError:
    pass

  # Headers first
  with zipfile.ZipFile(f"{outdir}/{identifier}-{VERSION}-headers.zip", "w") as zf:
    for root, dirs, files in os.walk("target/headers"):
      for file in files:
        zf.write(f"{root}/{file}", os.path.relpath(f"{root}/{file}", "target/headers"))

  # Then everything else
  files = {}
  if "windows" in platform:
    files = {
      'shared': { 'grapplefrcdriver.dll': 'grapplefrcdriver.dll', 'grapplefrcdriver.dll.lib': 'grapplefrcdriver.lib', 'grapplefrcdriver.pdb': 'grapplefrcdriver.pdb' },
      'static': { 'grapplefrcdriver.lib': 'grapplefrcdriver.lib' }
    }
  elif "osx" in platform:
    files = {
      'shared': { 'libgrapplefrcdriver.dylib': 'libgrapplefrcdriver.dylib' }
    }
  else:
    files = {
      'shared': { 'libgrapplefrcdriver.so': 'libgrapplefrcdriver.so' },
      'static': { 'libgrapplefrcdriver.a': 'libgrapplefrcdriver.a' }
    }

  for (linkage, linkageClassifier) in [("shared", ""), ("static", "static")]:
    for (mode, classifier) in [("debug", f"{classifierBase}{linkageClassifier}debug"), ("release", f"{classifierBase}{linkageClassifier}")]:
      if linkage in files:
        with zipfile.ZipFile(f"{outdir}/{identifier}-{VERSION}-{classifier}.zip", "w") as zf:
          for (fkey, fname) in files[linkage].items():
            zf.write(f"target/{triple}/{mode}/{fkey}", f"{details['path']}/{linkage}/{fname}")

  # And lastly, the .pom
  with open(f"{outdir}/{identifier}-{VERSION}.pom", "w") as f:
    f.write("""<?xml version="1.0" encoding="UTF-8"?>
<project xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd" xmlns="http://maven.apache.org/POM/4.0.0"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <modelVersion>4.0.0</modelVersion>
  <groupId>{}</groupId>
  <artifactId>{}</artifactId>
  <version>{}</version>
  <packaging>pom</packaging>
</project>""".format(package.replace("/", "."), identifier, VERSION))

build(sys.argv[1])
