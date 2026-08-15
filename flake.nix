{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      cargoConfig = (fromTOML ./Cargo.toml);

      # dependencies for building sdl3
      # sdlPkgs = with pkgs; [
      #   pkg-config
      #   just
      #   cmake
      #   validatePkgConfig
      #   gcc
      #   wayland-scanner
      #   zenity
      #   libffi
      #   python313
      #   patchelf
      #   vulkan-headers
      #   vulkan-loader
      #   libGL
      #   libusb1
      #   libayatana-appindicator
      #   libdrm
      #   mesa
      #   wayland
      #   wayland-protocols
      #   pipewire
      #   libpulseaudio
      #   alsa-lib
      #   dbus
      #   libxtst
      #   libxcb
      #   libxkbcommon
      #   libx11
      #   libxscrnsaver
      #   libxcursor
      #   libxext
      #   libxfixes
      #   libxi
      #   libxrandr
      #
      #   # SDL_TTF
      #   freetype
      # ];
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = cargoConfig.package.name;
        version = cargoConfig.package.version;

        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          rust-analyzer
          pre-commit
          rustPackages.clippy

          pkg-config
          sdl3
          sdl3-ttf
          libevdev
        ];
      };
    };
}
