# System Information
So far, I have only implemented things which work on linux. I will, at some point, update with some `cfg` stuff, implementing a matching set of features for Windows.

### Windows
[wmic](https://ss64.com/nt/wmic.html) is a great tool for gathering information on windows systems.

https://superuser.com/questions/1234471/how-can-i-easily-get-the-manufacturer-and-model-of-my-monitor
https://superuser.com/questions/1234471/how-can-i-easily-get-the-manufacturer-and-model-of-my-monitor

```sh
wmic desktopmonitor get Caption, MonitorType, MonitorManufacturer, Name
Caption             MonitorManufacturer         MonitorType         Name
Generic PnP Monitor (Standard monitor types)    Generic PnP Monitor Generic PnP Monitor
```

```sh
wmic desktopmonitor get
# Some crazy output
```

https://www.windowscentral.com/how-create-and-run-batch-file-windows-10
```sh
echo "===================="
echo Windows Info
echo "===================="
systeminfo | findstr /c:"OS Name"
systeminfo | findstr /c:"OS Version"
systeminfo | findstr /c:"System Type"
echo "===================="
echo Hardware Info
echo "===================="
systeminfo | findstr /c:"Total Physical Memory"
wmic cpu get name
wmic diskdrive get name, model, size
wmic path win32_videocontroller get name
wmic path win32_VideoController get CurrentHorizontalResolution,CurrentVerticalResolution
echo "===================="
echo Network Info
echo "===================="
ipconfig | findstr IPv4ipconfig | findstr IPv6
```
