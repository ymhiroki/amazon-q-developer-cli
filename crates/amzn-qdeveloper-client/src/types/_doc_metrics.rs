// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DocMetrics {
    #[allow(missing_docs)] // documentation missing in model
    pub accepted_number_of_add_files: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub total_number_of_add_files: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub accepted_number_of_update_files: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub total_number_of_update_files: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub accepted_number_of_add_lines: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub total_number_of_add_lines: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub accepted_number_of_update_lines: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub total_number_of_update_lines: ::std::option::Option<i64>,
    #[allow(missing_docs)] // documentation missing in model
    pub characters_added_accepted: i32,
    #[allow(missing_docs)] // documentation missing in model
    pub characters_added_total: i32,
    #[allow(missing_docs)] // documentation missing in model
    pub characters_updated_accepted: i32,
    #[allow(missing_docs)] // documentation missing in model
    pub characters_updated_total: i32,
}
impl DocMetrics {
    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_add_files(&self) -> ::std::option::Option<i64> {
        self.accepted_number_of_add_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_add_files(&self) -> ::std::option::Option<i64> {
        self.total_number_of_add_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_update_files(&self) -> ::std::option::Option<i64> {
        self.accepted_number_of_update_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_update_files(&self) -> ::std::option::Option<i64> {
        self.total_number_of_update_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_add_lines(&self) -> ::std::option::Option<i64> {
        self.accepted_number_of_add_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_add_lines(&self) -> ::std::option::Option<i64> {
        self.total_number_of_add_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_update_lines(&self) -> ::std::option::Option<i64> {
        self.accepted_number_of_update_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_update_lines(&self) -> ::std::option::Option<i64> {
        self.total_number_of_update_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_added_accepted(&self) -> i32 {
        self.characters_added_accepted
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_added_total(&self) -> i32 {
        self.characters_added_total
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_updated_accepted(&self) -> i32 {
        self.characters_updated_accepted
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_updated_total(&self) -> i32 {
        self.characters_updated_total
    }
}
impl DocMetrics {
    /// Creates a new builder-style object to manufacture [`DocMetrics`](crate::types::DocMetrics).
    pub fn builder() -> crate::types::builders::DocMetricsBuilder {
        crate::types::builders::DocMetricsBuilder::default()
    }
}

/// A builder for [`DocMetrics`](crate::types::DocMetrics).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DocMetricsBuilder {
    pub(crate) accepted_number_of_add_files: ::std::option::Option<i64>,
    pub(crate) total_number_of_add_files: ::std::option::Option<i64>,
    pub(crate) accepted_number_of_update_files: ::std::option::Option<i64>,
    pub(crate) total_number_of_update_files: ::std::option::Option<i64>,
    pub(crate) accepted_number_of_add_lines: ::std::option::Option<i64>,
    pub(crate) total_number_of_add_lines: ::std::option::Option<i64>,
    pub(crate) accepted_number_of_update_lines: ::std::option::Option<i64>,
    pub(crate) total_number_of_update_lines: ::std::option::Option<i64>,
    pub(crate) characters_added_accepted: ::std::option::Option<i32>,
    pub(crate) characters_added_total: ::std::option::Option<i32>,
    pub(crate) characters_updated_accepted: ::std::option::Option<i32>,
    pub(crate) characters_updated_total: ::std::option::Option<i32>,
}
impl DocMetricsBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_add_files(mut self, input: i64) -> Self {
        self.accepted_number_of_add_files = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_accepted_number_of_add_files(mut self, input: ::std::option::Option<i64>) -> Self {
        self.accepted_number_of_add_files = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_accepted_number_of_add_files(&self) -> &::std::option::Option<i64> {
        &self.accepted_number_of_add_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_add_files(mut self, input: i64) -> Self {
        self.total_number_of_add_files = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_total_number_of_add_files(mut self, input: ::std::option::Option<i64>) -> Self {
        self.total_number_of_add_files = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_total_number_of_add_files(&self) -> &::std::option::Option<i64> {
        &self.total_number_of_add_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_update_files(mut self, input: i64) -> Self {
        self.accepted_number_of_update_files = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_accepted_number_of_update_files(mut self, input: ::std::option::Option<i64>) -> Self {
        self.accepted_number_of_update_files = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_accepted_number_of_update_files(&self) -> &::std::option::Option<i64> {
        &self.accepted_number_of_update_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_update_files(mut self, input: i64) -> Self {
        self.total_number_of_update_files = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_total_number_of_update_files(mut self, input: ::std::option::Option<i64>) -> Self {
        self.total_number_of_update_files = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_total_number_of_update_files(&self) -> &::std::option::Option<i64> {
        &self.total_number_of_update_files
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_add_lines(mut self, input: i64) -> Self {
        self.accepted_number_of_add_lines = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_accepted_number_of_add_lines(mut self, input: ::std::option::Option<i64>) -> Self {
        self.accepted_number_of_add_lines = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_accepted_number_of_add_lines(&self) -> &::std::option::Option<i64> {
        &self.accepted_number_of_add_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_add_lines(mut self, input: i64) -> Self {
        self.total_number_of_add_lines = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_total_number_of_add_lines(mut self, input: ::std::option::Option<i64>) -> Self {
        self.total_number_of_add_lines = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_total_number_of_add_lines(&self) -> &::std::option::Option<i64> {
        &self.total_number_of_add_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn accepted_number_of_update_lines(mut self, input: i64) -> Self {
        self.accepted_number_of_update_lines = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_accepted_number_of_update_lines(mut self, input: ::std::option::Option<i64>) -> Self {
        self.accepted_number_of_update_lines = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_accepted_number_of_update_lines(&self) -> &::std::option::Option<i64> {
        &self.accepted_number_of_update_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn total_number_of_update_lines(mut self, input: i64) -> Self {
        self.total_number_of_update_lines = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_total_number_of_update_lines(mut self, input: ::std::option::Option<i64>) -> Self {
        self.total_number_of_update_lines = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_total_number_of_update_lines(&self) -> &::std::option::Option<i64> {
        &self.total_number_of_update_lines
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_added_accepted(mut self, input: i32) -> Self {
        self.characters_added_accepted = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_characters_added_accepted(mut self, input: ::std::option::Option<i32>) -> Self {
        self.characters_added_accepted = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_characters_added_accepted(&self) -> &::std::option::Option<i32> {
        &self.characters_added_accepted
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_added_total(mut self, input: i32) -> Self {
        self.characters_added_total = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_characters_added_total(mut self, input: ::std::option::Option<i32>) -> Self {
        self.characters_added_total = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_characters_added_total(&self) -> &::std::option::Option<i32> {
        &self.characters_added_total
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_updated_accepted(mut self, input: i32) -> Self {
        self.characters_updated_accepted = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_characters_updated_accepted(mut self, input: ::std::option::Option<i32>) -> Self {
        self.characters_updated_accepted = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_characters_updated_accepted(&self) -> &::std::option::Option<i32> {
        &self.characters_updated_accepted
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn characters_updated_total(mut self, input: i32) -> Self {
        self.characters_updated_total = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_characters_updated_total(mut self, input: ::std::option::Option<i32>) -> Self {
        self.characters_updated_total = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_characters_updated_total(&self) -> &::std::option::Option<i32> {
        &self.characters_updated_total
    }

    /// Consumes the builder and constructs a [`DocMetrics`](crate::types::DocMetrics).
    pub fn build(self) -> crate::types::DocMetrics {
        crate::types::DocMetrics {
            accepted_number_of_add_files: self.accepted_number_of_add_files,
            total_number_of_add_files: self.total_number_of_add_files,
            accepted_number_of_update_files: self.accepted_number_of_update_files,
            total_number_of_update_files: self.total_number_of_update_files,
            accepted_number_of_add_lines: self.accepted_number_of_add_lines,
            total_number_of_add_lines: self.total_number_of_add_lines,
            accepted_number_of_update_lines: self.accepted_number_of_update_lines,
            total_number_of_update_lines: self.total_number_of_update_lines,
            characters_added_accepted: self.characters_added_accepted.unwrap_or_default(),
            characters_added_total: self.characters_added_total.unwrap_or_default(),
            characters_updated_accepted: self.characters_updated_accepted.unwrap_or_default(),
            characters_updated_total: self.characters_updated_total.unwrap_or_default(),
        }
    }
}
