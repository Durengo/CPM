#!/usr/bin/perl
use strict;
use warnings;
use JSON::PP;

package CPM::Command::Generate;
use base qw(CLI::Framework::Command);

use FindBin;
use File::Spec;
use lib File::Spec->catdir( $FindBin::Bin, '..', 'Lib' );
use CPMCache;

use CPMLog;

use Cwd;
my $working_dir = getcwd();

my $using_vcpkg_location = JSON::PP::false;

my $environemnt_cache = CPMCache->new();
my $installs_cache    = CPMCache->new();

use CPMBuildInterface;

use String::Util 'trim';

sub option_spec {
    [ 'help|h' => 'Display help.' ],
      [ 'wizard|w' => 'Start the generation wizard.' ],

      # [ 'cache-help|ca' => 'Display help for cache.' ],
      # [ 'project-help|ph' => 'Display help for project management.' ],

      [ 'cache_generate|cg' =>
          'Generate the cache file. (options_cache.json)' ],
      [ 'cache|ca'        => 'Display the contents of the cache.' ],
      [ 'cache_edit|cae'  => 'Edit the cache.' ],
      [ 'cache_get|cag=s' => 'Get a value from the cache by key.' ],

      [ 'project_generate|pg=s@' => 'Generate CMake project.' ],
      [ 'project_build|pb=s'     => 'Build CMake project.' ],
      [ 'project_install|pi=s'   => 'Install CMake project.' ],

      [ 'b' =>
'Runs the build on the current platform and rebuilds everything in debug.'
      ],

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

    # CPMBuildInterface::clear_build_cache();

    if ( $opts->{'cache_generate'} ) {
        my $vcpkg_location = get_vcpkg_location_from_cache();
        execute_build_py( '--cache-generate', $vcpkg_location );
    }
    elsif ( $opts->{'cache'} ) {
        execute_build_py('--cache');
    }
    elsif ( $opts->{'cache_edit'} ) {
        execute_build_py('--cache-edit');
    }
    elsif ( $opts->{'cache_get'} ) {
        my $arg1 = $opts->{'cache_get'};
        execute_build_py( '--cache-get', $arg1 );
    }
    elsif ( $opts->{'project_generate'} ) {
        my @args = @{ $opts->{'project_generate'} };
        print "args: @args\n";
        my $arg1 = $args[0];
        my $arg2 = $args[1];
        execute_build_py( '--project-generate', $arg1, $arg2 );
    }
    elsif ( $opts->{'project_build'} ) {
        my $arg1 = $opts->{'project_build'};
        execute_build_py( '--project-build', $arg1 );
    }
    elsif ( $opts->{'project_install'} ) {
        my $arg1 = $opts->{'project_install'};
        execute_build_py( '--project-install', $arg1 );
    }
    elsif ( $opts->{'help'} ) {
        return build_help();
    }
    else {
        print("No options provided.\n");
    }

    # if ( keys %$opts ) {

    # }
    # else {

    # }

    return 0;
}

sub get_vcpkg_location_from_cache {
    my $is_vcpkg_location_provided =
      $environemnt_cache->try_get_pair('using_provided_vcpkg_location');
    if ( $is_vcpkg_location_provided eq JSON::PP::true ) {
        return $environemnt_cache->get_pair('provided_vcpkg_location');
    }
    else {
        return $environemnt_cache->get_pair('generated_vcpkg_location');
    }
}

sub execute_build_py {
    my @args = @_;

    my $build_py = CPMBuildInterface::get_build_py();
    my $cmd      = "py \"$build_py\"";

    print("Executing: $cmd @args\n");

    my $script_location = CPMBuildInterface::get_script_location();
    chdir $script_location
      or die "Unable to change directory: $script_location\n";

    my $exit_status = system( $cmd, @args );
    if ( $exit_status == 0 ) {
        CPMLog::info("Successfully executed build.py");
    }
    else {
        die "Failed to generate project: $!\n";
    }

    chdir $working_dir
      or die "Unable to change directory: $working_dir\n";

}

# sub execute_build_py_and_get_key {
#     my @args = @_;

