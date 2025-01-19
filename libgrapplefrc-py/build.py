import os
import subprocess
import sys

# Note: you must run python -m venv .env first!

NEW_PATH = os.getenv("PATH") + ";" + os.path.expanduser("~/.gradle/toolchains/frc/2025/roborio/bin")

# TODO: Load from cargo metadata
VERSION = "2025.0.5"

def run(*cmd):
  env = os.environ.copy()
  env["PATH"] = NEW_PATH
  sp = subprocess.Popen(cmd, env=env)
  sp.wait()

TRIPLE_LOOKUP = {
  ("linuxathena"): {
    'triple': "arm-unknown-linux-gnueabi",
    'python': 'python3.10'
  },
  ("windowsx86-64"): {
    'triple': "x86_64-pc-windows-msvc",
    'python': 'python'  # Windows doesn't understand python3.11 when converting to a linkfile :(
  },
  ("windowsarm64"): {
    'triple': "aarch64-pc-windows-msvc",
    'python': 'python'
  },
  ("osxuniversal"): {
    'triple': "x86_64-apple-darwin",
    'python': 'python3.10'
  },
  ("linuxx86-64"): {
    'triple': "x86_64-unknown-linux-gnu",
    'python': 'python3.10'
  },
  ("linuxarm64"): {
    'triple': "aarch64-unknown-linux-gnu",
    'python': 'python3.10'
  },
  ("linuxarm32"): {
    'triple': "arm-unknown-linux-gnueabihf",
    'python': 'python3.10'
  }
}

def build(platform):
  details = TRIPLE_LOOKUP.get(platform)
  triple = details['triple']

  if triple is None:
    print("No Triple found for {}".format(platform), file=sys.stderr)
    exit(1)

  run("maturin", "build", "--release", "--target={}".format(triple), "-i", details["python"])

build(sys.argv[1])
