#!/usr/bin/perl
use strict;
use warnings;

package CPM::Command::Setup;
use base qw(CLI::Framework::Command);
use FindBin;
use File::Spec;
use String::Util 'trim';
use Try::Tiny;
use JSON::PP;
use Cwd;

use lib "$main::CoreDir\\Lib";
use CPMCache;
use CPMLog;
use CPMBuildInterface;
use CPMHelpText;

my $working_dir             = getcwd();
my $using_vcpkg_location    = JSON::PP::false;
my $setup_environemnt_cache = CPMCache->new();
my $installs_cache          = CPMCache->new();

my $prerequisites;
my $packages;

sub option_spec {
    [ 'no_local_vcpkg|nlv|n' =>
'Tries to find an existing vcpkg installation otherwise runs the setup without using a local vcpkg installation. Optionally skips package configurations.'
    ],
      [ 'vcpkg_location|vl|l=s' =>
'Runs the setup with the specified vcpkg directory. Optionally skips package configurations.'
      ],
      [ 'skip_package_configurations|spc' =>
          'Skips package configuration when running nlv or lv.' ],
      [ 'no_deps_check|ndc' =>
'Runs the setup without checking for runtime dependencies (ONLY FOR CI USE).'
      ],
      [ 'force_package_install|fpi' =>
'Forces vcpkg to install packages again (does not remove any existing packages).'
      ],
      [ 'help|h' => 'Display help.' ],;
}

sub run {
    my $self = shift;
    my $opts = shift;

    $setup_environemnt_cache->init_cache( 'env.json', 0 );

    try {
        $installs_cache->init_cache_from_path(
            File::Spec->catfile( $working_dir, 'cpm_install.json' ) );
    }
    catch {
        warn "An error occurred: $_";
        die "Does the file 'cpm_install.json' exist in the current directory?";
    };

    # my $prerequisites = $installs_cache->get_pair('prerequisites');
    # CPMLog::trace("Detected Prerequisites: ");
    # print join( ", ", @{$prerequisites} );
    # CPMLog::trace("");
    # my $packages = $installs_cache->get_pair('packages');
    # CPMLog::trace("Detected Packages: ");
    # foreach my $package ( @{$packages} ) {

    #     # Print out the library and triplet for each package
    #     print "Library: $package->{library}, Triplet: $package->{triplet}\n";
    # }

    my ( $prerequisites_ref, $packages_ref, $post_install_ref );

    if ( $opts->{no_local_vcpkg} and $opts->{skip_package_configurations} ) {
        ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
          retrieve_install_json($installs_cache);

        my $vcpkg_location = "";
        $vcpkg_location = try_retrieve_vcpkg_location();

        run_vcpkg_location(
            $vcpkg_location,   $prerequisites_ref, $packages_ref,
            $post_install_ref, JSON::PP::true
        );
    }
    elsif ( $opts->{no_local_vcpkg} ) {
        ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
          retrieve_install_json($installs_cache);

        my $vcpkg_location = "";
        $vcpkg_location = try_retrieve_vcpkg_location();

        run_vcpkg_location(
            $vcpkg_location,   $prerequisites_ref, $packages_ref,
            $post_install_ref, JSON::PP::false
        );
    }
    elsif ( $opts->{vcpkg_location} and $opts->{skip_package_configurations} ) {
        ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
          retrieve_install_json($installs_cache);

        my $vcpkg_location = $opts->{'vcpkg_location'};

        # print("vcpkg-location: $vcpkg_location\n")
        run_vcpkg_location(
            $vcpkg_location,   $prerequisites_ref, $packages_ref,
            $post_install_ref, JSON::PP::true
        );
    }
    elsif ( $opts->{vcpkg_location} ) {
        ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
          retrieve_install_json($installs_cache);
        my $vcpkg_location = $opts->{'vcpkg_location'};

        # print("vcpkg-location: $vcpkg_location\n")
        run_vcpkg_location(
            $vcpkg_location,   $prerequisites_ref, $packages_ref,
            $post_install_ref, JSON::PP::false
        );
    }
    elsif ( $opts->{no_deps_check} ) {
        ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
          retrieve_install_json($installs_cache);
        run_do_deps_check();
    }
    elsif ( $opts->{force_package_install} ) {
        ( $prerequisites_ref, $packages_ref, $post_install_ref ) =
          retrieve_install_json($installs_cache);
        force_package_install();
    }
    elsif ( $opts->{'help'} ) {
        return setup_help();
    }
    else {
        print("No options provided.\n");
    }

    return 0;
}

