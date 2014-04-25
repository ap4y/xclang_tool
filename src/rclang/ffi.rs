use libc::{c_char, c_int, c_uint};

use types::*;
use cx_string::CXString;
use source_location::CXSourceLocation;

#[link(name = "clang")]
extern {

    /**
     * Compilation Database
     **/
    pub fn clang_CompilationDatabase_fromDirectory(build_dir: *c_char,
                                                   error_code: *c_int)
                                                   -> CXCompilationDatabase;
    pub fn clang_CompilationDatabase_dispose(c_db: CXCompilationDatabase);
    pub fn clang_CompilationDatabase_getCompileCommands(c_db: CXCompilationDatabase,
                                                        complete_file_name: *c_char)
                                                        -> CXCompileCommands;

    /**
     * Compilation Command
     **/
    pub fn clang_CompileCommands_dispose(compile_commands: CXCompileCommands);

    pub fn clang_CompileCommands_getSize(compile_commands: CXCompileCommands) -> c_uint;
    pub fn clang_CompileCommands_getCommand(compile_commands: CXCompileCommands, i: c_uint)
                                            -> CXCompileCommand;

    pub fn clang_CompileCommand_getDirectory(command: CXCompileCommand) -> CXString;
    pub fn clang_CompileCommand_getNumArgs(command: CXCompileCommand) -> c_uint;
    pub fn clang_CompileCommand_getArg(command: CXCompileCommand, i: c_uint) -> CXString;

    /**
     * Index
     **/
    pub fn clang_createIndex(exclude_declarations_from_pch: c_int,
                             display_diagnostics: c_int) -> CXIndex;
    pub fn clang_disposeIndex(index: CXIndex);

    /**
     * Translation Unit
     **/
    pub fn clang_parseTranslationUnit(c_idx: CXIndex,
                                      source_filename: *c_char,
                                      command_line_args: **c_char,
                                      num_command_line_args: c_int,
                                      unsaved_files: *CXUnsavedFile,
                                      num_unsaved_files: c_uint,
                                      options: c_uint) -> CXTranslationUnit;

    pub fn clang_defaultEditingTranslationUnitOptions() -> c_uint;

    /**
     * Code Completion
     **/
    pub fn clang_codeCompleteAt(tu: CXTranslationUnit,
                                complete_filename: *c_char,
                                complete_line: c_uint,
                                complete_column: c_uint,
                                unsaved_files: *CXUnsavedFile,
                                num_unsaved_files: c_uint,
                                options: c_uint) -> *CXCodeCompleteResults;
    pub fn clang_defaultCodeCompleteOptions() -> c_uint;
    pub fn clang_disposeCodeCompleteResults(results: *CXCodeCompleteResults);

    /**
     * Code Completion Result
     **/
    pub fn clang_getCompletionPriority(completion_string: CXCompletionString) -> c_uint;
    pub fn clang_getCompletionAvailability(completion_string: CXCompletionString) -> c_uint;
    pub fn clang_getCompletionBriefComment(completion_string: CXCompletionString) -> CXString;
    pub fn clang_getNumCompletionChunks(completion_string: CXCompletionString) -> c_uint;
    pub fn clang_getCompletionChunkKind(completion_string: CXCompletionString,
                                        chunk_number: c_uint) -> c_uint;
    pub fn clang_getCompletionChunkText(completion_string: CXCompletionString,
                                        chunk_number: c_uint) -> CXString;

    /**
     * Diagnostics
     **/
    pub fn clang_getNumDiagnostics(tu: CXTranslationUnit) -> c_uint;
    pub fn clang_getDiagnostic(tu: CXTranslationUnit, index: c_uint) -> CXDiagnostic;
    pub fn clang_disposeDiagnostic(diagnostic: CXDiagnostic);

    pub fn clang_getDiagnosticCategory(diagnostic: CXDiagnostic) -> c_uint;
    pub fn clang_getDiagnosticCategoryName(category: c_uint) -> CXString;
    pub fn clang_getDiagnosticSeverity(diagnostic: CXDiagnostic) -> c_uint;
    pub fn clang_getDiagnosticSpelling(diagnostic: CXDiagnostic) -> CXString;
    pub fn clang_getDiagnosticLocation(diagnostic: CXDiagnostic) -> CXSourceLocation;
    pub fn clang_formatDiagnostic(diagnostic: CXDiagnostic, options: c_uint) -> CXString;
    pub fn clang_defaultDiagnosticDisplayOptions() -> c_uint;

    /**
     * Source Location
     **/
    pub fn clang_getSpellingLocation(location: CXSourceLocation,
                                     file: *CXFile,
                                     line: *c_uint,
                                     column: *c_uint,
                                     offset: *c_uint);
    pub fn clang_getExpansionLocation(location: CXSourceLocation,
                                      file: *CXFile,
                                      line: *c_uint,
                                      column: *c_uint,
                                      offset: *c_uint);
    pub fn clang_getLocation(tu: CXTranslationUnit,
                             file: CXFile,
                             line: c_uint,
                             column: c_uint) -> CXSourceLocation;

    /**
     * Cursor
     **/
    pub fn clang_getCursor(tu: CXTranslationUnit, location: CXSourceLocation) -> CXCursor;
    pub fn clang_getCursorDefinition(cursor: CXCursor) -> CXCursor;
    pub fn clang_getCursorReferenced(cursor: CXCursor) -> CXCursor;
    pub fn clang_getCursorLocation(cursor: CXCursor) -> CXSourceLocation;

    /**
     * Other
     **/
    pub fn clang_getFile(tu: CXTranslationUnit, file_name: *c_char) -> CXFile;
    pub fn clang_getFileName(file: CXFile) -> CXString;
    pub fn clang_getCString(string: CXString) -> *c_char;
}
