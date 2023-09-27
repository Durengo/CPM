use strict;
use warnings;
use Getopt::Long qw(GetOptions);
use Cwd;
use Data::Dumper;

my $working_dir = getcwd();
chdir $working_dir or die "Unable to change directory: $working_dir\n";

my %options;
my %option_actions = (
    "help"           => sub { print_help(); },
    "vcpkg-location" => sub { vcpkg_location(); },
    "no-deps-check"  => sub { core_setup_no_dep_check(); },
);

GetOptions( \%options, keys %option_actions )
  or die("Error in command line arguments\n");

# print Dumper(\%options);

# Process options dynamically based on the option_actions hash
foreach my $option ( keys %options ) {
    if ( defined $options{$option} && exists $option_actions{$option} ) {
        $option_actions{$option}->( $options{$option} );
    }
}

# If no options provided, provide default behavior or instructions
unless ( keys %options ) {
    print "Running default setup parameters.\n";
    core_setup();
}

sub print_help {
    print "Usage:
    $0 [] - runs the setup
    $0 [--help] - shows help
    $0 [--vcpkg-location <path/to/vcpkg>] - runs the setup with the specified vcpkg directory
    $0 [--no-deps-check] - runs the setup without checking for runtime dependencies (ONLY FOR CI USE)
    ";
    exit;
}

sub vcpkg_location {
    my $location = $ARGV[0];

    # my ($location) = @_;

    if ( defined $location ) {

        my $check_path = "$location";

        if ( -e $check_path ) {
            if ( -d $check_path ) {
                print "$check_path is a directory and it exists.\n";
            }
            elsif ( -f $check_path ) {
                die
"$check_path is a file and it exists. Provide vcpkg location.\n";
            }
            else {
                die
                  "$check_path exists but is neither a file nor a directory.\n";
            }
        }
        else {
            die "$check_path does not exist.\n";
        }

        my @prerequisite_checks = (
            \&check_git,
            \&check_cmake,
            \&check_python,
            \&check_msvc_compiler,
            sub {
                print "Installing fmt library...\n";
                my $exit_status =
                  system("vcpkg install fmt --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "fmt library installed successfully.\n";
                }
                else {
                    die "Failed to install fmt library: $!\n";
                }
            },
            sub {
                print "Installing spdlog library...\n";
                my $exit_status =
                  system("vcpkg install spdlog --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "spdlog library installed successfully.\n";
                }
                else {
                    die "Failed to install spdlog library: $!\n";
                }
            },
            sub {
                print "Installing stb library...\n";
                my $exit_status =
                  system("vcpkg install stb --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "stb library installed successfully.\n";
                }
                else {
                    die "Failed to install stb library: $!\n";
                }
            },
            sub {
                print "Installing pkgconf library...\n";
                my $exit_status =
                  system("vcpkg install pkgconf --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "pkgconf library installed successfully.\n";
                }
                else {
                    die "Failed to install pkgconf library: $!\n";
                }
            },
            sub {
                print "Installing raylib library...\n";
                my $exit_status =
                  system("vcpkg install raylib --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "raylib library installed successfully.\n";
                }
                else {
                    die "Failed to install raylib library: $!\n";
                }
            },
            sub {
                print "Installing gtest library...\n";
                my $exit_status =
                  system("vcpkg install gtest --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "gtest library installed successfully.\n";
                }
                else {
                    die "Failed to install gtest library: $!\n";
                }
            },
            sub {
                print "Installing boost library...\n";
                my $exit_status =
                  system("vcpkg install boost --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "boost library installed successfully.\n";
                }
                else {
                    die "Failed to install boost library: $!\n";
                }
                chdir $working_dir
                  or die "Unable to change directory: $working_dir\n";
            },
            sub {
                print "Installing boost-python[python3] library...\n";
                my $exit_status = system(
                    "vcpkg install boost-python[python3] --triplet=x64-windows"
                );
                if ( $exit_status == 0 ) {
                    print
                      "boost-python[python3] library installed successfully.\n";
                }
                else {
                    die "Failed to install boost-python[python3] library: $!\n";
                }
            },
            sub {
                print "Installing gtkmm library...\n";
                my $exit_status =
                  system("vcpkg install gtkmm --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "gtkmm library installed successfully.\n";
                }
                else {
                    die "Failed to install gtkmm library: $!\n";
                }
            },
            sub {
                print "Installing opengl library...\n";
                my $exit_status =
                  system("vcpkg install opengl --triplet=x64-windows");
                if ( $exit_status == 0 ) {
                    print "opengl library installed successfully.\n";
                }
                else {
                    die "Failed to install opengl library: $!\n";
                }
            },
            sub {
                print "Configuring PkgConfing modules...\n";
                my $exit_status = system(
"vcpkg install vcpkg-pkgconfig-get-modules --triplet=x64-windows"
                );
                if ( $exit_status == 0 ) {
                    print "pkgconfig modules configure successfully.\n";
                }
                else {
                    die "Failed to configure pkgconfig modules: $!\n";
                }
            },
            sub {
                print "Configuring vcpkg integration ...\n";
                {
                    my $exit_status = system("vcpkg integrate install");
                    if ( $exit_status == 0 ) {
                        print "pkgconfig modules configure successfully.\n";
                    }
                    else {
                        die "Failed to configure pkgconfig modules: $!\n";
                    }
                }
                {
                    my $exit_status = system("vcpkg integrate powershell");
                    if ( $exit_status == 0 ) {
                        print "pkgconfig modules configure successfully.\n";
                    }
                    else {
                        die "Failed to configure pkgconfig modules: $!\n";
                    }
                }

               # {
               #     my $exit_status = system("vcpkg bootsrap python");
               #     if ( $exit_status == 0 ) {
               #         print "pkgconfig modules configure successfully.\n";
               #     }
               #     else {
               #         die "Failed to configure pkgconfig modules: $!\n";
               #     }
               # }
               # {
               #     my $exit_status = system("vcpkg bootsrap python external");
               #     if ( $exit_status == 0 ) {
               #         print "pkgconfig modules configure successfully.\n";
               #     }
               #     else {
               #         die "Failed to configure pkgconfig modules: $!\n";
               #     }
               # }
                {
                    chdir "$location/installed/x64-windows/tools/boost-build"
                      or die "Unable to change directory: $location\n";
                    my $exit_status = system("bootstrap");
                    if ( $exit_status == 0 ) {
                        print "Bootstrapped boost.\n";
                    }
                    else {
                        die "Failed to bootstrap boost: $!\n";
                    }

          # chdir "$location/installed/x64-windows/tools/boost-build/src/engine"
          #   or die "Unable to change directory: $location\n";
          # my $exit_status = system("b2 --prefix=$location");
          # if ( $exit_status == 0 ) {
          #     print "Installed boostrap engine.\n";
          # }
          # else {
          #     die "Failed to install bootstrap engine: $!\n";
          # }
                }

                chdir $working_dir
                  or die "Unable to change directory: $working_dir\n";
            },
            \&purge_previous_options_cache,
            sub {
                print "Setting up build.py for internal use...\n";

                chdir "utils" or die "Unable to change directory: utils\n";

                my $cmd  = "build.bat";
                my @args = ( "-cg", "$location" );

                my $exit_status = system( $cmd, @args );
                if ( $exit_status == 0 ) {
                    print "build.py setup successfully.\n";
                }
                else {
                    die "Failed to setup build.py: $!\n";
                }

                chdir $working_dir
                  or die "Unable to change directory: $working_dir\n";
            },
        );

        my $total_checks     = scalar(@prerequisite_checks);
        my $completed_checks = 0;

        foreach my $check_function (@prerequisite_checks) {
            $completed_checks++;
            print "[$completed_checks/$total_checks] ";
            $check_function->();
        }

    }
    else {
        die "Error: --vcpkg-location requires an argument.\n";
    }

    print "Successfully ran setup.pl with vcpkg location: $location\n";
    exit;
}