sub try_retrieve_vcpkg_location {

    # for windows execute $where command to find vcpkg.exe
    my $os         = $^O;
    my $vcpkg_path = "";
    if ( $os eq 'MSWin32' ) {
        my $cmd  = "where";
        my @args = ("vcpkg");

        try {
            $vcpkg_path = qx/$cmd @args/;
            chomp $vcpkg_path;

            if ($vcpkg_path) {
                ($vcpkg_path) = $vcpkg_path =~ /^(.*)\\vcpkg\.exe$/i;
                CPMLog::info("vcpkg found at: $vcpkg_path");
                return $vcpkg_path;
            }
            else {
                CPMLog::error("vcpkg not found.");
                return $vcpkg_path;
            }
        }
        catch {
            warn "Error trying to find vcpkg: $_";
            return $vcpkg_path;
        }
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

sub retrieve_install_json {
    my ($installs_cache) = @_;

    my $prerequisites_ref = $installs_cache->get_pair('prerequisites');
    CPMLog::trace("Detected Prerequisites: ");
    print join( ", ", @{$prerequisites_ref} ) . "\n";
    CPMLog::trace("");

    my $packages_ref = $installs_cache->get_pair('packages');
    CPMLog::trace("Detected Packages: ");
    foreach my $package ( @{$packages_ref} ) {
        print "Library: $package->{library}, Triplet: $package->{triplet}\n";
    }

    my $post_install_ref = $installs_cache->get_pair('post_install');
    CPMLog::trace("Detected Post Install Requests: ");
    print join( ", ", @{$post_install_ref} ) . "\n";

    return ( $prerequisites_ref, $packages_ref, $post_install_ref );
}

sub check_vcpkg {
    my ( $success, $value ) =
      $setup_environemnt_cache->try_get_pair('using_vcpkg_location');
    if ($success) {
        $using_vcpkg_location = $value;
    }
    else {
        $setup_environemnt_cache->put_pair( 'using_provided_vcpkg_location',
            $using_vcpkg_location );
    }
}

sub force_package_install {
    return;
}

# sub purge_previous_options_cache {
#     my $file_remove = "options_cache.json";

#     chdir "utils" or die "Unable to change directory: utils\n";

#     if ( -e $file_remove ) {
#         unlink $file_remove or die "Unable to remove $file_remove: $!";
#         print "File $file_remove removed successfully.\n";
#     }
#     else {
#         print "File $file_remove does not exist.\n";
#     }

#     chdir $working_dir or die "Unable to change directory: $working_dir\n";
# }

# sub setup_build_script {
#     print "Setting up build.py for internal use...\n";

#     chdir "utils" or die "Unable to change directory: utils\n";

#     my $cmd  = "build.bat";
#     my @args = ( "-cg", "$working_dir/vendor/vcpkg" );

#     my $exit_status = system( $cmd, @args );
#     if ( $exit_status == 0 ) {
#         print "build.py setup successfully.\n";
#     }
#     else {
#         die "Failed to setup build.py: $!\n";
#     }

#     chdir $working_dir or die "Unable to change directory: $working_dir\n";
# }

sub vcpkg_setup {
    my ($vcpkg_location) = shift;

    $vcpkg_location = trim($vcpkg_location);

    if ( defined $vcpkg_location and $vcpkg_location ne "" ) {
        my $platform_vcpkg_path = File::Spec->canonpath($vcpkg_location);
        print "Parameter provided: $platform_vcpkg_path\n";

        if ( -e $platform_vcpkg_path ) {
            if ( -d $platform_vcpkg_path ) {
                CPMLog::info("$platform_vcpkg_path found.");
            }
            elsif ( -f $platform_vcpkg_path ) {
                die
"$platform_vcpkg_path is a file and it exists. Provide vcpkg root directory.\n";
            }
            else {
                die
"$platform_vcpkg_path exists but is neither a file nor a directory.\n";
            }
        }
        else {
            die "$platform_vcpkg_path does not exist.\n";
        }

        $setup_environemnt_cache->put_pair( 'provided_vcpkg_location',
            $platform_vcpkg_path );
        $using_vcpkg_location = JSON::PP::true;
        $setup_environemnt_cache->put_pair( 'using_provided_vcpkg_location',
            $using_vcpkg_location );

        my $options_cache = CPMCache->new();
        $options_cache->init_cache( 'options_cache.json', 0 );
        $options_cache->put_pair( 'vcpkg_root', $platform_vcpkg_path );

  #         my @prerequisite_checks = (
  #             sub {
  #                 print "Configuring vcpkg integration ...\n";
  #                 {
  #                     my $exit_status = system("vcpkg integrate install");
  #                     if ( $exit_status == 0 ) {
  #                         print "pkgconfig modules configure successfully.\n";
  #                     }
  #                     else {
  #                         die "Failed to configure pkgconfig modules: $!\n";
  #                     }
  #                 }
  #                 {
  #                     my $exit_status = system("vcpkg integrate powershell");
  #                     if ( $exit_status == 0 ) {
  #                         print "pkgconfig modules configure successfully.\n";
  #                     }
  #                     else {
  #                         die "Failed to configure pkgconfig modules: $!\n";
  #                     }
  #                 }

#                # {
#                #     my $exit_status = system("vcpkg bootsrap python");
#                #     if ( $exit_status == 0 ) {
#                #         print "pkgconfig modules configure successfully.\n";
#                #     }
#                #     else {
#                #         die "Failed to configure pkgconfig modules: $!\n";
#                #     }
#                # }
#                # {
#                #     my $exit_status = system("vcpkg bootsrap python external");
#                #     if ( $exit_status == 0 ) {
#                #         print "pkgconfig modules configure successfully.\n";
#                #     }
#                #     else {
#                #         die "Failed to configure pkgconfig modules: $!\n";
#                #     }
#                # }
#                 {
#                     chdir "$location/installed/x64-windows/tools/boost-build"
#                       or die "Unable to change directory: $location\n";
#                     my $exit_status = system("bootstrap");
#                     if ( $exit_status == 0 ) {
#                         print "Bootstrapped boost.\n";
#                     }
#                     else {
#                         die "Failed to bootstrap boost: $!\n";
#                     }

#           # chdir "$location/installed/x64-windows/tools/boost-build/src/engine"
#           #   or die "Unable to change directory: $location\n";
#           # my $exit_status = system("b2 --prefix=$location");
#           # if ( $exit_status == 0 ) {
#           #     print "Installed boostrap engine.\n";
#           # }
#           # else {
#           #     die "Failed to install bootstrap engine: $!\n";
#           # }
#                 }

        #                 chdir $working_dir
        #                   or die "Unable to change directory: $working_dir\n";
        #             },
        #             \&purge_previous_options_cache,
        #             sub {
        #                 print "Setting up build.py for internal use...\n";

   #                 chdir "utils" or die "Unable to change directory: utils\n";

        #                 my $cmd  = "build.bat";
        #                 my @args = ( "-cg", "$location" );

        #                 my $exit_status = system( $cmd, @args );
        #                 if ( $exit_status == 0 ) {
        #                     print "build.py setup successfully.\n";
        #                 }
        #                 else {
        #                     die "Failed to setup build.py: $!\n";
        #                 }

        #                 chdir $working_dir
        #                   or die "Unable to change directory: $working_dir\n";
        #             },
        #         );

        #         my $total_checks     = scalar(@prerequisite_checks);
        #         my $completed_checks = 0;

        #         foreach my $check_function (@prerequisite_checks) {
        #             $completed_checks++;
        #             print "[$completed_checks/$total_checks] ";
        #             $check_function->();
        #         }
    }
    else {
        print "Setting up a new vcpkg installation...\n";

        chdir $working_dir or die "Unable to change directory: $working_dir\n";

        my $new_dir    = "Vendor";
        my $vcpkg_repo = "https://github.com/microsoft/vcpkg";

        if ( -d "$new_dir/vcpkg" ) {
            CPMLog::info(
"Destination directory '$new_dir/vcpkg' already exists or is not an empty directory. Skipping vcpkg setup step."
            );
            return;
        }

        unless ( -d $new_dir ) {
            mkdir $new_dir or die "Unable to create directory: $new_dir\n";
        }
        else {
            CPMLog::error("Directory already exists: $new_dir");
        }

        my $git_clone_command = "git clone $vcpkg_repo $new_dir/vcpkg";
        my $exit_status       = system($git_clone_command);

        if ( $exit_status == 0 ) {
            CPMLog::info(
                "Repository cloned successfully into: $working_dir/$new_dir");
        }
        else {
            die "Failed to clone repository: $!\n";
        }

        my $cmd  = "bootstrap-vcpkg.bat";
        my @args = (" -disableMetrics");
        chdir($new_dir) or die "Unable to change directory: $new_dir\n";
        chdir("vcpkg")  or die "Unable to change directory: $new_dir/vcpkg\n";
        my $exit_status2 = system( $cmd, @args );

        if ( $exit_status2 == 0 ) {
            CPMLog::info(
"$working_dir/$new_dir/vcpkg/bootstrap-vcpkg.bat script executed successfully."
            );
        }
        else {
            die "Failed to execute bootstrap-vcpkg.bat script: $!\n";
        }

        chdir $working_dir or die "Unable to change directory: $working_dir\n";

        CPMLog::info("vcpkg setup complete.");

        my $platform_vcpkg_path =
          File::Spec->canonpath("$working_dir/$new_dir/vcpkg");

        $setup_environemnt_cache->put_pair( 'generated_vcpkg_location',
            $platform_vcpkg_path );
        $using_vcpkg_location = JSON::PP::false;
        $setup_environemnt_cache->put_pair( 'using_provided_vcpkg_location',
            $using_vcpkg_location );

        return;

    }
}

# Prerequisite Checks
sub check_git {
    print("Checking if Git is installed...\n");

    my $git_version_output = `git --version`;

    if ( $? == 0 ) {
        if ( $git_version_output =~ /git version (\S+)/ ) {
            my $git_version = $1;

            CPMLog::info("Git is installed. Version: $git_version");
        }
        else {
            die "Unable to extract Git version.\n";
        }
    }
    else {
        die "Git is not installed or an error occurred.\n";
    }
}

sub check_cmake {
    print("Checking if CMake is installed...\n");

    my $cmake_version_output = `cmake --version`;

    if ( $? == 0 ) {
        if ( $cmake_version_output =~ /cmake version (\S+)/ ) {
            my $cmake_version = $1;
            CPMLog::info("CMake is installed. Version: $cmake_version");
        }
        else {
            die "Unable to extract CMake version.\n";
        }
    }
    else {
        die "CMake is not installed or an error occurred.\n";
    }
}

sub check_python {
    print("Checking if Python is installed...\n");

    my $python_version_output = `python --version  2>&1`;

    if ( $? == 0 ) {
        if ( $python_version_output =~ /python (\S+)/i ) {
            my $python_version = $1;
            CPMLog::info("Python is installed. Version: $python_version");
        }
        else {
            die "Unable to extract Python version.\n";
        }
    }
    else {
        die "Python is not installed or an error occurred.\n";
    }
}

sub check_cl {
    print("Checking if MSVC compiler is installed...\n");

    my $cl_output = `cl 2>&1`;

    if ( $? == 0 ) {
        CPMLog::info("MSVC compiler is available.");

        # print "Compiler Output:\n$cl_output";
    }
    else {
        die "MSVC compiler is not installed or an error occurred.\n";
    }
}

sub post_venv {
    print("Running Venv Post-Install Configuration...\n");

    my $PY_VENV_DIR = "venv"
}

# sub setup_venv {
#     print "Setting up venv for usage ...\n";

#     if ( -d $venv_dir ) {
#         print "Directory '$venv_dir' already exists. Clearing...\n";
#         chdir "$working_dir/$venv_dir/Scripts"
#           or die "Unable to change directory: $working_dir/$venv_dir/Scripts\n";
#         my $exit_status = system("deactivate");
#         if ( $exit_status == 0 ) {
#             print "Python venv deactivated successfully.\n";
#         }
#         else {
#             die "Failed to deactivate Python venv: $!\n";
#         }
#         chdir $working_dir or die "Unable to change directory: $working_dir\n";
#         rmdir $venv_dir    or die "Failed to clear directory '$venv_dir': $!\n";
#     }
#     else {
#         mkdir $venv_dir
#           or die "Failed to create directory '$venv_dir': $!\n";
#         print "Directory '$venv_dir' created successfully.\n";
#     }

#     # chdir $venv_dir or die "Unable to change directory: '$venv_dir'\n";

#     my $vcpkg_location = $ARGV[0];

#     if ( defined $vcpkg_location ) {
#         my $path_to_python_interpretor =
#           "$vcpkg_location/installed/x64-windows/tools/python3";
#         chdir $path_to_python_interpretor
#           or die "Unable to change directory: $path_to_python_interpretor\n";
#         my $exit_status = system("python -m venv $working_dir/$venv_dir");
#         if ( $exit_status == 0 ) {
#             print "Python venv initialized successfully.\n";
#         }
#         else {
#             die "Failed to initialize Python venv: $!\n";
#         }
#     }
#     else {
#         my $path_to_python_interpretor =
#           "$working_dir/vendor/vcpkg/installed/x64-windows/tools/python3";
#         chdir $path_to_python_interpretor
#           or die "Unable to change directory: $path_to_python_interpretor\n";
#         my $exit_status = system("python -m venv $working_dir/$venv_dir");
#         if ( $exit_status == 0 ) {
#             print "Python venv initialized successfully.\n";
#         }
#         else {
#             die "Failed to initialize Python venv: $!\n";
#         }
#     }

#     chdir "$working_dir\\$venv_dir\\Scripts"
#       or die "Unable to change directory: $working_dir\\$venv_dir\\Scripts\n";
#     {
#         my $exit_status = system("activate");
#         if ( $exit_status == 0 ) {
#             print "Python venv activated successfully.\n";
#         }
#         else {
#             die "Failed to activate Python venv: $!\n";
#         }
#     }

#     chdir $working_dir or die "Unable to change directory: $working_dir\n";
# }

# sub store_venv_location_to_cache {
#     print "Using build.py to set venv location to cache...\n";

#     chdir "utils" or die "Unable to change directory: utils\n";

#     my $cmd  = "build.bat";
#     my @args = (
#         "-can-vp",
# "venv_root:$working_dir/vendor/vcpkg/installed/x64-windows/tools/python3"
#     );

#     my $exit_status = system( $cmd, @args );
#     if ( $exit_status == 0 ) {
#         print "venv location saved to cache.\n";
#     }
#     else {
#         die "Failed to save venv location to cache: $!\n";
#     }

#     chdir $working_dir
#       or die "Unable to change directory: $working_dir\n";
# }

sub install_vcpkg_package {
    my ( $package, $triplet ) = @_;

    my $package_already_installed =
      check_if_package_installed( $package, $triplet );

    if ( $package_already_installed eq JSON::PP::true ) {
        CPMLog::info("$package:$triplet package is already installed.");
        return;
    }

    my $is_vcpkg_location_provided =
      $setup_environemnt_cache->get_pair('using_provided_vcpkg_location');

    if ( $is_vcpkg_location_provided == JSON::PP::true ) {
        my $provided_vcpkg_location =
          $setup_environemnt_cache->get_pair('provided_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$provided_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {

                # CPMLog::info("$vcpkg_location found.");
                print "Installing $package ($triplet) library...\n";
                my $exit_status =
                  system("$vcpkg_location install $package --triplet=$triplet");
                if ( $exit_status == 0 ) {
                    CPMLog::info("$package package installed successfully.");
                }
                else {
                    die "Failed to install $package package: $!\n";
                }
            }
            else {
                die;
            }
        }
    }
    else {
        my $generated_vcpkg_location =
          $setup_environemnt_cache->get_pair('generated_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$generated_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {

                # CPMLog::info("$vcpkg_location found.");
                print "Installing $package ($triplet) library...\n";
                my $exit_status =
                  system("$vcpkg_location install $package --triplet=$triplet");
                if ( $exit_status == 0 ) {
                    CPMLog::info("$package package installed successfully.");
                }
                else {
                    die "Failed to install $package package: $!\n";
                }
            }
            else {
                die;
            }
        }
    }
}

