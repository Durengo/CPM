#!/usr/bin/perl
use strict;
use warnings;

package CPM::Command::Venv;
use base qw(CLI::Framework::Command);
use FindBin;
use File::Spec;
use Cache::FileCache;
use String::Util 'trim';
use File::Path 'remove_tree';
use Try::Tiny;
use JSON::PP;
use Cwd;

use lib "$main::CoreDir\\Lib";
use CPMCache;
use CPMLog;
use CPMBuildInterface;
use CPMHelpText;

my $working_dir          = getcwd();
my $using_vcpkg_location = JSON::PP::false;
my $environment_cache    = CPMCache->new();
my $installs_cache       = CPMCache->new();
my $options_cache        = CPMCache->new();
my $PY_VENV_DIR;

sub option_spec {
    [ 'initialize|i' => 'Initializes the venv environment.' ],
      [ 'deinitialize|d' => 'Deinitializes the venv environment.' ],
      [ 'reinitialize|r' =>
          'deinitializes then initializes the venv environment.' ],
      [ 'force_clean|fc' =>
'Force clean the venv environment. Use this if reinitialize or deinitializes fail.'
      ],
      [ 'help|h' => 'Display help.' ],;
}

sub run {
    my $self = shift;
    my $opts = shift;

    $environment_cache->init_cache( 'env.json', 0 );
    $options_cache->init_cache( 'options_cache.json', 0 );

    try {
        $installs_cache->init_cache_from_path(
            File::Spec->catfile( $working_dir, 'cpm_install.json' ) );
    }
    catch {
        warn "An error occurred: $_";
        die "Does the file 'cpm_install.json' exist in the current directory?";
    };

    my ( $success, $value ) = $options_cache->try_get_pair('source_directory');
    if ($success) {
        $working_dir = $value;
    }
    else {
        die "Source directory is not set.\n";
    }

    my ( $success1, $value1 ) =
      $options_cache->try_get_pair('source_directory');
    if ($success1) {
        $PY_VENV_DIR = File::Spec->canonpath("$value1\\venv");
    }
    else {
        die "Source directory is not set.\n";
    }

    my ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
      retrieve_install_json($installs_cache);

   # check $post_install_ref list and if we locate "venv" string we can continue
    my $found_venv = 0;
    foreach my $package (@$post_install_ref) {
        if ( $package eq "venv" ) {
            $found_venv = 1;
            last;
        }
    }

    if ( $found_venv == 0 ) {
        die "No venv found in post installation instructions.\n";
    }

    if ( !keys %$opts ) {
        print("No options provided.\n");
        return;
    }
    if ( $opts->{'help'} ) {
        return venv_help();
    }
    if ( $opts->{'initialize'} ) {
        initialize_venv();
    }
    if ( $opts->{'deinitialize'} ) {
        deinitialize_venv();
    }
    if ( $opts->{'reinitialize'} ) {
        reinitialize_venv();
    }
    if ( $opts->{'force_clean'} ) {
        delete_venv_folder();
    }

    return 0;
}

sub retrieve_install_json {
    my ($installs_cache) = @_;

    my $prerequisites_ref = $installs_cache->get_pair('prerequisites');

    my $packages_ref = $installs_cache->get_pair('packages');

    my $post_install_ref = $installs_cache->get_pair('post_install');

    return ( $prerequisites_ref, $packages_ref, $post_install_ref );
}

sub reinitialize_venv {
    print("Running Venv Post-Install Configuration...\n");
    if ( -d $PY_VENV_DIR ) {
        deinitialize_venv();
    }
    initialize_venv();
}