sub core_setup_no_dep_check {
    my @prerequisite_checks = (
        \&vcpkg_setup,      \&vcpkg_package_1,
        \&vcpkg_package_2,  \&vcpkg_package_3,
        \&vcpkg_package_4,  \&vcpkg_package_5,
        \&vcpkg_package_6,  \&vcpkg_package_7,
        \&vcpkg_package_8,  \&vcpkg_package_9,
        \&vcpkg_package_10, \&configure_package_4,
        \&vcpkg_integrate,  \&purge_previous_options_cache,
        \&setup_build_script,
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub core_setup {
    my @prerequisite_checks = (
        \&check_git,        \&check_cmake,
        \&check_python,     \&check_msvc_compiler,
        \&vcpkg_setup,      \&vcpkg_package_1,
        \&vcpkg_package_2,  \&vcpkg_package_3,
        \&vcpkg_package_4,  \&vcpkg_package_5,
        \&vcpkg_package_6,  \&vcpkg_package_7,
        \&vcpkg_package_8,  \&vcpkg_package_9,
        \&vcpkg_package_10, \&configure_package_4,
        \&vcpkg_integrate,  \&purge_previous_options_cache,
        \&setup_build_script,
    );

    my $total_checks     = scalar(@prerequisite_checks);
    my $completed_checks = 0;

    foreach my $check_function (@prerequisite_checks) {
        $completed_checks++;
        print "[$completed_checks/$total_checks] ";
        $check_function->();
    }
}

sub purge_previous_options_cache {
    my $file_remove = "options_cache.json";

    chdir "utils" or die "Unable to change directory: utils\n";

    if ( -e $file_remove ) {
        unlink $file_remove or die "Unable to remove $file_remove: $!";
        print "File $file_remove removed successfully.\n";
    }
    else {
        print "File $file_remove does not exist.\n";
    }

    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub setup_build_script {
    print "Setting up build.py for internal use...\n";

    chdir "utils" or die "Unable to change directory: utils\n";

    my $cmd  = "build.bat";
    my @args = ( "-cg", "$working_dir/vendor/vcpkg" );

    my $exit_status = system( $cmd, @args );
    if ( $exit_status == 0 ) {
        print "build.py setup successfully.\n";
    }
    else {
        die "Failed to setup build.py: $!\n";
    }

    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_setup {
    print "Setting up vcpkg...\n";

    my $new_dir    = "vendor";
    my $vcpkg_repo = "https://github.com/microsoft/vcpkg";

    if ( -d "$new_dir/vcpkg" ) {
        print
"Destination directory '$new_dir/vcpkg' already exists and is not an empty directory.\nSkipping vcpkg setup step.\n";
        return;
    }

    unless ( -d $new_dir ) {
        mkdir $new_dir or die "Unable to create directory: $new_dir\n";
        print "Created directory: $new_dir\n";
    }
    else {
        print "Directory already exists: $new_dir\n";
    }

    my $git_clone_command = "git clone $vcpkg_repo $new_dir/vcpkg";
    my $exit_status       = system($git_clone_command);

    if ( $exit_status == 0 ) {
        print "Repository cloned successfully into: $new_dir\n";
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
        print "bootstrap-vcpkg.bat script executed successfully.\n";
    }
    else {
        die "Failed to execute bootstrap-vcpkg.bat script: $!\n";
    }

    chdir $working_dir or die "Unable to change directory: $working_dir\n";

    print "vcpkg setup complete.\n";
}

sub vcpkg_package_1 {
    print "Installing fmt library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install fmt --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "fmt library installed successfully.\n";
    }
    else {
        die "Failed to install fmt library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_2 {
    print "Installing spdlog library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install spdlog --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "spdlog library installed successfully.\n";
    }
    else {
        die "Failed to install spdlog library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_3 {
    print "Installing stb library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install stb --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "stb library installed successfully.\n";
    }
    else {
        die "Failed to install stb library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_4 {
    print "Installing pkgconf library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install pkgconf --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "pkgconf library installed successfully.\n";
    }
    else {
        die "Failed to install pkgconf library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_5 {
    print "Installing raylib library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install raylib --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "raylib library installed successfully.\n";
    }
    else {
        die "Failed to install raylib library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_6 {
    print "Installing gtest library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install gtest --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "gtest library installed successfully.\n";
    }
    else {
        die "Failed to install gtest library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_7 {
    print "Installing boost library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install boost --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "boost library installed successfully.\n";
    }
    else {
        die "Failed to install boost library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_8 {
    print "Installing boost-python[python3] library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status =
      system("vcpkg install boost-python[python3] --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "boost-python[python3] library installed successfully.\n";
    }
    else {
        die "Failed to install boost-python[python3] library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_9 {
    print "Installing gtkmm library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install gtkmm --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "gtkmm library installed successfully.\n";
    }
    else {
        die "Failed to install gtkmm library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_package_10 {
    print "Installing opengl library...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status = system("vcpkg install opengl --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "opengl library installed successfully.\n";
    }
    else {
        die "Failed to install opengl library: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub configure_package_4 {
    print "Configuring PkgConfing modules...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    my $exit_status =
      system("vcpkg install vcpkg-pkgconfig-get-modules --triplet=x64-windows");
    if ( $exit_status == 0 ) {
        print "pkgconfig modules configure successfully.\n";
    }
    else {
        die "Failed to configure pkgconfig modules: $!\n";
    }
    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

sub vcpkg_integrate {
    print "Configuring vcpkg integration ...\n";
    chdir "vendor/vcpkg" or die "Unable to change directory: vendor/vcpkg\n";
    {
        my $exit_status = system("vcpkg integrate install");
        if ( $exit_status == 0 ) {
            print "pkgconfig modules configure successfully.\n";
        }
        else {
            die "Failed to configure pkgconfig modules: $!\n";
        }
    }
    {
        my $exit_status = system("vcpkg integrate powershell");
        if ( $exit_status == 0 ) {
            print "pkgconfig modules configure successfully.\n";
        }
        else {
            die "Failed to configure pkgconfig modules: $!\n";
        }
    }

    chdir $working_dir or die "Unable to change directory: $working_dir\n";
}

# Prerequisite Checks
# TODO: Add stricter version checking.
sub check_git {
    print("Checking if Git is installed...\n");

    my $git_version_output = `git --version`;

    if ( $? == 0 ) {
        if ( $git_version_output =~ /git version (\S+)/ ) {
            my $git_version = $1;

            print "Git is installed. Version: $git_version\n";
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
            print "CMake is installed. Version: $cmake_version\n";
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

    my $python_version_output = `python --version 2>&1`;

    if ( $? == 0 ) {
        if ( $python_version_output =~ /python (\S+)/i ) {
            my $python_version = $1;
            print "Python is installed. Version: $python_version\n";
        }
        else {
            die "Unable to extract Python version.\n";
        }
    }
    else {
        die "Python is not installed or an error occurred.\n";
    }
}

sub check_msvc_compiler {
    print("Checking if MSVC compiler is installed...\n");

    my $cl_output = `cl 2>&1`;

    if ( $? == 0 ) {
        print "MSVC compiler is available.\n";

        # print "Compiler Output:\n$cl_output";
    }
    else {
        die "MSVC compiler is not installed or an error occurred.\n";
    }
}