sub check_if_package_installed {
    my $package = shift;
    my $triplet = shift;

    my $is_vcpkg_location_provided =
      $setup_environemnt_cache->get_pair('using_provided_vcpkg_location');

    if ( $is_vcpkg_location_provided == JSON::PP::true ) {
        my $provided_vcpkg_location =
          $setup_environemnt_cache->get_pair('provided_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$provided_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                print
                  "Checking if $package ($triplet) library is installed...\n";
                my $output = qx($vcpkg_location list $package:$triplet);

                if ( $output =~ /\b\Q$package:$triplet\E\b/ ) {
                    return JSON::PP::true;
                }
                else {
                    return JSON::PP::false;
                }
            }
            else {
                die;
            }
        }
    }
    else {
        my $generated_vcpkg_location =
          $setup_environemnt_cache->get_pair('generated_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$generated_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                print
                  "Checking if $package ($triplet) library is installed...\n";
                my $output = qx($vcpkg_location list $package:$triplet);

                if ( $output =~ /\b\Q$package:$triplet\E\b/ ) {
                    return JSON::PP::true;
                }
                else {
                    return JSON::PP::false;
                }
            }
            else {
                die;
            }
        }
    }
}

sub intergrate_vcpkg {
    print "Configuring vcpkg integration ...\n";

    my $is_vcpkg_location_provided =
      $setup_environemnt_cache->get_pair('using_provided_vcpkg_location');

    if ( $is_vcpkg_location_provided == JSON::PP::true ) {
        my $provided_vcpkg_location =
          $setup_environemnt_cache->get_pair('provided_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$provided_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                my $exit_status = system("$vcpkg_location integrate install");
                if ( $exit_status == 0 ) {
                    CPMLog::info("vcpkg integrated successfully.");
                }
                else {
                    die "Failed to integrate vcpkg: $!\n";
                }
            }
            else {
                die;
            }
        }
    }
    else {
        my $generated_vcpkg_location =
          $setup_environemnt_cache->get_pair('generated_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$generated_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                my $exit_status = system("$vcpkg_location integrate install");
                if ( $exit_status == 0 ) {
                    CPMLog::info("vcpkg integrated successfully.");
                }
                else {
                    die "Failed to integrate vcpkg: $!\n";
                }
            }
            else {
                die;
            }
        }
    }
}

