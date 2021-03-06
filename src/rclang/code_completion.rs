use std::fmt;
use collections::enum_set;

use types::*;
use ffi::*;

/**
 * Completion Chunk
 **/

pub struct CompletionChunk {
    pub kind: CXCompletionChunkKind,
    pub text: ~str
}

impl fmt::Show for CompletionChunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chunk_text = match self.kind {
            CXCompletionChunk_ResultType      => "[#" + self.text + "]",
            CXCompletionChunk_Placeholder     => "<#" + self.text + ">",
            CXCompletionChunk_TypedText  |
            CXCompletionChunk_LeftParen  |
            CXCompletionChunk_RightParen |
            CXCompletionChunk_Comma      |
            CXCompletionChunk_HorizontalSpace => self.text.clone(),
            _ => {
                debug!("missing: {}:{}", self.kind as int, self.text);
                self.text.clone()
            }
        };
        f.buf.write(chunk_text.as_bytes())
    }
}

/**
 * Completion Results
 **/

pub struct CompletionResult {
    pub priority:     uint,
    pub comment:      ~str,
    pub availability: CXAvailabilityKind,
    chunks:       Vec<CompletionChunk>
}

impl CompletionResult {
    pub fn new(completion: &CXCompletionString) -> CompletionResult {
        let priority     = unsafe { clang_getCompletionPriority(*completion) };
        let availability = unsafe { clang_getCompletionAvailability(*completion) };
        let comment      = unsafe { clang_getCompletionBriefComment(*completion) };

        let mut chunks = Vec::new();
        let num_chunks = unsafe { clang_getNumCompletionChunks(*completion) };
        for idx in range(0, num_chunks) {
            let kind = unsafe { clang_getCompletionChunkKind(*completion, idx) };
            let text = unsafe { clang_getCompletionChunkText(*completion, idx) };

            chunks.push(CompletionChunk {
                kind: enum_set::CLike::from_uint(kind as uint),
                text: text.to_str()
            });
        }

        CompletionResult {
            priority:     priority as uint,
            comment:      comment.to_str(),
            availability: enum_set::CLike::from_uint(availability as uint),
            chunks:       chunks
        }
    }

    pub fn to_yas(&self) -> ~str {
        let mut return_value = StrBuf::from_str("");
        let mut snippet = StrBuf::from_str("");
        for chunk in self.chunks.iter() {
            match chunk.kind {
                CXCompletionChunk_ResultType  => return_value.push_str(chunk.text),
                CXCompletionChunk_Placeholder => snippet.push_str("${" + chunk.text + "}"),
                _ => snippet.push_str(chunk.text)
            };
        }

        let mut result = snippet.clone();
        result.push_str("\t");
        result.push_str(return_value.into_owned());
        result.into_owned()
    }
}

impl fmt::Show for CompletionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chunks_text: Vec<~str> = self.chunks.iter().map(|chunk| {
            chunk.to_str()
        }).collect();
        f.buf.write(chunks_text.connect("").as_bytes())
    }
}
