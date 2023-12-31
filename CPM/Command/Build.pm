#!/usr/bin/perl
use strict;
use warnings;

package CPM::Command::Build;
use base qw(CLI::Framework::Command);
use FindBin;
use File::Spec;
use String::Util 'trim';
use File::Path 'remove_tree';
use JSON::PP;
use Cwd;

use lib "$main::CoreDir\\Lib";
use CPMCache;
use CPMLog;
use CPMBuildInterface;
use CPMHelpText;
use CPMScriptInterface;

my $working_dir             = getcwd();
my $using_vcpkg_location    = JSON::PP::false;
my $build_environemnt_cache = CPMCache->new();
my $build_options_cache     = CPMCache->new();
my $build_installs_cache    = CPMCache->new();
my $build_type              = "";

sub option_spec {
    (
        [ 'help|h' => 'Display help.' ],

        [ 'cache|ca'        => 'Display the contents of the cache.' ],
        [ 'cache_edit|cae'  => 'Edit the cache.' ],
        [ 'cache_get|cag=s' => 'Get a value from the cache by key.' ],
        [ 'release|r'       => 'Prepares script to work with release.' ],
        [ 'debug|d'         => 'Prepares script to work with debug.' ],
        [
            'project_generate|pg=s' =>
              'Generate CMake project. Must provide -r or -d option beforehand.'
        ],
        [
            'build|b' =>
'Builds the generated CMake Project. Must provide -r or -d option beforehand.'
        ],
        [
            'install|i' =>
              'Installs the build. Must provide -r or -d option beforehand.'
        ],
        [
            'symlink|s' =>
'Creates a symlink to all exes located in the bin. Must provide -r or -d option beforehand. Build must be installed beforehand.'
        ],
        [ 'clean|c' => 'Cleans the build and install directories.' ],
        [
            'clean_rebuild|cr' =>
'Cleans the build and install directories and rebuilds. Must provide -r or -d option beforehand.'
        ],
        [
            'clean_install|ci' =>
'Cleans the build and install directories then rebuilds and reinstalls. Must provide -r or -d option beforehand.'
        ],
        [ 'clean_dir|cd=s' => 'Cleans <b>uild or <i>nstall directory.' ],
    )
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
    CPMScriptInterface::shell_script_location();

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
    if ( $opts->{'clean'} ) {
        clean_both_dirs();
    }
    if ( $opts->{'release'} and $opts->{'debug'} ) {
        die "Cannot use both -r and -d options.\n";
    }
    if ( $opts->{'release'} ) {
        $build_type = "Release";
        print "Build Type Set To: $build_type\n";
    }
    if ( $opts->{'debug'} ) {
        $build_type = "Debug";
        print "Build Type Set To: $build_type\n";
    }
    if ( $opts->{'project_generate'} ) {
        if ( $build_type eq "" ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        $arg1 = $opts->{'project_generate'};
        $build_environemnt_cache->put_pair( 'last_used_system_type', $arg1 );
        execute_build_py( '--project-generate', $arg1, $build_type );
    }
    if ( $opts->{'build'} ) {

        if ( $build_type eq "" ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        check_build_dir();
        execute_build_py( '--project-build', $build_type );
    }
    if ( $opts->{'install'} ) {
        if ( $build_type eq '' ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        execute_build_py( '--project-install', $build_type );
    }
    if ( $opts->{'symlink'} ) {
        if ( $build_type eq '' ) {
            die "Must provide -r or -d option beforehand.\n";
        }
        symlink_installation();
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

    return 0;
}

sub check_build_dir {
    my $build_dir;

    my ( $success, $value ) =
      $build_options_cache->try_get_pair('source_directory');
    if ($success) {
        $build_dir = File::Spec->canonpath("$value\\Build");
    }
    else {
        die "Source directory is not set.\n";
    }

    $build_dir = File::Spec->canonpath("$value\\Build");

    if ( -e $build_dir ) {
        return;
    }
    else {
        print
"$build_dir does not exist. Attempting to regenerate CMake Project.\n";
        my ( $success2, $value2 ) =
          $build_environemnt_cache->try_get_pair('last_used_system_type');
        if ($success2) {
            execute_build_py( '--project-generate', $value2, $build_type );
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

sub symlink_installation {
    die "Disabled due to required elevated permissions.\n";

    my $install_dir = "";

    my ( $success, $value ) =
      $build_options_cache->try_get_pair('last_installation_directory');
    if ($success) {
        $install_dir = File::Spec->canonpath("$value");
    }
    else {
        die "Source directory is not set.\n";
    }

    if ( $install_dir eq "" ) {
        die "Source directory is empty.\n";
    }

    # Get all files into a list that are in $install_dir/bin and end with .exe
    my $bin_path = File::Spec->canonpath("$install_dir\\bin");
    my @exes     = glob("$bin_path\\*.exe");

    # my $symlink_exe_location = CPMScriptInterface::shell_script_location();

    foreach my $exe (@exes) {
        my $exe_name = File::Basename::basename($exe);
        $exe_name =~ s/\.exe$//i;
        CPMScriptInterface::execute_symlink( $exe_name, $install_dir, $exe );
    }
}

sub build_help {
    return CPMHelpText::build_help();
}

1;
