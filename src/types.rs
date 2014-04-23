use libc::{c_void, c_char, c_ulong, c_uint, c_int};
use collections::enum_set::CLike;
use std::cast;

/**
 * Compilation Database
 **/

pub type CXCompilationDatabase = *c_void;
pub type CXCompileCommand      = *c_void;
pub type CXCompileCommands     = *c_void;

pub enum CXCompilationDatabase_Error {
    CXCompilationDatabase_NoError,
    CXCompilationDatabase_CanNotLoadDatabase
}

/**
 * Translation Unit
 **/

pub type CXIndex            = *c_void;
pub type CXTranslationUnit  = *c_void;

pub enum CXTranslationUnit_Flags {
    CXTranslationUnit_None                                 = 0x0,
    CXTranslationUnit_DetailedPreprocessingRecord          = 0x01,
    CXTranslationUnit_Incomplete                           = 0x02,
    CXTranslationUnit_PrecompiledPreamble                  = 0x04,
    CXTranslationUnit_CacheCompletionResults               = 0x08,
    CXTranslationUnit_ForSerialization                     = 0x10,
    CXTranslationUnit_CXXChainedPCH                        = 0x20,
    CXTranslationUnit_SkipFunctionBodies                   = 0x40,
    CXTranslationUnit_IncludeBriefCommentsInCodeCompletion = 0x80
}

/**
 * Completion
 **/

pub type CXCompletionString = *c_void;

pub struct CXUnsavedFile {
    filename: *c_char,
    contents: *c_char,
    length:   c_ulong
}

pub struct CXCompletionResult {
    cursor_kind:       c_int,
    completion_string: CXCompletionString
}

pub struct CXCodeCompleteResults {
    results:     *CXCompletionResult,
    num_results: c_uint
}

#[repr(uint)]
pub enum CXAvailabilityKind {
    CXAvailability_Available,
    CXAvailability_Deprecated,
    CXAvailability_NotAvailable,
    CXAvailability_NotAccessible
}
impl CLike for CXAvailabilityKind {
    fn to_uint(&self) -> uint { *self as uint }
    fn from_uint(v: uint) -> CXAvailabilityKind { unsafe { cast::transmute(v) } }
}

#[repr(uint)]
pub enum CXCompletionChunkKind {
    CXCompletionChunk_Optional,
    CXCompletionChunk_TypedText,
    CXCompletionChunk_Text,
    CXCompletionChunk_Placeholder,
    CXCompletionChunk_Informative,
    CXCompletionChunk_CurrentParameter,
    CXCompletionChunk_LeftParen,
    CXCompletionChunk_RightParen,
    CXCompletionChunk_LeftBracket,
    CXCompletionChunk_RightBracket,
    CXCompletionChunk_LeftBrace,
    CXCompletionChunk_RightBrace,
    CXCompletionChunk_LeftAngle,
    CXCompletionChunk_RightAngle,
    CXCompletionChunk_Comma,
    CXCompletionChunk_ResultType,
    CXCompletionChunk_Colon,
    CXCompletionChunk_SemiColon,
    CXCompletionChunk_Equal,
    CXCompletionChunk_HorizontalSpace,
    CXCompletionChunk_VerticalSpace
}
impl CLike for CXCompletionChunkKind {
    fn to_uint(&self) -> uint { *self as uint }
    fn from_uint(v: uint) -> CXCompletionChunkKind { unsafe { cast::transmute(v) } }
}

/**
 * Diagnostic
 **/

pub type CXDiagnostic = *c_void;
pub type CXFile       = *c_void;

#[repr(uint)]
pub enum CXDiagnosticSeverity {
    CXDiagnostic_Ignored,
    CXDiagnostic_Note,
    CXDiagnostic_Remark,
    CXDiagnostic_Warning,
    CXDiagnostic_Error,
    CXDiagnostic_Fatal
}
impl CLike for CXDiagnosticSeverity {
    fn to_uint(&self) -> uint { *self as uint }
    fn from_uint(v: uint) -> CXDiagnosticSeverity { unsafe { cast::transmute(v) } }
}

/**
 * Cursor
 **/

pub struct CXCursor {
    kind:  c_uint,
    xdata: c_int,
    data0: *c_void,
    data1: *c_void,
    data2: *c_void
}
