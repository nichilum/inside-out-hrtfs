{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      nixpkgs,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            # config = {
            #   cudaSupport = true;
            #   allowUnfree = true;
            # };
            # overlays = [ ];
          };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              uv
              python313
              stdenv.cc
              stdenv.cc.cc.lib
              zlib
              pkg-config
            ];
            env = {
              UV_PYTHON = "${pkgs.python313}/bin/python3";
              UV_PYTHON_DOWNLOADS = "never";
            };
            LD_LIBRARY_PATH =
              pkgs.lib.makeLibraryPath [
                pkgs.stdenv.cc.cc.lib
                pkgs.zlib
              ]
              + ":/run/opengl-driver/lib"; # https://wiki.nixos.org/wiki/OpenGL
          };
        }
      );
    };
}
