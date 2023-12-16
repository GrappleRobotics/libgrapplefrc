import os
import subprocess
import sys
import zipfile

NEW_PATH = os.getenv("PATH") + ";" + os.path.expanduser("~/.gradle/toolchains/frc/2023/roborio/bin")

# TODO: Load from cargo metadata
VERSION = "2023.0.0-beta3"

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
  ("osxuniversal"): {
    'triple': "x86_64-apple-darwin",
    'path': 'osx/universal'
  },
  ("linuxx86-64"): {
    'triple': "x86_64-unknown-linux-gnu",
    'path': 'linux/x86-64'
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
    files = { 'grapplefrcdriver.dll': 'grapplefrcdriver.dll', 'grapplefrcdriver.dll.lib': 'grapplefrcdriver.lib', 'grapplefrcdriver.pdb': 'grapplefrcdriver.pdb' }
  elif "osx" in platform:
    files = { 'libgrapplefrcdriver.dylib': 'libgrapplefrcdriver.dylib' }
  else:
    files = { 'libgrapplefrcdriver.so': 'libgrapplefrcdriver.so' }

  for (mode, classifier) in [("debug", f"{classifierBase}debug"), ("release", classifierBase)]:
    with zipfile.ZipFile(f"{outdir}/{identifier}-{VERSION}-{classifier}.zip", "w") as zf:
      for (fkey, fname) in files.items():
        zf.write(f"target/{triple}/{mode}/{fkey}", f"{details['path']}/shared/{fname}")
  
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