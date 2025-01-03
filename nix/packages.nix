{pkgs, ...}: rec {
  my-package = pkgs.rustPlatform.buildRustPackage {
    pname = "my-binary";
    version = "0.1.0";

    /*
    nativeBuildInputs = with pkgs; [
    pkg-config
    clang
    ];

    buildInputs = with pkgs; [
    ];
    */

    src = ./..;
    hash = pkgs.lib.fakeHash;
    cargoHash = "sha256-tE3ECVyEkTXa9zqXypoNKo2D4bApVPs9vFwGesQjQvY=";
  };

  default = my-package;
}
