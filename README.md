ScreenRuster support for XScreenSaver and XLock hacks
=====================================================
This saver provides support for using the plethora of existing screen savers
from XScreenSaver and XLock as ScreenRuster savers.

Settings
========
If you want the minimal setup you can just add `hacks` in the ScreenRuster
saver `use`, then it will automatically pickup your choices and settings from
the file generated by `xscreensaver-demo`.

Otherwise you can specify various settings from the centralized configuration
file.

`use`
-----
An array containing the names of the hacks you want ScreenRuster to use.

`<hack>`
--------
Specific settings for the hack.

- `<name> = true` will translate to `-<name>`
- `<name> = false` will translate to `-no-<name>`
- `<name> = <value>` will translate to `-<name> <value>`
