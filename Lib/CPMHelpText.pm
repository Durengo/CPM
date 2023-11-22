#!/usr/bin/perl
package CPMHelpText;

use strict;
use warnings;

sub setup_help {
    return qq{
    Usage: $0 setup <verb> <args> ...

    \t# Tries to find an existing vcpkg installation otherwise runs the setup without using a local vcpkg installation. Optionally skips package configurations.
    \t<n|nlv|no-local-vcpkg>

    \t# Runs the setup with the specified vcpkg directory. Optionally skips package configurations.
    \t<l|vl|vcpkg-location> [<path/to/vcpkg>] [spc|skip_package_configurations]

    \t# Skips package configuration when running nlv or lv.
    \t<spc|skip_package_configurations>

    \t# Runs the setup without checking for runtime dependencies (ONLY FOR CI USE).
    \t<ndc|no-deps-check>

    \t# Forces vcpkg to install packages again (does not remove any existing packages).
    \t<fpi|force_package_install>

};
}

sub build_help {
    return qq{
    Usage: $0 build <verb> <args> ...
    
    \t# Displays the help.
    \t<h|help>

    \t# Generate the cache file - options_cache.json.
    \t<cg|cache_generate> <vcpkg_location>

    \t# Display the contents of the cache.
    \t<ca|cache>

    \t# Edit the cache.
    \t<cae|cache_edit>

    \t# Get a value from the cache by key.
    \t<cag|cache_get> <key>

    \t# Prepares script to work with release.
    \t<r|release>

    \t# Prepares script to work with debug.
    \t<d|debug>

    \t# Generate CMake project. Must provide -r or -d option beforehand.
    \t<pg|project_generate> <system-type> <build-type>
    \tCompiler types: "nt/msvc", "unix/clang", "unix/gcc"
    \tExample: -r <pg|project_generate> "nt/msvc"
    \tNotes:
    \t* This will generate the CMake project in the current directory.
    \t* This will NOT overwrite any existing CMake project (if the project was already generated).
    \t* Both arguments are required.

    \t# Builds the generated CMake Project. Must provide -r or -d option beforehand.
    \t<b|build>
    \tExample: -r <b|build>
    \tNote: This will build the CMake project in the current directory.

    \t# Installs the build. Must provide -r or -d option beforehand.
    \t<i|install>
    \tExample: -r <i|install>
    \tNotes:
    \t* This will install the CMake project in the current directory.
    \t* The project must be built before it can be installed.
    \t* This will overwrite any existing installation (if the project was already installed).

    \t# Creates a symlink to all exes located in the bin. Must provide -r or -d option beforehand. Build must be installed beforehand.
    \t<s|symlink>
    \tExample: -r <s|symlink>
    \tNotes:
    \t* This will create a symlink to all exes located in the bin directory.
    \t* The project must be installed before it can be symlinked.
    \t* This will overwrite any existing symlinks (if the project was already symlinked).

    \t# Cleans the build and install directories.
    \t<c|clean>

    \t# Cleans the build and install directories and rebuilds. Must provide -r or -d option beforehand.
    \t<cr|clean_rebuild>
    \tExample: -r <cr|clean_rebuild>

    \t# Cleans the build and install directories then rebuilds and reinstalls. Must provide -r or -d option beforehand.
    \t<ci|clean_install>
    \tExample: -r <ci|clean_install>

    \t# Cleans <b>uild or <i>nstall directory.
    \t<cd|clean_dir> "b" or "i"
    \tExample: <cd|clean_dir> b

};
}

sub generate_help {
    return qq{
    Usage: $0 generate <verb> <args> ...

    \t# Template
    \t<t|template>

};
}

sub venv_help {
    return qq{
    Usage: $0 venv <verb> <args> ...

    \t# Initializes the venv environment.
    \t<i|initialize>

    \t# Deinitializes the venv environment.
    \t<d|deinitialize>

    \t# Deinitializes then initializes the venv environment.
    \t<r|reinitialize>

    \t# Force clean the venv environment. Use this if reinitialize or deinitializes fail.
    \t<fc|force_clean>

};
}

1;
