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
    return CPMHelpText::generate_help;
}

sub setup_help {
    return CPMHelpText::setup_help;
}

sub build_help {
    return CPMHelpText::build_help;
}

1;
