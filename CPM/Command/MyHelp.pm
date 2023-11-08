# #!/usr/bin/perl
use strict;
use warnings;

package CPM::Command::MyHelp;
use base qw(CLI::Framework::Command::Help);

use lib "$main::CoreDir\\Lib";
use CPMHelpText;

sub run {
    my $self = shift;
    my $opts = shift;
    my @args = @_;

    if (@args) {
        my $topic = $args[0];
        if ( $topic eq 'generate' ) {
            return $self->generate_help;
        }
        elsif ( $topic eq 'setup' ) {
            return $self->setup_help;
        }
        elsif ( $topic eq 'build' ) {
            return $self->build_help;
        }
    }

    # Fall back to the default help text if no arguments were provided,
    # or if an unknown argument was provided.
    return $self->SUPER::run(@_);
}

sub generate_help {
    return qq{
    Usage: $0 generate <verb> <args> ...

};
}

sub setup_help {
    return CPMHelpText::setup_help;

    #     return qq{
    #     Usage: $0 setup <verb> <args> ...

#     \t# Tries to find an existing vcpkg installation otherwise runs the setup without using a local vcpkg installation. Optionally skips package configurations.
#     \t<n|nlv|no-local-vcpkg>

#     \t# Runs the setup with the specified vcpkg directory. Optionally skips package configurations.
#     \t<l|vl|vcpkg-location> [<path/to/vcpkg>] [spc|skip_package_configurations]

#     \t# Forces vcpkg to install packages again (does not remove any existing packages).
#     \t<fpi|force_package_install>

#     \t# Runs the setup without checking for runtime dependencies (ONLY FOR CI USE).
#     \t<ndc|no-deps-check>

    # };
}

sub build_help {
    return CPMHelpText::build_help;
}

# ... other help methods ...

1;

# package CPM::Command::MyHelp;
# use base qw(CLI::Framework::Command::Help);

# sub option_spec {
#     [ 'all|a' => 'display all help.' ],
#       [ 'generate|g' => 'display generation help.' ],
#       [ 'setup|s'    => 'display setup help.' ],
#       [ 'build|b'    => 'display build help.' ],;
# }

# sub run {
#     my $self = shift;
#     my $opts = shift;
#     my @args = @_;

#     if ( scalar(@args) == 0 ) {
#         return core_help();
#     }
#     if ( $opts->{all} ) {
#         print( core_help() );
#         print( generation_help() );
#         print( setup_help() );
#         print( build_help() );
#     }
#     else if ( $opts->{generate} ) {
#         return core_help();
#     }
#     else if ( $opts->{setup} ) {
#         return core_help();
#     }
#     else if ( $opts->{build} ) {
#         return core_help();
#     }
#     else {
#         return core_help();
#     }
# }

# sub core_help {
#     return qq{
#     Usage: $0 <verb> <args> ...

#     \t# Help (this)
#     \t<h|help>

#     \t# Generate
#     \t<g|generate> <name> <path>

#     \t# Setup
#     \t<s|setup> <name> <path>

#     \t# Display build help
#     \t<bh|build-help> <name> <path>

#     Options:
#       -h, --help\tHow to use this application

# };
# }

# sub generation_help {
#     return qq{

# };
# }

# sub setup_help {
#     return qq{

# };
# }

# sub build_help {
#     return qq{
#     Usage: $0 build <verb> <args> ...

#     \t# Help (this).
#     \t<h|help>

#     \t# Runs the setup without using a local vcpkg installation.
#     \t<nlv|no-local-vcpkg>

#     \t# Runs the setup with the specified vcpkg directory.
#     \t<vl|vcpkg-location> <path/to/vcpkg>

#     \t# Runs the setup without checking for runtime dependencies (ONLY FOR CI USE)/
#     \t<ndc|no-deps-check>
# };
# }

# 1;
