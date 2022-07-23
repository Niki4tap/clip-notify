{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		xorg.libxcb
		alsa-lib
		pkg-config
	];
}
