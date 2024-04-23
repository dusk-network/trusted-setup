Using the `compute` executable compiled from https://github.com/dusk-network/powersoftau.git at commit `588b70a`

Device:
 + OS - Pop!_OS 22.04 LTS
 + Kernel - 6.6.10-76060610-generic
 + CPU - Intel® Celeron(R) J4005

Entropy:
 + `base64 /dev/random | head -c 100000 | egrep -ao "\w" | tr -d '\n' > /mnt/ramdisk/random` - Write some UTF-8 randomness from /dev/random to a ram disk, run process, unmount ram disk, reboot.

Computation:
 + `cat /mnt/ramdisk/random | ./compute`

Computation Time:
 + 2h 30m

Response Hash:
 + `2d27da97 e3130413 bb81c68a 3eef7082 ab2bf273 09fe880b 8c9ba1bf 3012348d eb84863c 3ef50694 a8f7a5e4 9ea96c12 ab1175ce 3806ec0b f5c574d8 06327630`