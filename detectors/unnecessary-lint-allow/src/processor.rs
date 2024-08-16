use serde::de::Error;
use serde_json::Value;
use std::{collections::HashMap, ffi::CStr, os::raw::c_char};

/// Determines whether a finding should be included.
///
/// # Safety
///
/// This function is marked as unsafe because it deals with raw pointers.
/// The caller is responsible for ensuring the safety of the pointers passed as arguments.
#[no_mangle]
pub unsafe extern "C" fn should_include_finding(
    finding_json: *const c_char,
    all_findings_json: *const c_char,
) -> bool {
    // Check for null pointers
    if finding_json.is_null() || all_findings_json.is_null() {
        return false;
    }

    let finding = match parse_json(finding_json) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let all_findings = match parse_json(all_findings_json) {
        Ok(Value::Array(v)) => v,
        _ => return false,
    };

    should_include_finding_impl(&finding, &all_findings)
}

unsafe fn parse_json(input: *const c_char) -> Result<Value, serde_json::Error> {
    let c_str = CStr::from_ptr(input);
    let input_str = c_str
        .to_str()
        .map_err(|_| serde_json::Error::custom("Invalid UTF-8"))?;
    serde_json::from_str(input_str)
}

#[derive(Debug, Clone)]
struct Finding {
    detector: String,
    file_name: String,
    span: (usize, usize),
    allowed_lint: Option<String>,
}

fn spans_overlap(span1: (usize, usize), span2: (usize, usize)) -> bool {
    span1.0 <= span2.1 && span2.0 <= span1.1
}

fn parse_finding(finding: &Value) -> Option<Finding> {
    let detector = finding.get("code")?.get("code")?.as_str()?;
    let span = finding.get("spans")?.as_array()?.first()?;
    let file_name = span.get("file_name")?.as_str()?;
    let line_start = span.get("line_start")?.as_u64()?;
    let line_end = span.get("line_end")?.as_u64()?;

    let allowed_lint = if detector == "unnecessary_lint_allow" {
        finding
            .get("children")?
            .get(0)?
            .get("message")?
            .as_str()
            .and_then(|msg| msg.split('`').nth(1).map(String::from))
    } else {
        None
    };

    // Check for potential integer overflow when converting from u64 to usize
    let start = usize::try_from(line_start).ok()?;
    let end = usize::try_from(line_end).ok()?;

    Some(Finding {
        detector: detector.to_owned(),
        file_name: file_name.to_owned(),
        span: (start, end),
        allowed_lint,
    })
}

pub fn should_include_finding_impl(finding: &Value, all_findings: &[Value]) -> bool {
    let current_finding = match parse_finding(finding) {
        Some(f) => f,
        None => return false, // If we can't parse the finding, we don't include it
    };

    let mut findings_by_file: HashMap<String, Vec<Finding>> = HashMap::new();
    for f in all_findings {
        if let Some(parsed) = parse_finding(f) {
            findings_by_file
                .entry(parsed.file_name.clone())
                .or_default()
                .push(parsed);
        }
    }

    if let Some(file_findings) = findings_by_file.get(&current_finding.file_name) {
        let (unnecessary_allows, other_findings): (Vec<_>, Vec<_>) = file_findings
            .iter()
            .partition(|f| f.detector == "unnecessary_lint_allow");

        if current_finding.detector == "unnecessary_lint_allow" {
            if let Some(allowed_lint) = &current_finding.allowed_lint {
                let lint_present = other_findings.iter().any(|f| {
                    &f.detector == allowed_lint && spans_overlap(f.span, current_finding.span)
                });
                !lint_present // Include if the lint is not present (unnecessary allow)
            } else {
                true // Include if we can't determine the allowed lint
            }
        } else {
            !unnecessary_allows.iter().any(|allow| {
                allow
                    .allowed_lint
                    .as_ref()
                    .map_or(false, |lint| lint == &current_finding.detector)
                    && spans_overlap(allow.span, current_finding.span)
            }) // Include if the finding is not allowed
        }
    } else {
        true // If we can't find the file, we include it by default
    }
}
