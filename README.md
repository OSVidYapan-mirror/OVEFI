# OVEFI
Skip over to README-EN to see Enligsh description
___
## README-TR
Bu benim yaptığım UEFI için olan işletim sistemim. (BOOTX64.efi dosaysı)

### Nasıl derlenir
build.py dosaysını bu komutla çalıştır.

'pyhton3 build.py'

### Nasıl önyüklenir
Sadece bunu FAT32 olan bir USB'de EFI/BOOT/BOOTX64.efi olarak koyup güvenli önyüklemeyi devre dışı bırakmanız yeterli.

___
## README-EN

This is my operating system for UEFI written in rust (Don't be too obsessed this is a BOOTX64.efi file)

### How to build?
Run the build.py file with this command

'pyhton3 build.py'

### How to boot from it???
Just put this file on EFI/BOOT/BOOTX64.efi on a FAT32 USB and disable secure boot.