sub initialize_venv {

    if ( -d $PY_VENV_DIR ) {
        print "Directory '$PY_VENV_DIR' already exists. Clearing...\n";

        # disable_venv($PY_VENV_DIR);
        delete_venv_folder($PY_VENV_DIR);
    }

    mkdir $PY_VENV_DIR
      or die "Failed to create directory: $PY_VENV_DIR\n";

    # chdir $venv_dir or die "Unable to change directory: '$venv_dir'\n";

    my $vcpkg_location;

    # my $working_dir;
    my $path_to_python_interpretor;
    my $is_vcpkg_location_provided;

    my ( $success, $value ) =
      $environment_cache->try_get_pair('using_provided_vcpkg_location');
    if ($success) {
        $is_vcpkg_location_provided = $value;
    }
    else {
        die "using_provided_vcpkg_location not set.\n";
    }

    if ( $is_vcpkg_location_provided eq JSON::PP::true ) {
        my ( $success2, $value2 ) =
          $environment_cache->try_get_pair('provided_vcpkg_location');
        if ($success) {
            $vcpkg_location = $value2;
        }
        else {
            die "provided_vcpkg_location not set.\n";
        }
    }
    else {
        my ( $success2, $value2 ) =
          $environment_cache->try_get_pair('generated_vcpkg_location');
        if ($success) {
            $vcpkg_location = $value2;
        }
        else {
            die "generated_vcpkg_location not set.\n";
        }
    }

    $path_to_python_interpretor = File::Spec->canonpath(
        "$vcpkg_location\\installed\\x64-windows\\tools\\python3");

    chdir $working_dir
      or die "Unable to change directory: $working_dir\n";

    my $exit_status =
      system("$path_to_python_interpretor/python -m venv $PY_VENV_DIR");
    if ( $exit_status == 0 ) {
        print "Python venv initialized successfully.\n";
    }
    else {
        die "Failed to initialize Python venv: $!\n";
    }

    my $activation_script = File::Spec->canonpath("$PY_VENV_DIR\\Scripts");
    my $exit_status2      = system("$activation_script\\activate");
    if ( $exit_status2 == 0 ) {
        print "Python venv activated successfully.\n";
    }
    else {
        die "Failed to activate Python venv: $!\n";
    }

    set_cache_values();
}

sub deinitialize_venv {
    if ( -d $PY_VENV_DIR ) {
        print "Directory '$PY_VENV_DIR' found. Clearing...\n";

        disable_venv();
        delete_venv_folder($PY_VENV_DIR);
    }
    else {
        die "Directory '$PY_VENV_DIR' does not exist. Aborting.\n";
    }

}

sub disable_venv {

    my $script_location = File::Spec->canonpath("$PY_VENV_DIR\\Scripts");

    chdir "$script_location"
      or die "Unable to change directory: $script_location\n";
    my $exit_status = system("deactivate");
    if ( $exit_status == 0 ) {
        print "Python venv deactivated successfully.\n";
    }
    else {
        die "Failed to deactivate Python venv: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub delete_venv_folder {
    print "Removing $PY_VENV_DIR\n";

    if ( -d $PY_VENV_DIR ) {

        remove_tree( $PY_VENV_DIR, { error => \my $err } );
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
}

sub set_cache_values {
    my $vcpkg_location;
    my $path_to_python_interpretor;
    my $is_vcpkg_location_provided;

    my ( $success, $value ) =
      $environment_cache->try_get_pair('using_provided_vcpkg_location');
    if ($success) {
        $is_vcpkg_location_provided = $value;
    }
    else {
        die "using_provided_vcpkg_location not set.\n";
    }

    if ( $is_vcpkg_location_provided eq JSON::PP::true ) {
        my ( $success2, $value2 ) =
          $environment_cache->try_get_pair('provided_vcpkg_location');
        if ($success) {
            $vcpkg_location = $value2;
        }
        else {
            die "provided_vcpkg_location not set.\n";
        }
    }
    else {
        my ( $success2, $value2 ) =
          $environment_cache->try_get_pair('generated_vcpkg_location');
        if ($success) {
            $vcpkg_location = $value2;
        }
        else {
            die "generated_vcpkg_location not set.\n";
        }
    }

    $path_to_python_interpretor = File::Spec->canonpath(
        "$vcpkg_location\\installed\\x64-windows\\tools\\python3");

    $options_cache->put_pair( 'local_venv_root',
        File::Spec->canonpath("$PY_VENV_DIR\\Scripts") );

    $options_cache->put_pair( 'local_python_interpretor',
        $path_to_python_interpretor );
}

# sub execute_build_py {
#     my @args = @_;

#     my $build_py = CPMBuildInterface::get_build_py();
#     my $cmd      = "py \"$build_py\"";

#     print("Executing: $cmd @args\n");

#     my $script_location = CPMBuildInterface::get_script_location();

#     my $exit_status = system( $cmd, @args );
#     if ( $exit_status == 0 ) {
#         CPMLog::info("Successfully executed build.py");
#     }
#     else {
#         die "Failed to generate project: $!\n";
#     }

# }

sub venv_help {
    return CPMHelpText::venv_help();
}