sub configure_package {

    # my ($packages_ref) = @_;
    # my @packages = @{$packages_ref};

    # foreach my $pkg (@packages) {
    #     my ( $library, $triplet ) = @{$pkg}{qw/library triplet/};

    # }
    my ( $library, $triplet ) = @_;

    print "Checking configuration for $library...\n";
    if ( $library eq "boost" ) {
        chdir_and_execute_vcpkg_command(
            "installed/x64-windows/tools/boost-build", "bootstrap" );

        # chdir "$location/installed/x64-windows/tools/boost-build"
        #   or die "Unable to change directory: $location\n";
        # my $exit_status = system("bootstrap");
        # if ( $exit_status == 0 ) {
        #     print "Bootstrapped boost.\n";
        # }
        # else {
        #     die "Failed to bootstrap boost: $!\n";
        # }

    }

#     if ( $library eq "pkgconf" ) {
#         print "Configuring PkgConfing modules...\n";
#         my $is_vcpkg_location_provided =
#           $setup_environemnt_cache->get_pair('using_provided_vcpkg_location');

  #         if ( $is_vcpkg_location_provided == JSON::PP::true ) {
  #             my $provided_vcpkg_location =
  #               $setup_environemnt_cache->get_pair('provided_vcpkg_location');

    #             my $vcpkg_location =
    #               File::Spec->canonpath("$provided_vcpkg_location/vcpkg.exe");

 #             if ( -e $vcpkg_location ) {
 #                 if ( -f $vcpkg_location ) {
 #                     my $exit_status =
 #                       system(
 # "$vcpkg_location install vcpkg-pkgconfig-get-modules --triplet=$triplet"
 #                       );
 #                     if ( $exit_status == 0 ) {
 #                         CPMLog::info("vcpkg integrated successfully.");
 #                     }
 #                     else {
 #                         die "Failed to integrate vcpkg: $!\n";
 #                     }
 #                 }
 #                 else {
 #                     die;
 #                 }
 #             }
 #         }
 #         else {
 #             my $generated_vcpkg_location =
 #               $setup_environemnt_cache->get_pair('generated_vcpkg_location');

   #             my $vcpkg_location =
   #               File::Spec->canonpath("$generated_vcpkg_location/vcpkg.exe");

    #             if ( -e $vcpkg_location ) {
    #                 if ( -f $vcpkg_location ) {
    #                     my $exit_status =
    #                       system(
    # "$vcpkg_location install vcpkg-pkgconfig-get-modules --triplet=$triplet"
    #                       );
    #                     if ( $exit_status == 0 ) {
    #                         CPMLog::info("vcpkg integrated successfully.");
    #                     }
    #                     else {
    #                         die "Failed to integrate vcpkg: $!\n";
    #                     }
    #                 }
    #                 else {
    #                     die;
    #                 }
    #             }
    #         }
    #     }
}

