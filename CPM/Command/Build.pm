#!/usr/bin/perl
use strict;
use warnings;
use JSON::PP;

package CPM::Command::Build;
use base qw(CLI::Framework::Command);

use FindBin;
use File::Spec;

use lib "$main::CoreDir\\Lib";

# use lib File::Spec->catdir( $FindBin::Bin, '..', 'Lib' );
use CPMCache;

use CPMLog;

use Cwd;
my $working_dir = getcwd();

my $using_vcpkg_location = JSON::PP::false;

my $build_environemnt_cache = CPMCache->new();
my $build_options_cache     = CPMCache->new();
my $build_installs_cache    = CPMCache->new();

use CPMBuildInterface;

use String::Util 'trim';

use CPMHelpText;

use File::Path 'remove_tree';

my $build_type = "something";

sub option_spec {
    [ 'help|h' => 'Display help.' ],

      [ 'cache|ca'               => 'Display the contents of the cache.' ],
      [ 'cache_edit|cae'         => 'Edit the cache.' ],
      [ 'cache_get|cag=s'        => 'Get a value from the cache by key.' ],
      [ 'project_generate|pg=s@' => 'Generate CMake project.' ],
      [ 'release|r'              => 'Prepares script to work with release.' ],
      [ 'debug|d'                => 'Prepares script to work with debug.' ],
      [ 'build|b' =>
'Builds the generated CMake Project. Must provide -r or -d option beforehand.'
      ],
      [ 'install|i' =>
          'Installs the build. Must provide -r or -d option beforehand.' ],
      [ 'clean|c' => 'Cleans the build and install directories.' ],
      [ 'clean_rebuild|cr' =>
'Cleans the build and install directories and rebuilds. Must provide -r or -d option beforehand.'
      ],
      [ 'clean_install|ci' =>
'Cleans the build and install directories then rebuilds and reinstalls. Must provide -r or -d option beforehand.'
      ],
      [ 'clean_dir|cd=s' => 'Cleans <b>uild or <i>nstall directory.' ],

      ;
}

