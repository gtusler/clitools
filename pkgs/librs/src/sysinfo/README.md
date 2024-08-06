# System Information
So far, I have only implemented things which work on linux. I will, at some point, update with some `cfg` stuff, implementing a matching set of features for Windows.

### Windows
[wmic](https://ss64.com/nt/wmic.html) is a great tool for gathering information on windows systems.

```sh
wmic desktopmonitor get Caption, MonitorType, MonitorManufacturer, Name
Caption             MonitorManufacturer         MonitorType         Name
Generic PnP Monitor (Standard monitor types)    Generic PnP Monitor Generic PnP Monitor
```

```sh
wmic desktopmonitor get
# Some crazy output
```
