# CPM

Cmake Project Manager - automate cmake build commands, generate CMakeLists files with presets.

## Building From Source

1. Clone repo
2. ``cargo build``
3. ???
4. Profit

## Installation

1. Download the [alpha binaries](https://github.com/Durengo/CPM/releases/tag/alpha)
2. Extract it into your project like <PROJECT_ROOT>/CPM
3. CD into the root of **your** project
4. *Execute the ``cpm.exe init`` from the project as working directory
5. Test by running: ``./cpm -V``

Output:

```batch
❯ ./cpm -V
cpm 1.0.0
```

<p>1* - Depending on the operating system an appropriate batch/shell script will be created. This is the entrypoint to the CPM program from the project root.</p>

## Road to achieving CLI greatness

* Generating CMake files.
* Build specific executables.

## How to use

CPM is divided into modules which expose specific functionality.

The following will display the list of included modules and how to access them:

```batch
❯ ./cpm -h
Usage: cpm.exe [OPTIONS] [COMMAND]

Commands:
  init   Initialize CPM in the current directory
  setup  Setup CPM in the current directory
  build  Build CPM in the current directory
  cache  Manage CPM Cache
  help   Print this message or the help of the given subcommand(s)

Options:
      --no-init       Do not run the 'init' command. This is meant for when the entrypoint is created. Otherwise, the 'i
  -f, --force-reinit  Force settings reinitialization.
                      WARNING: This will overwrite the current settings file. This will break the current state if alrea
  -h, --help          Print help
  -V, --version       Print version
```

| COMMAND |                 INFO                  |
| :-----: | :-----------------------------------: |
|  init   | Prepare internal cache and variables. |
|  setup  |       Access the setup module.        |
|  build  |       Access the build module.        |
|  cache  |        Access the venv module.        |

### Generate

WIP.

### Setup

The setup module provides functionality which fetches the necessary libraries, prerequisites, and post installation setup commands.

Currently only [VCPKG](https://vcpkg.io/en/) is supported, limiting this program to the Windows platform.

Once CPM is initialized for a project, there are 3 choices to continue:

1. You have a global VCPKG install and have added it to the system (environment) path
   * >./cpm setup -a
2. You have a global VCPKG install but have not added it to the system (environment) path
   * >./cpm setup -u "path/to/vcpkg"
3. You do not have a global VCPKG install
   * >./cpm setup -n

The 3rd option will download VCPKG into Vendor/vcpkg in the folder where you have initialized CPM. It should be noted that it is better to have a global VCPKG installation somewhere, because if it gets accidentally deleted it can greatly impact iteration time when building the project. The local VCPKG is provided so that in the future it might be possible to archive artifacts of VCPKG and have extremely fast download/build times for newly instantiated projects or ones that use modified VCPKG libraries.

``-n, --no-toolchain-path`` and ``--no-toolchain-path`` are not implemeneted in 1.0.0 yet.

|                   COMMAND                   |                                                                                                INFO                                                                                                |
| :-----------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
|          -a, --auto-toolchain-path          |                              Tries to find an existing toolchain installation if it is added to the path. Runs the setup. Windows: uses where.exe to find toolchain.                               |
| -u, --use-toolchain-path `<TOOLCHAIN-PATH>` |                                         Runs the setup with the specified vcpkg directory. Must be set to root directory of the toolchain. Runs the setup.                                         |
|           -n, --no-toolchain-path           |  Tries to find an existing toolchain installation if it is added to the path, otherwise attempts to download and setup the toolchain. Runs the setup. Windows: uses where.exe to find toolchain.   |
|                  ~~--spc~~                  | ~~Skips package configuration when running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'. Pass before running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'.~~ |
|                  ~~--ndc~~                  |                ~~Runs the setup without checking for runtime dependencies (ONLY FOR CI USE). Pass before running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'.~~                |
|                  ~~--fpi~~                  |              ~~Forces vcpkg to install packages again (does not remove any existing packages). Pass before running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'.~~              |
|      -p, --platform `<TOOLCHAIN-PATH>`      |                                                            Forces to use specific OS install. Supported OS types: windows, linux, macos                                                            |

### Build

The setup build modules utilizes CMake functionality to generate a project, build it, and install it.

First of all, before even running any build commands you must generate the project at least once.

Once you generate a CMake project, you will want to build it. Before running any of the commands you will need to make sure that you provide a build type - either Release (-r) or Debug (-d). This is mandatory before -g (Generate), -b (Build), and -i (Install) flags.

This module has been made highly scalable and multiple options can be provided.

Simple use scenarios:

1. I want to build the project in Debug.
    > $ ./cpm build -db
2. I want to build the project in Release.
    > $ ./cpm build -rb
3. I want to build the project in Debug and also generate the Install folder.
    > $ ./cpm build -dbi
4. I want to delete the Build and the Install directories.
    > $ ./cpm build -c
5. I want to delete the Build and the Install directories then I want to regenerate and build the project in Debug and also generate the Install folder.
    > $ ./cpm build -c -dg -bi

|                 COMMAND                  |                                                                                                                                 INFO                                                                                                                                  |
| :--------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
|          -d, --debug-build-type          |                                                                                                                       Sets Build Type to Debug.                                                                                                                       |
|         -r, --release-build-type         |                                                                                                                      Sets Build Type to Release.                                                                                                                      |
| -g, --generate-project `[<SYSTEM_TYPE>]` | Generate CMake Project. Will not run without a build type set flag. System types: nt/msvc     ---> Windows, MSVC compiler, unix/clang  ---> Unix, Clang compiler, unix/gcc    ---> Unix, GCC compiler. Provide no option to retrieve last ran cmake generate command. |
|           -b, --build-project            |                                                                                                       Build CMake Project. Automatically uses CMAKE_BUILD_TYPE.                                                                                                       |
|          -i, --install-project           |                                                                                                      Install CMake Project. Automatically uses CMAKE_BUILD_TYPE.                                                                                                      |
| -c, --clean-project `[<WHAT_TO_CLEAN>]`  |                                                                Clean CMake Project WHAT_TO_CLEAN: (Combine characters to clean multiple things), b   ---> Build directory, i   ---> Install directory                                                                 |

### Cache

In the new rust version the cache has been unified and there are no more multiple cache files.
This module allows to change key values but currently it's not really usefull since the cache can be manually edited.

|                COMMAND                 |                                                         INFO                                                          |
| :------------------------------------: | :-------------------------------------------------------------------------------------------------------------------: |
|      -p, --print-cache `[<KEY>]`       | Print the cache. If a key is provided, only that key will be printed. If no key is provided, all keys will be printed |
| -e, --edit-cache-key `<KEY>` `<VALUE>` |                                                 Change a cache value                                                  |
|            -o, --open-cache            |                                       Open the cache file in the default editor                                       |

### Venv

CURRENTLY DEPRECATED.

~~CPM can also manage a venv instance within the instantiated folder. This is still in progress but currently there is simple functionality to initialize, deinitialize, and reinitialize a venv environment.~~

|       COMMAND        |                                       INFO                                        |
| :------------------: | :-------------------------------------------------------------------------------: |
|  initialize (or -i)  |                         Initializes the venv environment.                         |
| deinitialize (or -d) |                        Deinitializes the venv environment.                        |
| reinitialize (or -r) |               Deinitializes then initializes the venv environment.                |
|     force_clean      | Force clean the venv environment. Use this if reinitialize or deinitializes fail. |