sub execute_vcpkg_command {
    my $command = shift;

    my $is_vcpkg_location_provided =
      $setup_environemnt_cache->get_pair('using_provided_vcpkg_location');

    if ( $is_vcpkg_location_provided == JSON::PP::true ) {
        my $provided_vcpkg_location =
          $setup_environemnt_cache->get_pair('provided_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$provided_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                my $exit_status = system("$command");
                if ( $exit_status == 0 ) {
                    CPMLog::info("vcpkg integrated successfully.");
                }
                else {
                    die "Failed to integrate vcpkg: $!\n";
                }
            }
            else {
                die;
            }
        }
    }
    else {
        my $generated_vcpkg_location =
          $setup_environemnt_cache->get_pair('generated_vcpkg_location');

        my $vcpkg_location =
          File::Spec->canonpath("$generated_vcpkg_location/vcpkg.exe");

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                my $exit_status = system("$command");
                if ( $exit_status == 0 ) {
                    CPMLog::info("vcpkg integrated successfully.");
                }
                else {
                    die "Failed to integrate vcpkg: $!\n";
                }
            }
            else {
                die;
            }
        }
    }
}

sub add_source_directory {

    CPMLog::trace("Adding source directory to options cache.");

    my $options_cache = CPMCache->new();
    $options_cache->init_cache( 'options_cache.json', 0 );

    $options_cache->put_pair( 'source_directory',
        File::Spec->canonpath($working_dir) );

}

