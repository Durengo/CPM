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

    \t# Forces vcpkg to install packages again (does not remove any existing packages).
    \t<fpi|force_package_install>

    \t# Runs the setup without checking for runtime dependencies (ONLY FOR CI USE).
    \t<ndc|no-deps-check>

};
}

sub build_help {
    return qq{
    Usage: $0 build <verb> <args> ...
    
    \t# Displays the help.
    \t<h|help>

    \t# Generate the cache file - options_cache.json
    \t<cg|cache_generate> <vcpkg_location>

    \t# Display the contents of the cache.
    \t<ca|cache>

    \t# Edit the cache.
    \t<cae|cache_edit>

    \t# Get a value from the cache by key.
    \t<cag|cache_get> <key>

    \t# Generate CMake project.
    \t<pg|project_generate> <system-type> <build-type>
    \tSystem types: "nt/msvc", "unix/clang", "unix/gcc"
    \tBuild types: "Debug", "Release"
    \tExample: <pg|project_generate> "nt/msvc" <pg|project_generate> "Debug"
    \tNotes:
    \t* This will generate the CMake project in the current directory.
    \t* This will NOT overwrite any existing CMake project (if the project was already generated).
    \t* Both arguments are required.

    \t# Prepares script to work with release.
    \t<r|release>

    \t# Prepares script to work with debug.
    \t<d|debug>

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

1;
