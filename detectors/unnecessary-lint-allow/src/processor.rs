use serde::de::Error;
use serde_json::Value;
use std::{collections::HashMap, ffi::CStr, ffi::CString, os::raw::c_char};

/// Process the findings to filter out unnecessary findings.
///
/// # Safety
///
/// This function is marked as unsafe because it deals with raw pointers.
/// The caller is responsible for ensuring the safety of the pointers passed as arguments.
#[no_mangle]
pub unsafe extern "C" fn process_findings(
    successful_findings_json: *const c_char,
    output_json: *const c_char,
    inside_vscode: bool,
) -> *mut c_char {
    let successful_findings = match parse_json(successful_findings_json) {
        Ok(Value::Array(v)) => v,
        _ => return std::ptr::null_mut(),
    };

    let output = match parse_json(output_json) {
        Ok(Value::Array(v)) => v,
        _ => return std::ptr::null_mut(),
    };

    let (console_findings, output_string_vscode) =
        process_findings_impl(successful_findings, output, inside_vscode);

    let result = serde_json::json!({
        "console_findings": console_findings,
        "output_string_vscode": output_string_vscode
    });

    let result_string = serde_json::to_string(&result).unwrap_or_default();
    let c_str = CString::new(result_string).unwrap_or_default();
    c_str.into_raw()
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

fn process_findings_impl(
    successful_findings: Vec<Value>,
    output: Vec<Value>,
    inside_vscode: bool,
) -> (Vec<Value>, String) {
    let console_findings: Vec<_> = successful_findings
        .iter()
        .filter(|&finding| should_include_finding_impl(finding, &successful_findings))
        .cloned()
        .collect();

    let output_vscode: Vec<_> = if inside_vscode {
        let all_findings: Vec<_> = output
            .iter()
            .filter_map(|val| val.get("message").cloned())
            .collect();

        output
            .into_iter()
            .filter(|val| {
                val.get("message")
                    .map(|message| should_include_finding_impl(message, &all_findings))
                    .unwrap_or(true)
            })
            .collect()
    } else {
        Vec::new()
    };

    let output_string_vscode = output_vscode
        .into_iter()
        .filter_map(|finding| serde_json::to_string(&finding).ok())
        .collect::<Vec<_>>()
        .join("\n");

    (console_findings, output_string_vscode)
}

// Add this function to free the memory allocated by process_findings
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
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