sub chdir_and_execute_vcpkg_command {
    my $path    = shift;
    my $command = shift;

    my $is_vcpkg_location_provided =
      $setup_environemnt_cache->get_pair('using_provided_vcpkg_location');

    if ( $is_vcpkg_location_provided == JSON::PP::true ) {
        my $provided_vcpkg_location =
          $setup_environemnt_cache->get_pair('provided_vcpkg_location');

        my $goto_dir = File::Spec->canonpath("$provided_vcpkg_location/$path");

        my $vcpkg_location =
          File::Spec->canonpath("$provided_vcpkg_location/vcpkg.exe");

        chdir $goto_dir or die "Unable to change directory: $goto_dir\n";

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                print "Executing $goto_dir $command...\n";
                my $exit_status = system("$command");
                if ( $exit_status == 0 ) {
                    CPMLog::info("$command executed successfully.");
                }
                else {
                    die "Failed to integrate vcpkg: $!\n";
                }
            }
            else {
                die;
            }
        }

        chdir $working_dir or die "Unable to change directory: $working_dir\n";
    }
    else {
        my $generated_vcpkg_location =
          $setup_environemnt_cache->get_pair('generated_vcpkg_location');

        my $goto_dir = File::Spec->canonpath("$generated_vcpkg_location/$path");

        my $vcpkg_location =
          File::Spec->canonpath("$generated_vcpkg_location/vcpkg.exe");

        chdir $goto_dir or die "Unable to change directory: $goto_dir\n";

        if ( -e $vcpkg_location ) {
            if ( -f $vcpkg_location ) {
                print "Executing $goto_dir $command...\n";
                my $exit_status = system("$command");
                if ( $exit_status == 0 ) {
                    CPMLog::info("$command executed successfully.");
                }
                else {
                    die "Failed to integrate vcpkg: $!\n";
                }
            }
            else {
                die;
            }
        }

        chdir $working_dir or die "Unable to change directory: $working_dir\n";
    }
}

