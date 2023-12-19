#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use std::{env, fs, path::Path};

use rustc_ast::Crate;
use rustc_lint::{EarlyContext, EarlyLintPass};
use scout_audit_internal::Detector;
use semver::*;

dylint_linting::declare_early_lint! {
    /// ### What it does
    /// Checks the soroban version of the contract
    ///
    /// ### Why is this bad?
    /// Using an outdated version of soroban could lead to security vulnerabilities, bugs, and other issues.
    pub CHECK_SOROBAN_VERSION,
    Warn,
    Detector::SorobanVersion.get_lint_message()
}

impl EarlyLintPass for CheckSorobanVersion {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, _: &Crate) {
        let latest_version = get_version();
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
        let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Unable to read Cargo.toml");
        let toml: toml::Value = toml::from_str(&cargo_toml).unwrap();
        let soroban_version = match toml
            .get("dependencies")
            .and_then(|d| d.get("soroban-sdk").and_then(|i| i.get("version")))
        {
            Some(version) => version.to_string(),
            None => return,
        };

        let req = Version::parse(&latest_version.replace('\"', "")).unwrap();
        let soroban_version = VersionReq::parse(&soroban_version.replace('\"', "")).unwrap();

        if !soroban_version.matches(&req) {
            Detector::SorobanVersion.span_lint_and_help(
                cx,
                CHECK_SOROBAN_VERSION,
                rustc_span::DUMMY_SP,
                &format!("The latest Soroban version is {latest_version}, and your version is {soroban_version}"),
            );
        }
    }
}

fn get_version() -> String {
    let resp: serde_json::Value = ureq::get("https://crates.io/api/v1/crates/soroban-sdk")
        .set("User-Agent", "Scout/1.0")
        .call()
        .expect("Failed to get soroban version from crates.io")
        .into_json()
        .expect("Failed to parse soroban version from crates.io");
    let version = resp
        .get("crate")
        .unwrap()
        .get("max_stable_version")
        .unwrap()
        .to_string();
    version
}
