#!/usr/bin/perl
use strict;
use warnings;
use FindBin;
use File::Spec;
use File::Copy;
use Getopt::Long;
use Cwd;

# use Term::Menus;

my $os;
my $this_dir;
my $provided_dir;

# Variables to set up CMake project
# LIST OF MODIFIABLE VARIABLES
# |CMAKE_MINIMUM_VERSION| - Minimum version of CMake required | 3.22 |
# |CXX_OR_C_STANDARD| - Standard of C++ or C | C or CXX |
# |CXX_OR_C_VERSION_NAME| - Version of C++ or C | 99, 11, 14, 17, 20 |
# |TESTING| - Whether or not to include testing | ON or OFF|
# |EXAMPLES| - Whether or not to include examples | ON or OFF|
# |PROJECT_NAME| - Name of project | MyProject |
# |BUILD_PY_LOCATION| - Location of build.py file | PATH TO OS SPECIFIC BUILD.PY |
# |IF_VCPKG| - Whether or not to use vcpkg | ON or OFF |
# |VCPKG_LIBS| - List of vcpkg dependencies | "find_package(GiT QUIET)\n", "find_package(fmt CONFIG REQUIRED)\n", etc... |
# |VCPKG_LIBS_TARGETS| - List of vcpkg dependencies to provide target links
# "fmt::fmt\n"
# "spdlog::spdlog\n"
# "OpenGL::GL\n"
# "${OPENGL_LIBRARIES}\n"
# "${raylib_LIBRARIES}\n"
# "${Python3_LIBRARIES}\n"
# "${Boost_LIBRARIES}\n"
# "${VCPKG_CORE}/lib/${PYTHON_VERSION}.lib\n"
# "PkgConfig::GTKMM_LIB\n"
# etc...
# |
# |VCPOKG_INCLUDE_TARGETS| - List of vcpkg dependencies to provide target includes
# |
# "${PROJECT_SOURCE_DIR}/src\n"
# "${OPENGL_INCLUDE_DIR}\n"
# "${raylib_INCLUDE_DIRS}\n"
# "${Python3_INCLUDE_DIRS}\n"
# "${Boost_INCLUDE_DIRS}\n"
# "${GTKMM_LIB_INCLUDE_DIRS}\n"
# |
# |IF_PYTHON| - Whether or not to use Python vcpkg library
# |IF_BOOST| - Whether or not to use Boost vcpkg library
# |IF_STB| - Whether or not to use stb vcpkg library
# |PKG_CONFIG_LIBS| - List of pkg-config dependencies
# |
# pkg_check_modules(GTKMM_LIB REQUIRED IMPORTED_TARGET gtkmm-4.0)
#        if(GTKMM_LIB_FOUND)
#                message(STATUS "GTKmm found")
#        else()
#                message(FATAL_ERROR "GTKmm not found")
#        endif()
# |
# |EXE_NAME| - Name of executable | HelloWorld |
# |IF_LIBRARY| - Whether or not to create a library | ON or OFF |
# |LIB_NAME| - Name of library | MyLibrary |
# LIST OF MODIFIABLE VARIABLES

my $project_name           = 'MyProject';
my $cmake_minimum_required = '3.22';
my $language               = 'CXX';
my $cxx_standard           = '11';

my $testing  = 0;
my $examples = 0;

my $library  = 0;
my $exe_name = 'HelloWorld';

my $vcpkg      = 0;
my @vcpkg_libs = ();

my $build_py_location = "";

my $current_menu = 'main_menu';

main();

sub main {
    os_specific();
}

sub check_os {
    $os = $^O;
    if ( $os eq 'MSWin32' ) {
        print "Windows platform detected.\n";
    }
    elsif ( $os eq 'linux' ) {
        die "Linux is not supported.\n";
    }
    elsif ( $os eq 'darwin' ) {
        die "Mac OS X is not supported.\n";
    }
    else {
        die "Unknown operating system: $os\n";
    }
}

sub os_specific {
    check_os();

    if ( $os eq 'MSWin32' ) {
        for_windows();
    }

    elsif ( $os eq 'linux' ) {
        for_linux();
    }

    elsif ( $os eq 'darwin' ) {
        for_mac();
    }
}

sub for_windows {
    while (1) {
        if ( $current_menu eq 'main_menu' ) {
            print_main_menu();
            my $choice = get_user_input();
            if ( $choice == 0 ) {
                exit;
            }
            elsif ( $choice == 1 ) {
                $current_menu = 'generate_menu';
            }
            else {
                print "Invalid choice. Please try again.\n";
            }
        }
        elsif ( $current_menu eq 'generate_menu' ) {
            print_generate_menu();
            my $choice = get_user_input();
            if ( $choice == 0 ) {
                $current_menu = 'main_menu';
            }
            if ( $choice == 1 ) {
                generate_cpp_default();
            }

            # elsif ( $choice == 2 ) {
            #     generate_cpp_cmake();
            # }
            # elsif ( $choice == 3 ) {
            #     generate_python_cmake();
            # }
            # elsif ( $choice == 4 ) {
            #     generate_custom_cmake();
            # }
            else {
                print "Invalid choice. Please try again.\n";
            }
        }
    }
}

sub print_main_menu {
    print "\nMain Menu:\n";
    print "0. Exit\n";
    print "1. Generate CMakeLists.txt\n";
    print "Please enter your choice: ";
}

sub print_generate_menu {
    print "\nGenerate Menu:\n";
    print "0. Return to Previous Menu\n";
    print "1. Generate Default CPP CMakeLists.txt\n";
    print "Please enter your choice: ";
}

sub get_user_input {
    my $input = <STDIN>;
    chomp($input);
    return $input;
}

sub generate_cpp_default {

    # Implement logic to generate Basic CMakeLists.txt here
    print "Generating Default CMakeLists.txt...\n";
}

# sub generate_cpp_cmake {

#     # Implement logic to generate C/C++ Project CMakeLists.txt here
#     print "Generating CMakeLists.txt for C/C++ Project...\n";
# }
