#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use std::fs;

use rustc_lint::EarlyLintPass;
use scout_audit_internal::Detector;

dylint_linting::declare_early_lint! {
    /// ### What it does
    /// Checks the ink! version of the contract
    /// ### Why is this bad?
    /// Using an outdated version of ink! could lead to security vulnerabilities, bugs, and other issues.
    ///```
    pub OVERFLOW_CHECK,
    Warn,
    Detector::OverflowCheck.get_lint_message()
}

impl EarlyLintPass for OverflowCheck {
    fn check_crate(&mut self, cx: &rustc_lint::EarlyContext<'_>, _: &rustc_ast::Crate) {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let cargo_toml_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");

        let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Unable to read Cargo.toml");

        let toml: toml::Value = toml::from_str(&cargo_toml).unwrap();

        let profiles = toml.get("profile").and_then(|p| p.as_table());

        if profiles.is_some() {
            for profile in profiles.unwrap() {
                let profile_name = profile.0;
                let table = profile.1.as_table();
                if table.is_some() && table.unwrap().contains_key("overflow-checks") {
                    let has_overflow_check = table
                        .unwrap()
                        .get("overflow-checks")
                        .is_some_and(|f| f.as_bool().unwrap_or(false));
                    if !has_overflow_check {
                        Detector::OverflowCheck.span_lint_and_help(
                            cx,
                            OVERFLOW_CHECK,
                            rustc_span::DUMMY_SP,
                            &format!("Enable overflow-checks on profile.{profile_name}"),
                        );
                    }
                } else {
                    Detector::OverflowCheck.span_lint_and_help(
                        cx,
                        OVERFLOW_CHECK,
                        rustc_span::DUMMY_SP,
                        &format!("Enable overflow-checks on profile.{profile_name}"),
                    );
                }
            }
        }

        /*let req = Version::parse(&latest_version.replace('\"', "")).unwrap();
        let ink_version = VersionReq::parse(&ink_version.replace('\"', "")).unwrap();

        if !ink_version.matches(&req) {
            Detector::OverflowCheck.span_lint_and_help(
                cx,
                OVERFLOW_CHECK,
                rustc_span::DUMMY_SP,
                &format!("The latest ink! version is {latest_version}, and your version is {ink_version}"),
            );
        }*/
    }
}
