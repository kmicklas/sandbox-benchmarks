let source = import ./nix/sources.nix { };

in with import source.nixpkgs { };

mkShell { packages = [ bubblewrap gnuplot niv podman ]; }
