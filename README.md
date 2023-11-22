# CPM

Cmake Project Manager - automate cmake build commands, generate CMakeLists files with presets.

## Requirements

There are only 2 requirements:

1. [Perl runtime](https://www.perl.org/).
2. [Python runtime](https://www.python.org/).

There are other dependencies for the program, but those are download and setup whenever you initiate CPM for your project.

Note: Perl libraries are installed globally.

## Installation

1. Clone this repository where you are planning to use CPM.
2. CD into the root of **your** project
3. *Execute the following: ``$ perl /path/to/this/repository/init.pl``
4. Test by running: ``$./cpm v``

Output:

```batch
❯ ./cpm v
CPM Version: 0.0.1
```

<p>1* - This will create "cpm.pl" & depending on the OS a corresponding script like "cpm.bat" or "cpm.sh". This is the entrypoint to the CPM program.</p>

## How to use

CPM is divided into modules which expose specific functionality.

The following will display the list of included modules and how to access them:

```batch
❯ ./cpm h
```

|     COMMAND      |             INFO              |
| :--------------: | :---------------------------: |
|   help (or -h)   |      Displays the help.       |
| generate (or -g) | Access the generation module. |
|  setup (or -g)   |   Access the setup module.    |
|  build (or -b)   |   Access the build module.    |
|       venv       |    Access the venv module.    |

### Generate

WIP.

### Setup

The setup module provides functionality which fetches the necessary libraries, prerequisites, and post installation setup commands.

Currently only [VCPKG](https://vcpkg.io/en/) is supported, limiting this program to the Windows platform.

Once CPM is initialized for a project, there are 3 choices to continue:

1. You have a global VCPKG install and have added it to the system (environment) path
   * >$ ./cpm setup -n
2. You have a global VCPKG install but have not added it to the system (environment) path
   * >$ ./cpm setup -l "path/to/vcpkg"
3. You do not have a global VCPKG install
   * >$ ./cpm setup -n

The 3rd option will download VCPKG into Vendor/vcpkg in the folder where you have initialized CPM. It should be noted that it is better to have a global VCPKG installation somewhere, because if it gets accidentally deleted it can greatly impact iteration time when building the project. The local VCPKG is provided so that in the future it might be possible to archive artifacts of VCPKG and have extremely fast download/build times for newly instantiated projects or ones that use modified VCPKG libraries.

|           COMMAND           |                                                                           INFO                                                                           |
| :-------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------: |
|   no_local_vcpkg (or -n)    | Tries to find an existing vcpkg installation otherwise runs the setup without using a local vcpkg installation. Optionally skips package configurations. |
| vcpkg_location STR (or -l)  |                               Runs the setup with the specified vcpkg directory. Optionally skips package configurations.                                |
| skip_package_configurations |                                                   Skips package configuration when running nlv or lv.                                                    |
|        no_deps_check        |                                       Runs the setup without checking for runtime dependencies (ONLY FOR CI USE).                                        |
|    force_package_install    |                                     Forces vcpkg to install packages again (does not remove any existing packages).                                      |

### Build

The setup build modules utilizes CMake functionality to generate a project, build it, and install it.

First of all, before even running any build commands you must generate the project at least once.

Once you generate a CMake project, you will want to build it. Before running any of the commands you will need to make sure that you provide a build type - either Release (-r) or Debug (-d). This is mandatory before -b (Build) and -i (Install) flags.

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
5. I want to delete the Build and the Install directories then I want to build the project in Debug and also generate the Install folder.
    > $ ./cpm build -cdbi

|     COMMAND      |                                                                   INFO                                                                   |
| :--------------: | :--------------------------------------------------------------------------------------------------------------------------------------: |
|  cache_generate  |                                              Generate the cache file - options_cache.json.                                               |
|      cache       |                                                    Display the contents of the cache.                                                    |
|    cache_edit    |                                                             Edit the cache.                                                              |
|    cache_get     |                                                    Get a value from the cache by key.                                                    |
| release (or -r)  |                                                  Prepares script to work with release.                                                   |
|  debug (or -d)   |                                                   Prepares script to work with debug.                                                    |
| project_generate |                                     Generate CMake project. Must provide -r or -d option beforehand.                                     |
|  build (or -b)   |                               Builds the generated CMake Project. Must provide -r or -d option beforehand.                               |
| install (or -i)  |                                       Installs the build. Must provide -r or -d option beforehand.                                       |
| symlink (or -s)  | Creates a symlink to all exes located in the bin. Must provide -r or -d option beforehand. Build must be installed beforehand. **(WIP)** |
|  clean (or -c)   |                                                Cleans the build and install directories.                                                 |
|  clean_rebuild   |                     Cleans the build and install directories and rebuilds. Must provide -r or -d option beforehand.                      |
|    clean_dir     |                                                  Cleans <b>uild or <i>nstall directory.                                                  |

### Venv

CPM can also manage a venv instance within the instantiated folder. This is still in progress but currently there is simple functionality to initialize, deinitialize, and reinitialize a venv environment.

|       COMMAND        |                                       INFO                                        |
| :------------------: | :-------------------------------------------------------------------------------: |
|  initialize (or -i)  |                         Initializes the venv environment.                         |
| deinitialize (or -d) |                        Deinitializes the venv environment.                        |
| reinitialize (or -r) |               Deinitializes then initializes the venv environment.                |
|     force_clean      | Force clean the venv environment. Use this if reinitialize or deinitializes fail. |