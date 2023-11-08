#!/usr/bin/perl
package CPMCache;

use strict;
use warnings;

use Cache::FileCache;
use JSON::XS;
use FindBin;
use File::Spec;

sub new {
    my $class = shift;
    my $self  = { cache => undef, };
    bless $self, $class;
    return $self;
}

sub init_cache {
    my ( $self, $filename, $overwrite ) = @_;
    my $this_file_dir = $main::CoreDir;
    my $json_file_dir = File::Spec->catdir( $this_file_dir, 'Cache' );
    $self->{cache} = File::Spec->catfile( $json_file_dir, $filename );

    if ( -e $self->{cache} ) {
        if ($overwrite) {
            unlink $self->{cache} or die "Could not unlink '$self->{cache}' $!";
        }
        else {
            return
"File '$self->{cache}' already exists. Use overwrite flag to overwrite it.";
        }
    }

    open my $fh, '>', $self->{cache}
      or die "Could not open '$self->{cache}' $!";
    close $fh;

    return "Cache initialized at '$self->{cache}'";
}

sub init_cache_from_path {
    my ( $self, $absolute_path ) = @_;

    unless ( -e $absolute_path && -f _ ) {
        die "Cache file does not exist at '$absolute_path'";
    }

    $self->{cache} = $absolute_path;

    return "Cache initialized with existing file at '$self->{cache}'";
}

sub get_pair {
    my ( $self, $key ) = @_;
    return unless $self->{cache} && $key;

    if ( -e $self->{cache} ) {
        open my $fh, '<', $self->{cache}
          or die "Could not open '$self->{cache}' $!";
        local $/;
        my $json_text = <$fh>;
        close $fh;

        my $data = decode_json($json_text);
        return $data->{$key} if exists $data->{$key};
    }
    return undef;
}

sub get_all {
    my $self = shift;
    return unless $self->{cache};

    if ( -e $self->{cache} ) {
        open my $fh, '<', $self->{cache}
          or die "Could not open '$self->{cache}' $!";
        local $/;
        my $json_text = <$fh>;
        close $fh;

        return $json_text ? decode_json($json_text) : undef;
    }
    return undef;
}

sub put_pair {
    my ( $self, $key, $value ) = @_;
    return unless $self->{cache};

    my $data = $self->get_all() || {};
    $data->{$key} = $value;

    open my $fh, '>', $self->{cache}
      or die "Could not open '$self->{cache}' $!";
    print $fh encode_json($data);
    close $fh;
}

sub try_get_pair {
    my ( $self, $key ) = @_;
    return unless defined $key;

    if ( -e $self->{cache} ) {
        open my $fh, '<', $self->{cache}
          or die "Could not open '$self->{cache}' $!";
        local $/;
        my $json_text = <$fh>;
        close $fh;

        if ( defined $json_text && $json_text ne '' ) {
            my $data = eval { decode_json($json_text) };
            if ($@) {
                warn "Failed to decode JSON: $@";
                return ( 0, undef );
            }

            if ( exists $data->{$key} ) {
                return ( 1, $data->{$key} );
            }
        }
    }
    return ( 0, undef );
}

1;
