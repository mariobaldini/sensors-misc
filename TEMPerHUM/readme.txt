



TEMPerHUM - Temperature and Humidity - USB Metallic
http://img.tradera.net/images/094/312230094_8aafa38f-04ab-49e8-9229-ff6773ea659f.jpg

DMESG output:
[ 4475.965692] usb 1-2: new low-speed USB device number 10 using xhci_hcd
[ 4476.097134] usb 1-2: New USB device found, idVendor=0c45, idProduct=7402, bcdDevice= 0.01
[ 4476.097137] usb 1-2: New USB device strings: Mfr=1, Product=2, SerialNumber=0
[ 4476.097139] usb 1-2: Product: TEMPERHUM1V1.0
[ 4476.097140] usb 1-2: Manufacturer: RDing
[ 4476.103192] input: RDing TEMPERHUM1V1.0 as /devices/pci0000:00/0000:00:14.0/usb1/1-2/1-2:1.0/0003:0C45:7402.0008/input/input20
[ 4476.156028] hid-generic 0003:0C45:7402.0008: input,hidraw1: USB HID v1.10 Keyboard [RDing TEMPERHUM1V1.0] on usb-0000:00:14.0-2/input0
[ 4476.163001] hid-generic 0003:0C45:7402.0009: hiddev96,hidraw2: USB HID v1.10 Device [RDing TEMPERHUM1V1.0] on usb-0000:00:14.0-2/input1


References:

https://github.com/shakemid/pcsensor-temper
Build and reading works, but data is uncalibrated




x86_64 build instructions @ Fedora 31

