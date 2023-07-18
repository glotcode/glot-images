{
  pkgs,
  name,
  tag,
  installedPackages,
  run ? "",
  env ? [],
  keepContentsDirlinks ? false
}:

let
  codeRunner =
    import ./coderunner.nix { pkgs = pkgs; };

  commonPackages = [
    pkgs.bash
    pkgs.coreutils
  ];

  commonEnv = [
    "LANG=C.UTF-8"
  ];
in
pkgs.dockerTools.buildImage {
  name = name;
  tag = tag;
  created = "now";

  contents =
    pkgs.lib.concatLists [
      commonPackages
      installedPackages
    ];

  diskSize = 8192;
  keepContentsDirlinks = keepContentsDirlinks;

  runAsRoot = ''
    ${pkgs.stdenv.shell}
    ${pkgs.dockerTools.shadowSetup}
    ${pkgs.shadow}/bin/groupadd glot
    ${pkgs.shadow}/bin/useradd -d /home/glot -g glot -s /bin/bash -u 11223 glot
    ${run}
  '';

  config = {
    Env =
      pkgs.lib.concatLists [
        commonEnv
        env
      ];

    Cmd = [ "${codeRunner}/bin/code-runner" "--path" "/home/glot"];
  };
}
