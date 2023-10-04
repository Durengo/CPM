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
my $project_name           = 'MyProject';
my $cmake_minimum_required = '3.22';
my $language               = 'CXX';
my $cxx_standard           = '11';
my $x64flag                = 1;

my $testing  = 0;
my $examples = 0;

my $library  = 0;
my $exe_name = 'HelloWorld';

my $vcpkg      = 0;
my @vcpkg_libs = ();

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
