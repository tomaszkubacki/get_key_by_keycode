# Linux (Wayland) console app to display pressed keys

Mostly for educational purposes.

## how to run

```shell

cargo run -- /dev/input/event0

```

## how to find my keyboard event input device

### list all input devices

```shell +exec
cat /proc/bus/input/devices
```

All devices with _kbd_ flag in handlers are keyboard devices.
Find event id in the "H:" row (eg. my keyboard is event4 not event0)