sub run_no_local_vcpkg {
    {
        print("no-local-vcpkg\n");
    }
}

sub run_vcpkg_location {

    # my $vcpkg_location = shift;
    # my @prerequisites  = @_;

    my ( $vcpkg_location, $prerequisites_ref,
        $packages_ref, $post_install_ref, $skip_package_configurations )
      = @_;

    my @prerequisites = @{$prerequisites_ref};
    my @packages      = @{$packages_ref};
    my @post_install  = @{$post_install_ref};

    my @prerequisite_checks;

    # my @prerequisite_checks = (
    #     \&check_git,    \&check_cmake,
    #     \&check_python, \&check_msvc_compiler,

    #     # \&vcpkg_setup,      \&vcpkg_package_1,
    #     # \&vcpkg_package_2,  \&vcpkg_package_3,
    #     # \&vcpkg_package_4,  \&vcpkg_package_5,
    #     # \&vcpkg_package_6,  \&vcpkg_package_7,
    #     # \&vcpkg_package_8,  \&vcpkg_package_9,
    #     # \&vcpkg_package_10, \&configure_package_4,
    #     # \&vcpkg_integrate,  \&purge_previous_options_cache,
    #     # \&setup_build_script,
    # );

    # foreach my $pre (@prerequisites) {
    #     push @prerequisite_checks, sub { check_prerequisite($pre) };
    # }

    push @prerequisite_checks, sub { vcpkg_setup($vcpkg_location) };

    foreach my $pre (@prerequisites) {
        my $check_function_name = "check_$pre";
        if ( my $coderef = __PACKAGE__->can($check_function_name) ) {
            push @prerequisite_checks, $coderef;
        }
        else {
            warn "No check function for $pre";
        }
    }

    # push @prerequisite_checks, sub { vcpkg_setup($vcpkg_location) };

    foreach my $pkg (@packages) {
        my ( $library, $triplet ) = @{$pkg}{qw/library triplet/};
        push @prerequisite_checks,
          sub { install_vcpkg_package( $library, $triplet ) };
    }

    push @prerequisite_checks, \&intergrate_vcpkg;

    if ( $skip_package_configurations eq JSON::PP::false ) {
        foreach my $pkg (@packages) {
            my ( $library, $triplet ) = @{$pkg}{qw/library triplet/};
            push @prerequisite_checks,
              sub { configure_package( $library, $triplet ) };
        }
    }
    else {
        print "Skipping package configurations...\n";
    }

    # push @prerequisite_checks, sub { configure_packages($packages_ref) };

    foreach my $post (@post_install) {
        my $check_function_name = "post_$post";
        if ( my $coderef = __PACKAGE__->can($check_function_name) ) {
            push @prerequisite_checks, $coderef;
        }
        else {
            warn "No check function for $post";
        }
    }

    push @prerequisite_checks, \&add_source_directory;

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;

        # print_col( $GREEN, "[$completed_checks/$total_checks] " );
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub run_do_deps_check() {
    return 0;
}

sub setup_help {
    return CPMHelpText::setup_help();
}

1;
