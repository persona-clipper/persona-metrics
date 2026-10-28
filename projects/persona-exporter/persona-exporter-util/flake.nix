{
  description = "Metrics Exporter for Influx DB / Victoria Metrics";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      ...
    }:
    let
      # общие данные, доступные и в packages, и в nixosModules
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      projectName = cargoToml.package.name;
      projectVersion = cargoToml.package.version;
    in
    (utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = projectName;
          version = projectVersion;
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
          cargoHash = pkgs.lib.fakeHash;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ];

          meta = with pkgs.lib; {
            description = "Metrics exporter for InfluxDB / Victoria Metrics written in Rust";
            homepage = "https://github.com/Persona-Team/persona-exporter";
            license = licenses.mit;
            mainProgram = projectName;
          };
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    ))
    // {
      nixosModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        let
          cfg = config.services.persona-exporter;
        in
        {
          options.services.persona-exporter = {
            enable = lib.mkEnableOption "persona-exporter";
            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
              description = "The persona-exporter package to use.";
            };
          };

          config = lib.mkIf cfg.enable {
            users.users.persona-exp = {
              isSystemUser = true;
              group = "persona-exp";
              description = "Persona Exporter Service";
            };
            users.groups.persona-exp = { };

            systemd.services.persona-exporter = {
              description = "Persona Exporter";
              after = [ "network.target" ];
              wantedBy = [ "multi-user.target" ];

              serviceConfig = {
                User = "persona-exp";
                Group = "persona-exp";
                ConfigurationDirectory = "persona-exporter";
                ExecStart = "${cfg.package}/bin/${projectName}";
                Restart = "always";
                RestartSec = "5s";
                MemoryMax = "30M";
                MemoryHigh = "30M";
                CPUQuota = "5%";
                ProtectSystem = "full";
                ProtectHome = true;
                PrivateTmp = true;
                NoNewPrivileges = true;
              };
            };
          };
        };
    };
}