#     my $build_py = CPMBuildInterface::get_build_py();
#     my $cmd      = "py \"$build_py\"";

#     print("Executing: $cmd @args\n");

#     my $script_location = CPMBuildInterface::get_script_location();
#     chdir $script_location
#       or die "Unable to change directory: $script_location\n";

#     my $exit_status = system( $cmd, @args );
#     if ( $exit_status == 0 ) {
#         CPMLog::info("Successfully executed build.py");
#     }
#     else {
#         die "Failed to generate project: $!\n";
#     }

#     chdir $working_dir
#       or die "Unable to change directory: $working_dir\n";

#     my $key = $environemnt_cache->get_pair('generated_vcpkg_location');
#     return $key;
# }

# sub run_vcpkg_location {

#     # my $vcpkg_location = shift;
#     # my @prerequisites  = @_;

#     my (
#         $vcpkg_location, $prerequisites_ref,
#         $packages_ref,   $skip_package_configurations
#     ) = @_;

#     my @prerequisites = @{$prerequisites_ref};
#     my @packages      = @{$packages_ref};

#     my @prerequisite_checks;

#     # my @prerequisite_checks = (
#     #     \&check_git,    \&check_cmake,
#     #     \&check_python, \&check_msvc_compiler,

#     #     # \&vcpkg_setup,      \&vcpkg_package_1,
#     #     # \&vcpkg_package_2,  \&vcpkg_package_3,
#     #     # \&vcpkg_package_4,  \&vcpkg_package_5,
#     #     # \&vcpkg_package_6,  \&vcpkg_package_7,
#     #     # \&vcpkg_package_8,  \&vcpkg_package_9,
#     #     # \&vcpkg_package_10, \&configure_package_4,
#     #     # \&vcpkg_integrate,  \&purge_previous_options_cache,
#     #     # \&setup_build_script,
#     # );

#     # foreach my $pre (@prerequisites) {
#     #     push @prerequisite_checks, sub { check_prerequisite($pre) };
#     # }

#     foreach my $pre (@prerequisites) {
#         my $check_function_name = "check_$pre";
#         if ( my $coderef = __PACKAGE__->can($check_function_name) ) {
#             push @prerequisite_checks, $coderef;
#         }
#         else {
#             warn "No check function for $pre";
#         }
#     }

#     push @prerequisite_checks, sub { vcpkg_setup($vcpkg_location) };

#     foreach my $pkg (@packages) {
#         my ( $library, $triplet ) = @{$pkg}{qw/library triplet/};
#         push @prerequisite_checks,
#           sub { install_vcpkg_package( $library, $triplet ) };
#     }

#     push @prerequisite_checks, \&intergrate_vcpkg;

#     if ( $skip_package_configurations eq JSON::PP::false ) {
#         foreach my $pkg (@packages) {
#             my ( $library, $triplet ) = @{$pkg}{qw/library triplet/};
#             push @prerequisite_checks,
#               sub { configure_package( $library, $triplet ) };
#         }
#     }
#     else {
#         print "Skipping package configurations...\n";
#     }

#     # push @prerequisite_checks, sub { configure_packages($packages_ref) };

#     my $total_checks     = scalar(@prerequisite_checks);
#     my $completed_checks = 0;

#     foreach my $check_function (@prerequisite_checks) {
#         $completed_checks++;

#         # print_col( $GREEN, "[$completed_checks/$total_checks] " );
#         print "[$completed_checks/$total_checks] ";
#         $check_function->();
#     }
# }

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

    \t# Build CMake project.
    \t<pb|project_build> <build preset>
    \tBuild presets: "nt/msvc", "unix/clang", "unix/gcc"
    \tExample: <pb|project_build> "nt/msvc"
    \tNote: This will build the CMake project in the current directory.

    \t# Install CMake project.
    \t<pi|project_install> <build type>
    \tBuild types: "Debug", "Release"
    \tExample: <pi|project_install> "Debug"
    \tNotes:
    \t* This will install the CMake project in the current directory.
    \t* The project must be built before it can be installed.
    \t* This will overwrite any existing installation (if the project was already installed).

}
}

1;
