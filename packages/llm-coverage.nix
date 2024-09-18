{
  inputs,
  pkgs,
  system,
  ...
}: let
  inherit (pkgs) lib;
  craneLib = inputs.crane.mkLib pkgs;
  src = craneLib.cleanCargoSource ../.;

  craneLibLLvmTools =
    craneLib.overrideToolchain
    (inputs.fenix.packages.${system}.complete.withComponents [
      "cargo"
      "llvm-tools"
      "rustc"
    ]);

  # Common arguments can be set here to avoid repeating them later
  commonArgs = {
    inherit src;
    strictDeps = true;

    buildInputs =
      [
        # Add additional build inputs here
        pkgs.nixos-rebuild
        pkgs.age
        pkgs.ssh-to-age
      ]
      ++ lib.optionals pkgs.stdenv.isDarwin [
        # Additional darwin specific inputs can be set here
        pkgs.libiconv
      ];

    # Additional environment variables can be set directly
    # MY_CUSTOM_VAR = "some value";
  };

  # Build *just* the cargo dependencies, so we can reuse
  # all of that work (e.g. via cachix) when running in CI
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
  craneLibLLvmTools.cargoLlvmCov (commonArgs
    // {
      inherit cargoArtifacts;
    })
# TODO: how do we constrain this package to linux only without constraing the main package?
#lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
#my-crate-llvm-coverage =
#craneLibLLvmTools.cargoLlvmCov (commonArgs
#// {
#inherit cargoArtifacts;
#})
#};

