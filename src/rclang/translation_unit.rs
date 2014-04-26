use std::slice;
use std::ptr;

use types::*;
use ffi::*;
use compilation_database::{CompilationCommand, CompilationDatabase};
use code_completion::CompletionResult;
use diagnostic::Diagnostic;
use source_location::SourceLocation;

/**
 * Translation Unit
 **/

pub struct TranslationUnit {
    cx_index:            CXIndex,
    cx_translation_unit: CXTranslationUnit
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe { clang_disposeIndex(self.cx_index); }
    }
}

impl TranslationUnit {
    pub fn new(compilation_command: &CompilationCommand, file_path: &Path) -> TranslationUnit {
        let default_parse_options = unsafe { clang_defaultEditingTranslationUnitOptions() };
        let parse_options = default_parse_options | CXTranslationUnit_PrecompiledPreamble as u32;
        let _file_name = unsafe { file_path.to_c_str().unwrap() };
        let index = unsafe { clang_createIndex(0, 0) };

        let tu = unsafe {
            clang_parseTranslationUnit(index, _file_name,
                                       compilation_command.args_as_c_vec().as_ptr(),
                                       compilation_command.args.len() as i32,
                                       ptr::null(), 0, parse_options)
        };

        TranslationUnit { cx_translation_unit: tu, cx_index: index }
    }

    pub fn complete_code_at(&self, file_path: &Path, line: uint, column: uint) -> Vec<CompletionResult> {
        let _file_name = unsafe { file_path.to_c_str().unwrap() };

        let completions = unsafe {
            clang_codeCompleteAt(self.cx_translation_unit, _file_name,
                                 line as u32, column as u32,
                                 ptr::null(), 0,
                                 clang_defaultCodeCompleteOptions())
        };

        let mut results = Vec::new();
        let completion_vector = unsafe {
            slice::raw::from_buf_raw((*completions).results, (*completions).num_results as uint)
        };
        for idx in range(0, completion_vector.len()) {
            let completion_option = completion_vector.get(idx);
            match completion_option {
                Some(completion) => results.push(CompletionResult::new(&completion.completion_string)),
                _ => ()
            }
        }

        unsafe { clang_disposeCodeCompleteResults(completions) };
        results
    }

    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let num_diagnostics = unsafe { clang_getNumDiagnostics(self.cx_translation_unit) };

        let mut diagnostics = Vec::new();
        for idx in range(0, num_diagnostics) {
            let diagnostic = unsafe { clang_getDiagnostic(self.cx_translation_unit, idx) };
            diagnostics.push(Diagnostic::new(&diagnostic));
            unsafe { clang_disposeDiagnostic(diagnostic) };
        }

        diagnostics
    }

    pub fn referenced_location(&self, c_db: &CompilationDatabase,
                               cursor: CXCursor) -> SourceLocation {

        let referenced_cursor = unsafe { clang_getCursorReferenced(cursor) };
        let cx_location = unsafe { clang_getCursorLocation(referenced_cursor) };
        let location = cx_location.expansion_location();
        let m_file = location.file.replace(".h", ".m");

        match c_db.translation_unit_for(&Path::new(m_file)) {
            Some(child_tu) => {
                let line = location.line;
                let column = location.column;
                child_tu.go_to_definition(c_db, &Path::new(location.file), line, column)
            },
            None => location
        }
    }

    pub fn go_to_definition(&self, c_db: &CompilationDatabase, file_path: &Path,
                            line: uint, column: uint) -> SourceLocation {

        let file = file_path.with_c_str(|_file_name| {
            unsafe { clang_getFile(self.cx_translation_unit, _file_name) }
        });
        let location = unsafe { clang_getLocation(self.cx_translation_unit, file,
                                                  line as u32, column as u32) };
        let cursor = unsafe { clang_getCursor(self.cx_translation_unit, location) };

        match cursor.kind {
            104 => { // CXCursor_ObjCMessageExpr
                self.referenced_location(c_db, cursor)
            },
            _  => {
                let definition_cursor = unsafe { clang_getCursorDefinition(cursor) };
                let definition_location = unsafe { clang_getCursorLocation(definition_cursor) };
                definition_location.expansion_location()
            }
        }
    }
}
