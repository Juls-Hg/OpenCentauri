# Patched firmware

This section describes the installation process and features of OpenCentauri's patched firmware.

This firmware is based on Elegoo's 1.1.40 firmware, with patches applied to patch out unwanted behaviour and bugs.

If you find any bugs with this firmware, or want to suggest new features, please do so on the [cc-fw-tools](https://github.com/OpenCentauri/cc-fw-tools){target="_blank"} repository. Not everything is possible within the limits of patching existing firmware, specifically large features or overhauls are unattainable.

This project is in a beta state currently, expect some issues!

If you need any support, feel free to [join the discord](https://discord.gg/t6Cft3wNJ3)

If you'd like to support our work, you can do so on [Ko-Fi](https://ko-fi.com/opencentauri) :heart:! If spending money isn't your thing, we also have a [Makerworld page](https://makerworld.com/en/models/1924078-opencentauri-logo#profileId-2064746) where you can throw some boosts towards!

## Features (as of v0.3.0)

- SSH (user: "root", password: "OpenCentauri")
- Developer features (package manager, prevent booting Klipper, etc)
- Bootlogo can be replaced
    - [See available bootlogos on github](https://github.com/OpenCentauri/cc-fw-tools/tree/main/oc-patches/replace-bootlogo-patch){target="_blank"}
- Exhaust fan no longer automatically turns on during a print
    - Note: In OrcaSlicer the default profile still turns on this fan using a M106 P3 command when using PLA.
    - Note: On Elegoo filaments present in OrcaSlicer, the `Exhaust fan` section present on the Cooling tab inside of a Filament profile is respected. If it is enabled, then exhaust fan will turn on at the specified speed.
- Homing position has been set to the front right instead of the front left
- The webui (and other integrations like octoeverywhere and home assistant) now accepts modifications during a print (bug introduced in firmware 1.1.29. This was fixed in firmware 1.1.42 but has been backported to 1.1.40)
- The webui's store button has been removed
- The webui's logo has been replaced with an OpenCentauri logo
- The webui's corner radius (white pixels near the corners) has been fixed
- Z offset can be adjusted while the printer is idle
- Files can be uploaded while the printer is printing
- Filament usage is reported via the API 
- Connectivity checks are blocked (web traffic issue)
- Official OTA updates have been replaced by OpenCentauri OTA updates
- Support for usb ethernet adapters
- New gcode commands: 
    - `M8212` to turn off the chamber light
    - `M8213` to turn on the chamber light
    - `TEMPERATURE_WAIT SENSOR=box MINIMUM=XX` to wait until a certain temperature has been reached in the chamber

## Installation

1. [Download](https://github.com/OpenCentauri/oc-installer/releases/latest/download/oc-installer.zip) the latest OpenCentauri installer release
2. Insert a fat32-formatted thumbdrive into your pc
3. Extract the `install_opencentauri` folder from `oc-installer.zip` onto the thumbdrive
4. Eject your thumbdrive from your pc and insert it into your Centauri Carbon
5. On your Centauri Carbon, navigate to the files tab, then tap the `Usb Drive` menu, then tap the `install_opencentauri` folder
6. Touch hold the `IMPORT_ME_DO_NOT_PRINT` file, then tap `Import`
7. Run `Install OpenCentauri (Online)`
8. Restart when the install process finished

You should now be greeted by the OpenCentauri splash screen :tada:

This new firmware goes well with [modified machine start and end gcode in OrcaSlicer](./modified_start_end_machine_gcode.md). Specifically to center the purge line and to turn on the light of the chamber after starting a print. Installing this is optional but reccomended.

## Update

Either accept OTA updates from the device itself, follow the steps again in the [Installation section](#installation)

## Uninstall

Follow the steps again in the [Installation section](#installation). Instead of running `Install OpenCentauri (Online)`, run `Install Official 1.1.40 (Online)`