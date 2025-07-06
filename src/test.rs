use std::path::Path;

use crate::{Config, command_builder::Executer, options::ToSwitch, switch};

#[test]
fn system_switch() {
    let mut output = Vec::new();

    switch(
        &Config {
            identity: "test_identity".into(),
            nix_path: Path::new("/path/to/flake.nix").into(),
        },
        &[ToSwitch::System { offline: false }],
        true,
        Executer::new(true, &mut output),
    )
    .expect("Unable to run test commands.");

    let binding = String::from_utf8(output).expect("Output contained non-utf8 chars.");
    let mut outputs = binding.split_terminator('\n');

    assert_eq!(
        outputs.next().unwrap(),
        "echo 'Sudo perms required for system rebuild.'"
    );
    assert_eq!(
        outputs.next().unwrap(),
        "sudo echo 'Sudo perms given for system rebuild.'"
    );
    assert_eq!(
        outputs.next().unwrap(),
        "nix flake update --flake /path/to/flake.nix"
    );
    assert_eq!(
        outputs.next().unwrap(),
        "sudo nixos-rebuild --option experimental-features 'nix-command flakes pipe-operators' switch --flake /path/to/flake.nix#test_identity"
    );

    assert!(outputs.next().is_none());
}

#[test]
fn system_switch_no_update() {
    let mut output = Vec::new();

    switch(
        &Config {
            identity: "test_identity".into(),
            nix_path: Path::new("/path/to/flake.nix").into(),
        },
        &[ToSwitch::System { offline: false }],
        false,
        Executer::new(true, &mut output),
    )
    .expect("Unable to run test commands.");

    let binding = String::from_utf8(output).expect("Output contained non-utf8 chars.");
    let mut outputs = binding.split_terminator('\n');

    assert_eq!(
        outputs.next().unwrap(),
        "echo 'Sudo perms required for system rebuild.'"
    );
    assert_eq!(
        outputs.next().unwrap(),
        "sudo echo 'Sudo perms given for system rebuild.'"
    );
    assert_eq!(
        outputs.next().unwrap(),
        "sudo nixos-rebuild --option experimental-features 'nix-command flakes pipe-operators' switch --flake /path/to/flake.nix#test_identity"
    );

    assert!(outputs.next().is_none());
}

#[test]
fn home_switch() {
    let mut output = Vec::new();

    switch(
        &Config {
            identity: "test_identity".into(),
            nix_path: Path::new("/path/to/flake.nix").into(),
        },
        &[ToSwitch::Home],
        true,
        Executer::new(true, &mut output),
    )
    .expect("Unable to run test commands.");

    let binding = String::from_utf8(output).expect("Output contained non-utf8 chars.");
    let mut outputs = binding.split_terminator('\n');

    assert_eq!(
        outputs.next().unwrap(),
        "nix flake update --flake /path/to/flake.nix"
    );
    assert_eq!(
        outputs.next().unwrap(),
        "home-manager switch --flake /path/to/flake.nix#test_identity"
    );

    assert!(outputs.next().is_none());
}

#[test]
fn home_switch_no_update() {
    let mut output = Vec::new();

    switch(
        &Config {
            identity: "test_identity".into(),
            nix_path: Path::new("/path/to/flake.nix").into(),
        },
        &[ToSwitch::Home],
        false,
        Executer::new(true, &mut output),
    )
    .expect("Unable to run test commands.");

    let binding = String::from_utf8(output).expect("Output contained non-utf8 chars.");
    let mut outputs = binding.split_terminator('\n');

    assert_eq!(
        outputs.next().unwrap(),
        "home-manager switch --flake /path/to/flake.nix#test_identity"
    );

    assert!(outputs.next().is_none());
}
