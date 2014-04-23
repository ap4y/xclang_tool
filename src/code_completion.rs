use std::fmt;
use collections::enum_set;

use types::*;
use ffi::*;

/**
 * Completion Chunk
 **/

pub struct CompletionChunk {
    kind: CXCompletionChunkKind,
    text: ~str
}

/**
 * Completion Results
 **/

pub struct CompletionResult {
    priority:     uint,
    comment:      ~str,
    availability: CXAvailabilityKind,
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
}

impl fmt::Show for CompletionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chunks_text: Vec<~str> = self.chunks.iter().map(|chunk| {
            chunk.text.clone()
        }).collect();
        f.buf.write(chunks_text.connect("").as_bytes())
    }
}
