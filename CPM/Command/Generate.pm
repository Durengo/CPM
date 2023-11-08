#!/usr/bin/perl
use strict;
use warnings;

package CPM::Command::Generate;
use base qw(CLI::Framework::Command);
use FindBin;
use File::Spec;
use String::Util 'trim';
use Term::Menus;
use JSON::PP;
use Cwd;

use lib "$main::CoreDir\\Lib";
use CPMCache;
use CPMLog;
use CPMBuildInterface;
use CPMHelpText;

my $working_dir          = getcwd();
my $using_vcpkg_location = JSON::PP::false;
my $environemnt_cache    = CPMCache->new();
my $installs_cache       = CPMCache->new();

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

my %project_variables = (
    CMAKE_MINIMUM_VERSION  => "3.22",
    CXX_OR_C_STANDARD      => "CXX",
    CXX_OR_C_VERSION_NAME  => "11",
    TESTING                => "ON",
    EXAMPLES               => "ON",
    PROJECT_NAME           => "MyProject",
    IF_VCPKG               => "OFF",
    VCPKG_LIBS             => "",
    VCPKG_LIBS_TARGETS     => "",
    VCPOKG_INCLUDE_TARGETS => "",
    IF_PYTHON              => "OFF",
    IF_BOOST               => "OFF",
    IF_STB                 => "OFF",
    PKG_CONFIG_LIBS        => "",
    EXE_NAME               => "HelloWorld",
    IF_LIBRARY             => "OFF",
    LIB_NAME               => "MyLibrary",
);

sub option_spec {
    [ 'help|h' => 'Display help.' ],
      [ 'wizard|w' => 'Start the generation wizard.' ],

      ;
}

sub run {
    my $self = shift;
    my $opts = shift;

    $environemnt_cache->init_cache( 'env.json', 0 );
    $installs_cache->init_cache( 'install.json', 0 );

    my ( $success, $value ) =
      $environemnt_cache->try_get_pair('using_vcpkg_location');
    if ($success) {
        $using_vcpkg_location = $value;
    }

    CPMBuildInterface::check_build_py();

    if ( !keys %$opts ) {
        print("No options provided.\n");
        return;
    }
    if ( $opts->{'help'} ) {
        return generate_help_help();
    }
    if ( $opts->{'wizard'} ) {
        return main_menu();
    }

    return 0;
}

sub prompt_for_input {
    my ($prompt) = @_;
    print "$prompt: ";
    my $input = <STDIN>;
    chomp $input;
    return $input;
}

sub generate_cmake_lists {
    my %vars = @_;

    # This function will generate the actual CMakeLists.txt content
    # based on the variables provided. For now, it's just a stub.
    print "Generating CMakeLists.txt...\n";

    # ... Generate the content ...
    print "CMakeLists.txt generated!\n";
}

sub main_menu {
    my @list      = ( 'First Item', 'Second Item', 'Third Item' );
    my $banner    = "  Please Pick an Item:";
    my $selection = &pick( \@list, $banner );
    print "SELECTION = $selection\n";
}

# sub main_menu {
#     my @menu_options;
#     foreach my $key ( sort keys %project_variables ) {
#         push @menu_options, "$key ($project_variables{$key})";
#     }
#     push @menu_options, "Generate CMakeLists.txt", "Exit";

#     my $selection;
#     my $menu = {
#         banner => "Select a variable to modify:",
#         prompt => "Your Choice: ",
#         items  => \@menu_options,
#     };

#     # Define the settings for the menu
#     my $settings = {
#         banner   => "Select a variable to modify:",
#         prompt   => "Your Choice: ",
#         'select' => 'single',    # if you want only single selection
#         'return' => 'value',     # if you want the selected value to be returned
#         'index'  => 0            # to start the menu at the first item
#     };

#     # Set the settings using the settings subroutine from Term::Menus
#     settings($settings);

#     while (1) {
#         $selection = pick( \@menu_options );

#         last if $selection eq "Exit";
#         if ( $selection eq "Generate CMakeLists.txt" ) {
#             generate_cmake_lists(%project_variables);
#             last;
#         }
#         else {
#             $selection =~ /^(.*?)\s\(/;
#             my $var_name  = $1;
#             my $new_value = prompt_for_input("Enter new value for $var_name");
#             $project_variables{$var_name} = $new_value;

#             # Update the menu to reflect the changes
#             $menu->{items}->[ grep { $menu->{items}->[$_] =~ /^$var_name\s\(/ }
#               0 .. $#{ $menu->{items} } ] = "$var_name ($new_value)";
#         }
#     }
# }

sub generate_help_help {
    return CPMHelpText::generate_help();
}

1;
