# karlos
Repository for Karl OS

---

## ⚠️ Disclaimers
### Hardware
This project is still considered as **"UNSTABLE"**.
Do **NOT** run Karl OS on a real hardware.
### Bug & Glitch
This project is still under-development, you should be expecting glitch and bug
### Certification
Only official and non-unstable **release ISOs** will be signed by the project. If you chose the cross-compile it by yourself, you must **sign all the binary**, or **disable "Secure Boot"** in BIOS.

---
## ℹ️ Info
- This project is targeting x86_64 and only using UEFI currently, no BIOS booting support at **ALL**
- The kernel could panic, which will literally halt the CPU
- There are barely some checking in the bootloader, this could lead to UB
---
## Building from source
``` bash
git clone https://github.com/imkarl1229/karlos.git
cd karlos
```

### From Docker
``` bash
make build
```

### From Native
``` bash
make native-build
```
### Signing Bootloader
``` bash
# From docker
make sign-bl
# From native
make native-sign-bl
```

---

## License
This project is licensed under the [GPLv3 License](LICENSE).