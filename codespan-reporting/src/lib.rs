//! Diagnostic reporting support for the codespan crate.

use std::fmt;

pub mod diagnostic;
pub mod term;

pub trait Span {
    type ByteIndex;

    fn start(&self) -> Self::ByteIndex;
    fn end(&self) -> Self::ByteIndex;
}

impl Span for codespan::Span {
    type ByteIndex = codespan::ByteIndex;

    fn start(&self) -> codespan::ByteIndex {
        self.start()
    }

    fn end(&self) -> codespan::ByteIndex {
        self.end()
    }
}

pub trait Files {
    type FileId: Copy;
    type FileName: fmt::Display;

    type Span: Span;

    type LineIndex;
    type ColumnIndex;

    /// Get the name of the source file.
    fn name(&self, file_id: Self::FileId) -> Self::FileName;

    /// Get the span at the given line index.
    fn line_span(&self, file_id: Self::FileId, line_index: Self::LineIndex) -> Option<Self::Span>;

    /// Get the location at the given byte index in the source file.
    fn location(
        &self,
        file_id: Self::FileId,
        byte_index: <Self::Span as Span>::ByteIndex,
    ) -> Option<(Self::LineIndex, Self::ColumnIndex)>;

    /// Return the span of the full source.
    fn source_span(&self, file_id: Self::FileId) -> Self::Span;

    /// Return a slice of the source file, given a span.
    fn source_slice(&self, file_id: Self::FileId, span: Self::Span) -> Option<String>;
}

impl Files for codespan::Files {
    type FileId = codespan::FileId;
    type FileName = String;

    type Span = codespan::Span;

    type LineIndex = codespan::LineIndex;
    type ColumnIndex = codespan::ColumnIndex;

    fn name(&self, file_id: codespan::FileId) -> String {
        self.name(file_id).to_owned()
    }

    fn line_span(
        &self,
        file_id: codespan::FileId,
        line_index: codespan::LineIndex,
    ) -> Option<codespan::Span> {
        unimplemented!()
    }

    fn location(
        &self,
        file_id: codespan::FileId,
        byte_index: codespan::ByteIndex,
    ) -> Option<(codespan::LineIndex, codespan::ColumnIndex)> {
        unimplemented!()
    }

    fn source_span(&self, file_id: codespan::FileId) -> codespan::Span {
        unimplemented!()
    }

    fn source_slice(&self, file_id: codespan::FileId, span: codespan::Span) -> Option<String> {
        unimplemented!()
    }
}
