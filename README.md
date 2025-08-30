# Linux (Wayland) console app to display pressed keys

Mostly for educational purposes.

## how to find my keyboard input device

### list all input devices

```shell +exec
cat /proc/bus/input/devices
```

all devices with _kbd_ flag in handlers are keyboard devices
