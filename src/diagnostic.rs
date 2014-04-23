use collections::enum_set;

use types::{CXDiagnostic, CXDiagnosticSeverity};
use source_location::SourceLocation;
use ffi::*;

/**
 * Diagnostic
 **/

pub struct Diagnostic {
    category:  ~str,
    location:  SourceLocation,
    severity:  CXDiagnosticSeverity,
    spelling:  ~str,
    formatted: ~str
}

impl Diagnostic {
    pub fn new(diagnostic: &CXDiagnostic) -> Diagnostic {
        let category      = unsafe { clang_getDiagnosticCategory(*diagnostic) };
        let category_name = unsafe { clang_getDiagnosticCategoryName(category) };
        let severity      = unsafe { clang_getDiagnosticSeverity(*diagnostic) };
        let spelling      = unsafe { clang_getDiagnosticSpelling(*diagnostic) };
        let cx_location   = unsafe { clang_getDiagnosticLocation(*diagnostic) };

        let format_options = unsafe { clang_defaultDiagnosticDisplayOptions() };
        let formatted      = unsafe { clang_formatDiagnostic(*diagnostic, format_options) };

        Diagnostic {
            category:  category_name.to_str(),
            location:  cx_location.expansion_location(),
            severity:  enum_set::CLike::from_uint(severity as uint),
            spelling:  spelling.to_str(),
            formatted: formatted.to_str()
        }
    }
}