sub run {
    my $self = shift;
    my $opts = shift;

    $build_environemnt_cache->init_cache( 'env.json', 0 );
    $build_installs_cache->init_cache( 'install.json', 0 );
    $build_options_cache->init_cache( 'options_cache.json', 0 );

    my ( $success, $value ) =
      $build_environemnt_cache->try_get_pair('using_vcpkg_location');
    if ($success) {
        $using_vcpkg_location = $value;
    }

    CPMBuildInterface::check_build_py();

    # CPMBuildInterface::clear_build_cache();

    my $arg1;
    my $arg2;

    if ( !keys %$opts ) {
        print("No options provided.\n");
        return;
    }
    if ( $opts->{'help'} ) {
        return build_help();
    }

    if ( $opts->{'cache_generate'} ) {
        my $vcpkg_location = get_vcpkg_location_from_cache();
        execute_build_py( '--cache-generate', $vcpkg_location );
    }
    if ( $opts->{'cache'} ) {
        execute_build_py('--cache');
    }
    if ( $opts->{'cache_edit'} ) {
        execute_build_py('--cache-edit');
    }
    if ( $opts->{'cache_get'} ) {
        $arg1 = $opts->{'cache_get'};
        execute_build_py( '--cache-get', $arg1 );
    }
    if ( $opts->{'project_generate'} ) {
        my @args = @{ $opts->{'project_generate'} };
        print "args: @args\n";
        $arg1 = $args[0];
        $arg2 = $args[1];
        $build_environemnt_cache->put_pair( 'last_used_system_type', $arg1 );
        execute_build_py( '--project-generate', $arg1, $arg2 );
    }
    if ( $opts->{'clean'} ) {
        clean_both_dirs();
    }
    if ( $opts->{'release'} and $opts->{'debug'} ) {
        die "Cannot use both -r and -d options.\n";
    }
    if ( $opts->{'release'} ) {
        $build_type = "Release";
        print "build_type: $build_type\n";
    }
    if ( $opts->{'debug'} ) {
        $build_type = "Debug";
        print "build_type: $build_type\n";
    }
    if ( $opts->{'build'} ) {

        check_build_dir();
        if ( $build_type eq "" ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        execute_build_py( '--project-build', $build_type );
    }
    if ( $opts->{'install'} ) {
        if ( $build_type eq '' ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        execute_build_py( '--project-install', $build_type );
    }
    if ( $opts->{'clean-rebuild'} ) {
        if ( $build_type eq '' ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        clean_both_dirs();
        execute_build_py( '--project-build', $build_type );
    }
    if ( $opts->{'clean-install'} ) {
        if ( $build_type eq '' ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        clean_both_dirs();
        execute_build_py( '--project-build',   $build_type );
        execute_build_py( '--project-install', $build_type );
    }
    if ( $opts->{'clean-dir'} ) {
        $arg1 = $opts->{'clean-dir'};
        if ( $arg1 eq 'b' ) {
            clean_build_dir();
        }
        elsif ( $arg1 eq 'i' ) {
            clean_install_dir();
        }
        else {
            die "Invalid argument: $arg1\n";
        }
    }

    # elsif ( $opts->{'project_build'} ) {
    #     my $arg1 = $opts->{'project_build'};
    #     execute_build_py( '--project-build', $arg1 );
    # }
    # elsif ( $opts->{'project_install'} ) {
    #     my $arg1 = $opts->{'project_install'};
    #     execute_build_py( '--project-install', $arg1 );
    # }

    # if ( keys %$opts ) {

    # }
    # else {

    # }

    return 0;
}

sub check_build_dir {
    my $build_dir = File::Spec->canonpath("$main::CoreDir\\Build");

    if ( -e $build_dir ) {
        return;
    }
    else {
        print
"$build_dir does not exist. Attempting to regenerate CMake Project.\n";
        my ( $success, $value ) =
          $build_environemnt_cache->try_get_pair('last_used_system_type');
        if ($success) {

            # my $arg1 =
            #   $build_environemnt_cache->get_pair('last_used_system_type');
            # execute_build_py( '--project-generate', $arg1, $build_type );
            execute_build_py( '--project-generate', $value, $build_type );
        }
        else {
            die "Generate the CMake project at least once.\n";
        }
    }
}

sub clean_build_dir {
    my $build_dir = "";

    my ( $success, $value ) =
      $build_options_cache->try_get_pair('source_directory');
    if ($success) {
        $build_dir = File::Spec->canonpath("$value\\Build");
    }
    else {
        die "Source directory is not set.\n";
    }

    if ( $build_dir eq "" ) {
        die "Source directory is empty.\n";
    }

    if ( -e $build_dir ) {
        if ( -d $build_dir ) {
            print "Removing $build_dir\n";

            # rmdir $build_dir or die "Unable to remove $build_dir: $!";
            remove_tree( $build_dir, { error => \my $err } );
            if (@$err) {
                for my $diag (@$err) {
                    my ( $file, $message ) = %$diag;
                    if ( $file eq '' ) {
                        print "General error: $message\n";
                    }
                    else {
                        print "Problem unlinking $file: $message\n";
                    }
                }
            }
        }
        else {
            die "$build_dir is not a directory.\n";
        }
    }
    else {
        print "$build_dir does not exist.\n";
    }
}

sub clean_install_dir {

    # my $install_dir = File::Spec->canonpath("$main::CoreDir\\Install");

    my $install_dir = "";

    my ( $success, $value ) =
      $build_options_cache->try_get_pair('source_directory');
    if ($success) {
        $install_dir = File::Spec->canonpath("$value\\Install");
    }
    else {
        die "Source directory is not set.\n";
    }

    if ( $install_dir eq "" ) {
        die "Source directory is empty.\n";
    }

    if ( -e $install_dir ) {
        if ( -d $install_dir ) {
            print "Removing $install_dir\n";

            # rmdir $install_dir or die "Unable to remove $install_dir: $!";
            remove_tree( $install_dir, { error => \my $err } );
            if (@$err) {
                for my $diag (@$err) {
                    my ( $file, $message ) = %$diag;
                    if ( $file eq '' ) {
                        print "General error: $message\n";
                    }
                    else {
                        print "Problem unlinking $file: $message\n";
                    }
                }
            }
        }
        else {
            die "$install_dir is not a directory.\n";
        }
    }
    else {
        print "$install_dir does not exist.\n";
    }
}

sub clean_both_dirs {
    clean_build_dir();
    clean_install_dir();
}

sub get_vcpkg_location_from_cache {
    my $is_vcpkg_location_provided =
      $build_environemnt_cache->try_get_pair('using_provided_vcpkg_location');
    if ( $is_vcpkg_location_provided eq JSON::PP::true ) {
        return $build_environemnt_cache->get_pair('provided_vcpkg_location');
    }
    else {
        return $build_environemnt_cache->get_pair('generated_vcpkg_location');
    }
}

sub execute_build_py {
    my @args = @_;

    my $build_py = CPMBuildInterface::get_build_py();
    my $cmd      = "py \"$build_py\"";

    print("Executing: $cmd @args\n");

    my $script_location = CPMBuildInterface::get_script_location();

    my $exit_status = system( $cmd, @args );
    if ( $exit_status == 0 ) {
        CPMLog::info("Successfully executed build.py");
    }
    else {
        die "Failed to generate project: $!\n";
    }

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

#     my $key = $build_environemnt_cache->get_pair('generated_vcpkg_location');
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
    return CPMHelpText::build_help();
}

1;
