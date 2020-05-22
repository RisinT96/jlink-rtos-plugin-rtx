# J-Link GDB Server RTOS Plug-In - RTX [![Rust: Build](https://github.com/RisinT96/jlink-rtos-plugin-rtx/workflows/Rust/badge.svg)](https://github.com/RisinT96/jlink-rtos-plugin-rtx/actions?query=workflow%3ARust)[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

RTX RTOS awareness plugin for the J-Link GDB Server.

## Compatibility

### RTOS
- RTX V5 - Tested and seems stable.
- mBed - based on RTX V5, should work - not tested.

### Target
Currently only Cortex M0-M4 targets are supported.

## Requirements

- [SEGGER J-Link Software and Documentation Pack](https://www.segger.com/downloads/jlink#J-LinkSoftwareAndDocumentationPack)

## Installation
1. Download the ZIP from the release.
2. Extract the lib appropriate to your OS into a directory of your choice.
3. Launch the J-Link GDB Server and provide path to the lib: `JLinkGDBServer -rtos path-to-lib`

## Original files

This repository does not include any original SEGGER files.

## License

The original content is released under the [MIT License](https://opensource.org/licenses/MIT), with all rights reserved to [Tal Risin](https://github.com/risint96).

The plug-in client API definitions (the `RTOS_*` functions) and the GDB server API definitions are compatible with the SEGGER GDBServer RTOS Plug-in SDK API definitions.

All IP rights, title and interest in the GDBServer RTOS Plug-in SDK are and shall at all times remain with SEGGER.

```
Copyright (c) 2004-2020 SEGGER Microcontroller GmbH & Co. KG
Internet: www.segger.com        Support:  support@segger.com
```
