arch = str(input("Which architecture do you have? (x64, x86, arm)"))
import os
if arch == "x64":
  os.system ("cargo build --target-dir .target --target x86_64-unknown-uefi")
elif arch == "x86":
  os.system ('cargo build --target-dir .target --target i686-unknown-uefi')
elif arch == "arm":
  os.system('cargo build --target-dir .target --target aarch64-unknown-uefi')
else:
 print("Cancelled.")