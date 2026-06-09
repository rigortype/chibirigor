
use std::marker::PhantomData;
use std::ptr::NonNull;

#[allow(clippy::wildcard_imports)]
use ruby_prism_sys::*;
use crate::{ConstantId, ConstantList, Integer, Location, NodeList};
const PM_ALIAS_GLOBAL_VARIABLE_NODE: u16 = pm_node_type::PM_ALIAS_GLOBAL_VARIABLE_NODE as u16;
const PM_ALIAS_METHOD_NODE: u16 = pm_node_type::PM_ALIAS_METHOD_NODE as u16;
const PM_ALTERNATION_PATTERN_NODE: u16 = pm_node_type::PM_ALTERNATION_PATTERN_NODE as u16;
const PM_AND_NODE: u16 = pm_node_type::PM_AND_NODE as u16;
const PM_ARGUMENTS_NODE: u16 = pm_node_type::PM_ARGUMENTS_NODE as u16;
const PM_ARRAY_NODE: u16 = pm_node_type::PM_ARRAY_NODE as u16;
const PM_ARRAY_PATTERN_NODE: u16 = pm_node_type::PM_ARRAY_PATTERN_NODE as u16;
const PM_ASSOC_NODE: u16 = pm_node_type::PM_ASSOC_NODE as u16;
const PM_ASSOC_SPLAT_NODE: u16 = pm_node_type::PM_ASSOC_SPLAT_NODE as u16;
const PM_BACK_REFERENCE_READ_NODE: u16 = pm_node_type::PM_BACK_REFERENCE_READ_NODE as u16;
const PM_BEGIN_NODE: u16 = pm_node_type::PM_BEGIN_NODE as u16;
const PM_BLOCK_ARGUMENT_NODE: u16 = pm_node_type::PM_BLOCK_ARGUMENT_NODE as u16;
const PM_BLOCK_LOCAL_VARIABLE_NODE: u16 = pm_node_type::PM_BLOCK_LOCAL_VARIABLE_NODE as u16;
const PM_BLOCK_NODE: u16 = pm_node_type::PM_BLOCK_NODE as u16;
const PM_BLOCK_PARAMETER_NODE: u16 = pm_node_type::PM_BLOCK_PARAMETER_NODE as u16;
const PM_BLOCK_PARAMETERS_NODE: u16 = pm_node_type::PM_BLOCK_PARAMETERS_NODE as u16;
const PM_BREAK_NODE: u16 = pm_node_type::PM_BREAK_NODE as u16;
const PM_CALL_AND_WRITE_NODE: u16 = pm_node_type::PM_CALL_AND_WRITE_NODE as u16;
const PM_CALL_NODE: u16 = pm_node_type::PM_CALL_NODE as u16;
const PM_CALL_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_CALL_OPERATOR_WRITE_NODE as u16;
const PM_CALL_OR_WRITE_NODE: u16 = pm_node_type::PM_CALL_OR_WRITE_NODE as u16;
const PM_CALL_TARGET_NODE: u16 = pm_node_type::PM_CALL_TARGET_NODE as u16;
const PM_CAPTURE_PATTERN_NODE: u16 = pm_node_type::PM_CAPTURE_PATTERN_NODE as u16;
const PM_CASE_MATCH_NODE: u16 = pm_node_type::PM_CASE_MATCH_NODE as u16;
const PM_CASE_NODE: u16 = pm_node_type::PM_CASE_NODE as u16;
const PM_CLASS_NODE: u16 = pm_node_type::PM_CLASS_NODE as u16;
const PM_CLASS_VARIABLE_AND_WRITE_NODE: u16 = pm_node_type::PM_CLASS_VARIABLE_AND_WRITE_NODE as u16;
const PM_CLASS_VARIABLE_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_CLASS_VARIABLE_OPERATOR_WRITE_NODE as u16;
const PM_CLASS_VARIABLE_OR_WRITE_NODE: u16 = pm_node_type::PM_CLASS_VARIABLE_OR_WRITE_NODE as u16;
const PM_CLASS_VARIABLE_READ_NODE: u16 = pm_node_type::PM_CLASS_VARIABLE_READ_NODE as u16;
const PM_CLASS_VARIABLE_TARGET_NODE: u16 = pm_node_type::PM_CLASS_VARIABLE_TARGET_NODE as u16;
const PM_CLASS_VARIABLE_WRITE_NODE: u16 = pm_node_type::PM_CLASS_VARIABLE_WRITE_NODE as u16;
const PM_CONSTANT_AND_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_AND_WRITE_NODE as u16;
const PM_CONSTANT_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_OPERATOR_WRITE_NODE as u16;
const PM_CONSTANT_OR_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_OR_WRITE_NODE as u16;
const PM_CONSTANT_PATH_AND_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_PATH_AND_WRITE_NODE as u16;
const PM_CONSTANT_PATH_NODE: u16 = pm_node_type::PM_CONSTANT_PATH_NODE as u16;
const PM_CONSTANT_PATH_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_PATH_OPERATOR_WRITE_NODE as u16;
const PM_CONSTANT_PATH_OR_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_PATH_OR_WRITE_NODE as u16;
const PM_CONSTANT_PATH_TARGET_NODE: u16 = pm_node_type::PM_CONSTANT_PATH_TARGET_NODE as u16;
const PM_CONSTANT_PATH_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_PATH_WRITE_NODE as u16;
const PM_CONSTANT_READ_NODE: u16 = pm_node_type::PM_CONSTANT_READ_NODE as u16;
const PM_CONSTANT_TARGET_NODE: u16 = pm_node_type::PM_CONSTANT_TARGET_NODE as u16;
const PM_CONSTANT_WRITE_NODE: u16 = pm_node_type::PM_CONSTANT_WRITE_NODE as u16;
const PM_DEF_NODE: u16 = pm_node_type::PM_DEF_NODE as u16;
const PM_DEFINED_NODE: u16 = pm_node_type::PM_DEFINED_NODE as u16;
const PM_ELSE_NODE: u16 = pm_node_type::PM_ELSE_NODE as u16;
const PM_EMBEDDED_STATEMENTS_NODE: u16 = pm_node_type::PM_EMBEDDED_STATEMENTS_NODE as u16;
const PM_EMBEDDED_VARIABLE_NODE: u16 = pm_node_type::PM_EMBEDDED_VARIABLE_NODE as u16;
const PM_ENSURE_NODE: u16 = pm_node_type::PM_ENSURE_NODE as u16;
const PM_FALSE_NODE: u16 = pm_node_type::PM_FALSE_NODE as u16;
const PM_FIND_PATTERN_NODE: u16 = pm_node_type::PM_FIND_PATTERN_NODE as u16;
const PM_FLIP_FLOP_NODE: u16 = pm_node_type::PM_FLIP_FLOP_NODE as u16;
const PM_FLOAT_NODE: u16 = pm_node_type::PM_FLOAT_NODE as u16;
const PM_FOR_NODE: u16 = pm_node_type::PM_FOR_NODE as u16;
const PM_FORWARDING_ARGUMENTS_NODE: u16 = pm_node_type::PM_FORWARDING_ARGUMENTS_NODE as u16;
const PM_FORWARDING_PARAMETER_NODE: u16 = pm_node_type::PM_FORWARDING_PARAMETER_NODE as u16;
const PM_FORWARDING_SUPER_NODE: u16 = pm_node_type::PM_FORWARDING_SUPER_NODE as u16;
const PM_GLOBAL_VARIABLE_AND_WRITE_NODE: u16 = pm_node_type::PM_GLOBAL_VARIABLE_AND_WRITE_NODE as u16;
const PM_GLOBAL_VARIABLE_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_GLOBAL_VARIABLE_OPERATOR_WRITE_NODE as u16;
const PM_GLOBAL_VARIABLE_OR_WRITE_NODE: u16 = pm_node_type::PM_GLOBAL_VARIABLE_OR_WRITE_NODE as u16;
const PM_GLOBAL_VARIABLE_READ_NODE: u16 = pm_node_type::PM_GLOBAL_VARIABLE_READ_NODE as u16;
const PM_GLOBAL_VARIABLE_TARGET_NODE: u16 = pm_node_type::PM_GLOBAL_VARIABLE_TARGET_NODE as u16;
const PM_GLOBAL_VARIABLE_WRITE_NODE: u16 = pm_node_type::PM_GLOBAL_VARIABLE_WRITE_NODE as u16;
const PM_HASH_NODE: u16 = pm_node_type::PM_HASH_NODE as u16;
const PM_HASH_PATTERN_NODE: u16 = pm_node_type::PM_HASH_PATTERN_NODE as u16;
const PM_IF_NODE: u16 = pm_node_type::PM_IF_NODE as u16;
const PM_IMAGINARY_NODE: u16 = pm_node_type::PM_IMAGINARY_NODE as u16;
const PM_IMPLICIT_NODE: u16 = pm_node_type::PM_IMPLICIT_NODE as u16;
const PM_IMPLICIT_REST_NODE: u16 = pm_node_type::PM_IMPLICIT_REST_NODE as u16;
const PM_IN_NODE: u16 = pm_node_type::PM_IN_NODE as u16;
const PM_INDEX_AND_WRITE_NODE: u16 = pm_node_type::PM_INDEX_AND_WRITE_NODE as u16;
const PM_INDEX_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_INDEX_OPERATOR_WRITE_NODE as u16;
const PM_INDEX_OR_WRITE_NODE: u16 = pm_node_type::PM_INDEX_OR_WRITE_NODE as u16;
const PM_INDEX_TARGET_NODE: u16 = pm_node_type::PM_INDEX_TARGET_NODE as u16;
const PM_INSTANCE_VARIABLE_AND_WRITE_NODE: u16 = pm_node_type::PM_INSTANCE_VARIABLE_AND_WRITE_NODE as u16;
const PM_INSTANCE_VARIABLE_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_INSTANCE_VARIABLE_OPERATOR_WRITE_NODE as u16;
const PM_INSTANCE_VARIABLE_OR_WRITE_NODE: u16 = pm_node_type::PM_INSTANCE_VARIABLE_OR_WRITE_NODE as u16;
const PM_INSTANCE_VARIABLE_READ_NODE: u16 = pm_node_type::PM_INSTANCE_VARIABLE_READ_NODE as u16;
const PM_INSTANCE_VARIABLE_TARGET_NODE: u16 = pm_node_type::PM_INSTANCE_VARIABLE_TARGET_NODE as u16;
const PM_INSTANCE_VARIABLE_WRITE_NODE: u16 = pm_node_type::PM_INSTANCE_VARIABLE_WRITE_NODE as u16;
const PM_INTEGER_NODE: u16 = pm_node_type::PM_INTEGER_NODE as u16;
const PM_INTERPOLATED_MATCH_LAST_LINE_NODE: u16 = pm_node_type::PM_INTERPOLATED_MATCH_LAST_LINE_NODE as u16;
const PM_INTERPOLATED_REGULAR_EXPRESSION_NODE: u16 = pm_node_type::PM_INTERPOLATED_REGULAR_EXPRESSION_NODE as u16;
const PM_INTERPOLATED_STRING_NODE: u16 = pm_node_type::PM_INTERPOLATED_STRING_NODE as u16;
const PM_INTERPOLATED_SYMBOL_NODE: u16 = pm_node_type::PM_INTERPOLATED_SYMBOL_NODE as u16;
const PM_INTERPOLATED_X_STRING_NODE: u16 = pm_node_type::PM_INTERPOLATED_X_STRING_NODE as u16;
const PM_IT_LOCAL_VARIABLE_READ_NODE: u16 = pm_node_type::PM_IT_LOCAL_VARIABLE_READ_NODE as u16;
const PM_IT_PARAMETERS_NODE: u16 = pm_node_type::PM_IT_PARAMETERS_NODE as u16;
const PM_KEYWORD_HASH_NODE: u16 = pm_node_type::PM_KEYWORD_HASH_NODE as u16;
const PM_KEYWORD_REST_PARAMETER_NODE: u16 = pm_node_type::PM_KEYWORD_REST_PARAMETER_NODE as u16;
const PM_LAMBDA_NODE: u16 = pm_node_type::PM_LAMBDA_NODE as u16;
const PM_LOCAL_VARIABLE_AND_WRITE_NODE: u16 = pm_node_type::PM_LOCAL_VARIABLE_AND_WRITE_NODE as u16;
const PM_LOCAL_VARIABLE_OPERATOR_WRITE_NODE: u16 = pm_node_type::PM_LOCAL_VARIABLE_OPERATOR_WRITE_NODE as u16;
const PM_LOCAL_VARIABLE_OR_WRITE_NODE: u16 = pm_node_type::PM_LOCAL_VARIABLE_OR_WRITE_NODE as u16;
const PM_LOCAL_VARIABLE_READ_NODE: u16 = pm_node_type::PM_LOCAL_VARIABLE_READ_NODE as u16;
const PM_LOCAL_VARIABLE_TARGET_NODE: u16 = pm_node_type::PM_LOCAL_VARIABLE_TARGET_NODE as u16;
const PM_LOCAL_VARIABLE_WRITE_NODE: u16 = pm_node_type::PM_LOCAL_VARIABLE_WRITE_NODE as u16;
const PM_MATCH_LAST_LINE_NODE: u16 = pm_node_type::PM_MATCH_LAST_LINE_NODE as u16;
const PM_MATCH_PREDICATE_NODE: u16 = pm_node_type::PM_MATCH_PREDICATE_NODE as u16;
const PM_MATCH_REQUIRED_NODE: u16 = pm_node_type::PM_MATCH_REQUIRED_NODE as u16;
const PM_MATCH_WRITE_NODE: u16 = pm_node_type::PM_MATCH_WRITE_NODE as u16;
const PM_MISSING_NODE: u16 = pm_node_type::PM_MISSING_NODE as u16;
const PM_MODULE_NODE: u16 = pm_node_type::PM_MODULE_NODE as u16;
const PM_MULTI_TARGET_NODE: u16 = pm_node_type::PM_MULTI_TARGET_NODE as u16;
const PM_MULTI_WRITE_NODE: u16 = pm_node_type::PM_MULTI_WRITE_NODE as u16;
const PM_NEXT_NODE: u16 = pm_node_type::PM_NEXT_NODE as u16;
const PM_NIL_NODE: u16 = pm_node_type::PM_NIL_NODE as u16;
const PM_NO_KEYWORDS_PARAMETER_NODE: u16 = pm_node_type::PM_NO_KEYWORDS_PARAMETER_NODE as u16;
const PM_NUMBERED_PARAMETERS_NODE: u16 = pm_node_type::PM_NUMBERED_PARAMETERS_NODE as u16;
const PM_NUMBERED_REFERENCE_READ_NODE: u16 = pm_node_type::PM_NUMBERED_REFERENCE_READ_NODE as u16;
const PM_OPTIONAL_KEYWORD_PARAMETER_NODE: u16 = pm_node_type::PM_OPTIONAL_KEYWORD_PARAMETER_NODE as u16;
const PM_OPTIONAL_PARAMETER_NODE: u16 = pm_node_type::PM_OPTIONAL_PARAMETER_NODE as u16;
const PM_OR_NODE: u16 = pm_node_type::PM_OR_NODE as u16;
const PM_PARAMETERS_NODE: u16 = pm_node_type::PM_PARAMETERS_NODE as u16;
const PM_PARENTHESES_NODE: u16 = pm_node_type::PM_PARENTHESES_NODE as u16;
const PM_PINNED_EXPRESSION_NODE: u16 = pm_node_type::PM_PINNED_EXPRESSION_NODE as u16;
const PM_PINNED_VARIABLE_NODE: u16 = pm_node_type::PM_PINNED_VARIABLE_NODE as u16;
const PM_POST_EXECUTION_NODE: u16 = pm_node_type::PM_POST_EXECUTION_NODE as u16;
const PM_PRE_EXECUTION_NODE: u16 = pm_node_type::PM_PRE_EXECUTION_NODE as u16;
const PM_PROGRAM_NODE: u16 = pm_node_type::PM_PROGRAM_NODE as u16;
const PM_RANGE_NODE: u16 = pm_node_type::PM_RANGE_NODE as u16;
const PM_RATIONAL_NODE: u16 = pm_node_type::PM_RATIONAL_NODE as u16;
const PM_REDO_NODE: u16 = pm_node_type::PM_REDO_NODE as u16;
const PM_REGULAR_EXPRESSION_NODE: u16 = pm_node_type::PM_REGULAR_EXPRESSION_NODE as u16;
const PM_REQUIRED_KEYWORD_PARAMETER_NODE: u16 = pm_node_type::PM_REQUIRED_KEYWORD_PARAMETER_NODE as u16;
const PM_REQUIRED_PARAMETER_NODE: u16 = pm_node_type::PM_REQUIRED_PARAMETER_NODE as u16;
const PM_RESCUE_MODIFIER_NODE: u16 = pm_node_type::PM_RESCUE_MODIFIER_NODE as u16;
const PM_RESCUE_NODE: u16 = pm_node_type::PM_RESCUE_NODE as u16;
const PM_REST_PARAMETER_NODE: u16 = pm_node_type::PM_REST_PARAMETER_NODE as u16;
const PM_RETRY_NODE: u16 = pm_node_type::PM_RETRY_NODE as u16;
const PM_RETURN_NODE: u16 = pm_node_type::PM_RETURN_NODE as u16;
const PM_SELF_NODE: u16 = pm_node_type::PM_SELF_NODE as u16;
const PM_SHAREABLE_CONSTANT_NODE: u16 = pm_node_type::PM_SHAREABLE_CONSTANT_NODE as u16;
const PM_SINGLETON_CLASS_NODE: u16 = pm_node_type::PM_SINGLETON_CLASS_NODE as u16;
const PM_SOURCE_ENCODING_NODE: u16 = pm_node_type::PM_SOURCE_ENCODING_NODE as u16;
const PM_SOURCE_FILE_NODE: u16 = pm_node_type::PM_SOURCE_FILE_NODE as u16;
const PM_SOURCE_LINE_NODE: u16 = pm_node_type::PM_SOURCE_LINE_NODE as u16;
const PM_SPLAT_NODE: u16 = pm_node_type::PM_SPLAT_NODE as u16;
const PM_STATEMENTS_NODE: u16 = pm_node_type::PM_STATEMENTS_NODE as u16;
const PM_STRING_NODE: u16 = pm_node_type::PM_STRING_NODE as u16;
const PM_SUPER_NODE: u16 = pm_node_type::PM_SUPER_NODE as u16;
const PM_SYMBOL_NODE: u16 = pm_node_type::PM_SYMBOL_NODE as u16;
const PM_TRUE_NODE: u16 = pm_node_type::PM_TRUE_NODE as u16;
const PM_UNDEF_NODE: u16 = pm_node_type::PM_UNDEF_NODE as u16;
const PM_UNLESS_NODE: u16 = pm_node_type::PM_UNLESS_NODE as u16;
const PM_UNTIL_NODE: u16 = pm_node_type::PM_UNTIL_NODE as u16;
const PM_WHEN_NODE: u16 = pm_node_type::PM_WHEN_NODE as u16;
const PM_WHILE_NODE: u16 = pm_node_type::PM_WHILE_NODE as u16;
const PM_X_STRING_NODE: u16 = pm_node_type::PM_X_STRING_NODE as u16;
const PM_YIELD_NODE: u16 = pm_node_type::PM_YIELD_NODE as u16;

const PM_ARGUMENTS_NODE_FLAGS_CONTAINS_FORWARDING: u16 = pm_arguments_node_flags::PM_ARGUMENTS_NODE_FLAGS_CONTAINS_FORWARDING as u16;
const PM_ARGUMENTS_NODE_FLAGS_CONTAINS_KEYWORDS: u16 = pm_arguments_node_flags::PM_ARGUMENTS_NODE_FLAGS_CONTAINS_KEYWORDS as u16;
const PM_ARGUMENTS_NODE_FLAGS_CONTAINS_KEYWORD_SPLAT: u16 = pm_arguments_node_flags::PM_ARGUMENTS_NODE_FLAGS_CONTAINS_KEYWORD_SPLAT as u16;
const PM_ARGUMENTS_NODE_FLAGS_CONTAINS_SPLAT: u16 = pm_arguments_node_flags::PM_ARGUMENTS_NODE_FLAGS_CONTAINS_SPLAT as u16;
const PM_ARGUMENTS_NODE_FLAGS_CONTAINS_MULTIPLE_SPLATS: u16 = pm_arguments_node_flags::PM_ARGUMENTS_NODE_FLAGS_CONTAINS_MULTIPLE_SPLATS as u16;
const PM_ARRAY_NODE_FLAGS_CONTAINS_SPLAT: u16 = pm_array_node_flags::PM_ARRAY_NODE_FLAGS_CONTAINS_SPLAT as u16;
const PM_CALL_NODE_FLAGS_SAFE_NAVIGATION: u16 = pm_call_node_flags::PM_CALL_NODE_FLAGS_SAFE_NAVIGATION as u16;
const PM_CALL_NODE_FLAGS_VARIABLE_CALL: u16 = pm_call_node_flags::PM_CALL_NODE_FLAGS_VARIABLE_CALL as u16;
const PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE: u16 = pm_call_node_flags::PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE as u16;
const PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY: u16 = pm_call_node_flags::PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY as u16;
const PM_ENCODING_FLAGS_FORCED_UTF8_ENCODING: u16 = pm_encoding_flags::PM_ENCODING_FLAGS_FORCED_UTF8_ENCODING as u16;
const PM_ENCODING_FLAGS_FORCED_BINARY_ENCODING: u16 = pm_encoding_flags::PM_ENCODING_FLAGS_FORCED_BINARY_ENCODING as u16;
const PM_INTEGER_BASE_FLAGS_BINARY: u16 = pm_integer_base_flags::PM_INTEGER_BASE_FLAGS_BINARY as u16;
const PM_INTEGER_BASE_FLAGS_DECIMAL: u16 = pm_integer_base_flags::PM_INTEGER_BASE_FLAGS_DECIMAL as u16;
const PM_INTEGER_BASE_FLAGS_OCTAL: u16 = pm_integer_base_flags::PM_INTEGER_BASE_FLAGS_OCTAL as u16;
const PM_INTEGER_BASE_FLAGS_HEXADECIMAL: u16 = pm_integer_base_flags::PM_INTEGER_BASE_FLAGS_HEXADECIMAL as u16;
const PM_INTERPOLATED_STRING_NODE_FLAGS_FROZEN: u16 = pm_interpolated_string_node_flags::PM_INTERPOLATED_STRING_NODE_FLAGS_FROZEN as u16;
const PM_INTERPOLATED_STRING_NODE_FLAGS_MUTABLE: u16 = pm_interpolated_string_node_flags::PM_INTERPOLATED_STRING_NODE_FLAGS_MUTABLE as u16;
const PM_KEYWORD_HASH_NODE_FLAGS_SYMBOL_KEYS: u16 = pm_keyword_hash_node_flags::PM_KEYWORD_HASH_NODE_FLAGS_SYMBOL_KEYS as u16;
const PM_LOOP_FLAGS_BEGIN_MODIFIER: u16 = pm_loop_flags::PM_LOOP_FLAGS_BEGIN_MODIFIER as u16;
const PM_PARAMETER_FLAGS_REPEATED_PARAMETER: u16 = pm_parameter_flags::PM_PARAMETER_FLAGS_REPEATED_PARAMETER as u16;
const PM_PARENTHESES_NODE_FLAGS_MULTIPLE_STATEMENTS: u16 = pm_parentheses_node_flags::PM_PARENTHESES_NODE_FLAGS_MULTIPLE_STATEMENTS as u16;
const PM_RANGE_FLAGS_EXCLUDE_END: u16 = pm_range_flags::PM_RANGE_FLAGS_EXCLUDE_END as u16;
const PM_REGULAR_EXPRESSION_FLAGS_IGNORE_CASE: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_IGNORE_CASE as u16;
const PM_REGULAR_EXPRESSION_FLAGS_EXTENDED: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_EXTENDED as u16;
const PM_REGULAR_EXPRESSION_FLAGS_MULTI_LINE: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_MULTI_LINE as u16;
const PM_REGULAR_EXPRESSION_FLAGS_ONCE: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_ONCE as u16;
const PM_REGULAR_EXPRESSION_FLAGS_EUC_JP: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_EUC_JP as u16;
const PM_REGULAR_EXPRESSION_FLAGS_ASCII_8BIT: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_ASCII_8BIT as u16;
const PM_REGULAR_EXPRESSION_FLAGS_WINDOWS_31J: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_WINDOWS_31J as u16;
const PM_REGULAR_EXPRESSION_FLAGS_UTF_8: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_UTF_8 as u16;
const PM_REGULAR_EXPRESSION_FLAGS_FORCED_UTF8_ENCODING: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_FORCED_UTF8_ENCODING as u16;
const PM_REGULAR_EXPRESSION_FLAGS_FORCED_BINARY_ENCODING: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_FORCED_BINARY_ENCODING as u16;
const PM_REGULAR_EXPRESSION_FLAGS_FORCED_US_ASCII_ENCODING: u16 = pm_regular_expression_flags::PM_REGULAR_EXPRESSION_FLAGS_FORCED_US_ASCII_ENCODING as u16;
const PM_SHAREABLE_CONSTANT_NODE_FLAGS_LITERAL: u16 = pm_shareable_constant_node_flags::PM_SHAREABLE_CONSTANT_NODE_FLAGS_LITERAL as u16;
const PM_SHAREABLE_CONSTANT_NODE_FLAGS_EXPERIMENTAL_EVERYTHING: u16 = pm_shareable_constant_node_flags::PM_SHAREABLE_CONSTANT_NODE_FLAGS_EXPERIMENTAL_EVERYTHING as u16;
const PM_SHAREABLE_CONSTANT_NODE_FLAGS_EXPERIMENTAL_COPY: u16 = pm_shareable_constant_node_flags::PM_SHAREABLE_CONSTANT_NODE_FLAGS_EXPERIMENTAL_COPY as u16;
const PM_STRING_FLAGS_FORCED_UTF8_ENCODING: u16 = pm_string_flags::PM_STRING_FLAGS_FORCED_UTF8_ENCODING as u16;
const PM_STRING_FLAGS_FORCED_BINARY_ENCODING: u16 = pm_string_flags::PM_STRING_FLAGS_FORCED_BINARY_ENCODING as u16;
const PM_STRING_FLAGS_FROZEN: u16 = pm_string_flags::PM_STRING_FLAGS_FROZEN as u16;
const PM_STRING_FLAGS_MUTABLE: u16 = pm_string_flags::PM_STRING_FLAGS_MUTABLE as u16;
const PM_SYMBOL_FLAGS_FORCED_UTF8_ENCODING: u16 = pm_symbol_flags::PM_SYMBOL_FLAGS_FORCED_UTF8_ENCODING as u16;
const PM_SYMBOL_FLAGS_FORCED_BINARY_ENCODING: u16 = pm_symbol_flags::PM_SYMBOL_FLAGS_FORCED_BINARY_ENCODING as u16;
const PM_SYMBOL_FLAGS_FORCED_US_ASCII_ENCODING: u16 = pm_symbol_flags::PM_SYMBOL_FLAGS_FORCED_US_ASCII_ENCODING as u16;

/// An enum representing the different kinds of nodes that can be parsed.
pub enum Node<'pr> {
    /// The `AliasGlobalVariableNode` node
    AliasGlobalVariableNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_alias_global_variable_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_alias_global_variable_node_t>
    },
    /// The `AliasMethodNode` node
    AliasMethodNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_alias_method_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_alias_method_node_t>
    },
    /// The `AlternationPatternNode` node
    AlternationPatternNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_alternation_pattern_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_alternation_pattern_node_t>
    },
    /// The `AndNode` node
    AndNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_and_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_and_node_t>
    },
    /// The `ArgumentsNode` node
    ArgumentsNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_arguments_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_arguments_node_t>
    },
    /// The `ArrayNode` node
    ArrayNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_array_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_array_node_t>
    },
    /// The `ArrayPatternNode` node
    ArrayPatternNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_array_pattern_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_array_pattern_node_t>
    },
    /// The `AssocNode` node
    AssocNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_assoc_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_assoc_node_t>
    },
    /// The `AssocSplatNode` node
    AssocSplatNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_assoc_splat_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_assoc_splat_node_t>
    },
    /// The `BackReferenceReadNode` node
    BackReferenceReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_back_reference_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_back_reference_read_node_t>
    },
    /// The `BeginNode` node
    BeginNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_begin_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_begin_node_t>
    },
    /// The `BlockArgumentNode` node
    BlockArgumentNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_block_argument_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_block_argument_node_t>
    },
    /// The `BlockLocalVariableNode` node
    BlockLocalVariableNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_block_local_variable_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_block_local_variable_node_t>
    },
    /// The `BlockNode` node
    BlockNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_block_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_block_node_t>
    },
    /// The `BlockParameterNode` node
    BlockParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_block_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_block_parameter_node_t>
    },
    /// The `BlockParametersNode` node
    BlockParametersNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_block_parameters_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_block_parameters_node_t>
    },
    /// The `BreakNode` node
    BreakNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_break_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_break_node_t>
    },
    /// The `CallAndWriteNode` node
    CallAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_call_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_call_and_write_node_t>
    },
    /// The `CallNode` node
    CallNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_call_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_call_node_t>
    },
    /// The `CallOperatorWriteNode` node
    CallOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_call_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_call_operator_write_node_t>
    },
    /// The `CallOrWriteNode` node
    CallOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_call_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_call_or_write_node_t>
    },
    /// The `CallTargetNode` node
    CallTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_call_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_call_target_node_t>
    },
    /// The `CapturePatternNode` node
    CapturePatternNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_capture_pattern_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_capture_pattern_node_t>
    },
    /// The `CaseMatchNode` node
    CaseMatchNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_case_match_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_case_match_node_t>
    },
    /// The `CaseNode` node
    CaseNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_case_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_case_node_t>
    },
    /// The `ClassNode` node
    ClassNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_node_t>
    },
    /// The `ClassVariableAndWriteNode` node
    ClassVariableAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_variable_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_variable_and_write_node_t>
    },
    /// The `ClassVariableOperatorWriteNode` node
    ClassVariableOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_variable_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_variable_operator_write_node_t>
    },
    /// The `ClassVariableOrWriteNode` node
    ClassVariableOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_variable_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_variable_or_write_node_t>
    },
    /// The `ClassVariableReadNode` node
    ClassVariableReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_variable_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_variable_read_node_t>
    },
    /// The `ClassVariableTargetNode` node
    ClassVariableTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_variable_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_variable_target_node_t>
    },
    /// The `ClassVariableWriteNode` node
    ClassVariableWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_class_variable_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_class_variable_write_node_t>
    },
    /// The `ConstantAndWriteNode` node
    ConstantAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_and_write_node_t>
    },
    /// The `ConstantOperatorWriteNode` node
    ConstantOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_operator_write_node_t>
    },
    /// The `ConstantOrWriteNode` node
    ConstantOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_or_write_node_t>
    },
    /// The `ConstantPathAndWriteNode` node
    ConstantPathAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_path_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_path_and_write_node_t>
    },
    /// The `ConstantPathNode` node
    ConstantPathNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_path_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_path_node_t>
    },
    /// The `ConstantPathOperatorWriteNode` node
    ConstantPathOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_path_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_path_operator_write_node_t>
    },
    /// The `ConstantPathOrWriteNode` node
    ConstantPathOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_path_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_path_or_write_node_t>
    },
    /// The `ConstantPathTargetNode` node
    ConstantPathTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_path_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_path_target_node_t>
    },
    /// The `ConstantPathWriteNode` node
    ConstantPathWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_path_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_path_write_node_t>
    },
    /// The `ConstantReadNode` node
    ConstantReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_read_node_t>
    },
    /// The `ConstantTargetNode` node
    ConstantTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_target_node_t>
    },
    /// The `ConstantWriteNode` node
    ConstantWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_constant_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_constant_write_node_t>
    },
    /// The `DefNode` node
    DefNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_def_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_def_node_t>
    },
    /// The `DefinedNode` node
    DefinedNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_defined_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_defined_node_t>
    },
    /// The `ElseNode` node
    ElseNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_else_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_else_node_t>
    },
    /// The `EmbeddedStatementsNode` node
    EmbeddedStatementsNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_embedded_statements_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_embedded_statements_node_t>
    },
    /// The `EmbeddedVariableNode` node
    EmbeddedVariableNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_embedded_variable_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_embedded_variable_node_t>
    },
    /// The `EnsureNode` node
    EnsureNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_ensure_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_ensure_node_t>
    },
    /// The `FalseNode` node
    FalseNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_false_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_false_node_t>
    },
    /// The `FindPatternNode` node
    FindPatternNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_find_pattern_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_find_pattern_node_t>
    },
    /// The `FlipFlopNode` node
    FlipFlopNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_flip_flop_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_flip_flop_node_t>
    },
    /// The `FloatNode` node
    FloatNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_float_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_float_node_t>
    },
    /// The `ForNode` node
    ForNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_for_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_for_node_t>
    },
    /// The `ForwardingArgumentsNode` node
    ForwardingArgumentsNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_forwarding_arguments_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_forwarding_arguments_node_t>
    },
    /// The `ForwardingParameterNode` node
    ForwardingParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_forwarding_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_forwarding_parameter_node_t>
    },
    /// The `ForwardingSuperNode` node
    ForwardingSuperNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_forwarding_super_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_forwarding_super_node_t>
    },
    /// The `GlobalVariableAndWriteNode` node
    GlobalVariableAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_global_variable_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_global_variable_and_write_node_t>
    },
    /// The `GlobalVariableOperatorWriteNode` node
    GlobalVariableOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_global_variable_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_global_variable_operator_write_node_t>
    },
    /// The `GlobalVariableOrWriteNode` node
    GlobalVariableOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_global_variable_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_global_variable_or_write_node_t>
    },
    /// The `GlobalVariableReadNode` node
    GlobalVariableReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_global_variable_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_global_variable_read_node_t>
    },
    /// The `GlobalVariableTargetNode` node
    GlobalVariableTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_global_variable_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_global_variable_target_node_t>
    },
    /// The `GlobalVariableWriteNode` node
    GlobalVariableWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_global_variable_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_global_variable_write_node_t>
    },
    /// The `HashNode` node
    HashNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_hash_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_hash_node_t>
    },
    /// The `HashPatternNode` node
    HashPatternNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_hash_pattern_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_hash_pattern_node_t>
    },
    /// The `IfNode` node
    IfNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_if_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_if_node_t>
    },
    /// The `ImaginaryNode` node
    ImaginaryNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_imaginary_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_imaginary_node_t>
    },
    /// The `ImplicitNode` node
    ImplicitNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_implicit_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_implicit_node_t>
    },
    /// The `ImplicitRestNode` node
    ImplicitRestNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_implicit_rest_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_implicit_rest_node_t>
    },
    /// The `InNode` node
    InNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_in_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_in_node_t>
    },
    /// The `IndexAndWriteNode` node
    IndexAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_index_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_index_and_write_node_t>
    },
    /// The `IndexOperatorWriteNode` node
    IndexOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_index_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_index_operator_write_node_t>
    },
    /// The `IndexOrWriteNode` node
    IndexOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_index_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_index_or_write_node_t>
    },
    /// The `IndexTargetNode` node
    IndexTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_index_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_index_target_node_t>
    },
    /// The `InstanceVariableAndWriteNode` node
    InstanceVariableAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_instance_variable_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_instance_variable_and_write_node_t>
    },
    /// The `InstanceVariableOperatorWriteNode` node
    InstanceVariableOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_instance_variable_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_instance_variable_operator_write_node_t>
    },
    /// The `InstanceVariableOrWriteNode` node
    InstanceVariableOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_instance_variable_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_instance_variable_or_write_node_t>
    },
    /// The `InstanceVariableReadNode` node
    InstanceVariableReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_instance_variable_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_instance_variable_read_node_t>
    },
    /// The `InstanceVariableTargetNode` node
    InstanceVariableTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_instance_variable_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_instance_variable_target_node_t>
    },
    /// The `InstanceVariableWriteNode` node
    InstanceVariableWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_instance_variable_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_instance_variable_write_node_t>
    },
    /// The `IntegerNode` node
    IntegerNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_integer_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_integer_node_t>
    },
    /// The `InterpolatedMatchLastLineNode` node
    InterpolatedMatchLastLineNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_interpolated_match_last_line_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_interpolated_match_last_line_node_t>
    },
    /// The `InterpolatedRegularExpressionNode` node
    InterpolatedRegularExpressionNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_interpolated_regular_expression_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_interpolated_regular_expression_node_t>
    },
    /// The `InterpolatedStringNode` node
    InterpolatedStringNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_interpolated_string_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_interpolated_string_node_t>
    },
    /// The `InterpolatedSymbolNode` node
    InterpolatedSymbolNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_interpolated_symbol_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_interpolated_symbol_node_t>
    },
    /// The `InterpolatedXStringNode` node
    InterpolatedXStringNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_interpolated_x_string_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_interpolated_x_string_node_t>
    },
    /// The `ItLocalVariableReadNode` node
    ItLocalVariableReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_it_local_variable_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_it_local_variable_read_node_t>
    },
    /// The `ItParametersNode` node
    ItParametersNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_it_parameters_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_it_parameters_node_t>
    },
    /// The `KeywordHashNode` node
    KeywordHashNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_keyword_hash_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_keyword_hash_node_t>
    },
    /// The `KeywordRestParameterNode` node
    KeywordRestParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_keyword_rest_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_keyword_rest_parameter_node_t>
    },
    /// The `LambdaNode` node
    LambdaNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_lambda_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_lambda_node_t>
    },
    /// The `LocalVariableAndWriteNode` node
    LocalVariableAndWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_local_variable_and_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_local_variable_and_write_node_t>
    },
    /// The `LocalVariableOperatorWriteNode` node
    LocalVariableOperatorWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_local_variable_operator_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_local_variable_operator_write_node_t>
    },
    /// The `LocalVariableOrWriteNode` node
    LocalVariableOrWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_local_variable_or_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_local_variable_or_write_node_t>
    },
    /// The `LocalVariableReadNode` node
    LocalVariableReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_local_variable_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_local_variable_read_node_t>
    },
    /// The `LocalVariableTargetNode` node
    LocalVariableTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_local_variable_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_local_variable_target_node_t>
    },
    /// The `LocalVariableWriteNode` node
    LocalVariableWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_local_variable_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_local_variable_write_node_t>
    },
    /// The `MatchLastLineNode` node
    MatchLastLineNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_match_last_line_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_match_last_line_node_t>
    },
    /// The `MatchPredicateNode` node
    MatchPredicateNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_match_predicate_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_match_predicate_node_t>
    },
    /// The `MatchRequiredNode` node
    MatchRequiredNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_match_required_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_match_required_node_t>
    },
    /// The `MatchWriteNode` node
    MatchWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_match_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_match_write_node_t>
    },
    /// The `MissingNode` node
    MissingNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_missing_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_missing_node_t>
    },
    /// The `ModuleNode` node
    ModuleNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_module_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_module_node_t>
    },
    /// The `MultiTargetNode` node
    MultiTargetNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_multi_target_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_multi_target_node_t>
    },
    /// The `MultiWriteNode` node
    MultiWriteNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_multi_write_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_multi_write_node_t>
    },
    /// The `NextNode` node
    NextNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_next_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_next_node_t>
    },
    /// The `NilNode` node
    NilNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_nil_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_nil_node_t>
    },
    /// The `NoKeywordsParameterNode` node
    NoKeywordsParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_no_keywords_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_no_keywords_parameter_node_t>
    },
    /// The `NumberedParametersNode` node
    NumberedParametersNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_numbered_parameters_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_numbered_parameters_node_t>
    },
    /// The `NumberedReferenceReadNode` node
    NumberedReferenceReadNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_numbered_reference_read_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_numbered_reference_read_node_t>
    },
    /// The `OptionalKeywordParameterNode` node
    OptionalKeywordParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_optional_keyword_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_optional_keyword_parameter_node_t>
    },
    /// The `OptionalParameterNode` node
    OptionalParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_optional_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_optional_parameter_node_t>
    },
    /// The `OrNode` node
    OrNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_or_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_or_node_t>
    },
    /// The `ParametersNode` node
    ParametersNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_parameters_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_parameters_node_t>
    },
    /// The `ParenthesesNode` node
    ParenthesesNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_parentheses_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_parentheses_node_t>
    },
    /// The `PinnedExpressionNode` node
    PinnedExpressionNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_pinned_expression_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_pinned_expression_node_t>
    },
    /// The `PinnedVariableNode` node
    PinnedVariableNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_pinned_variable_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_pinned_variable_node_t>
    },
    /// The `PostExecutionNode` node
    PostExecutionNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_post_execution_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_post_execution_node_t>
    },
    /// The `PreExecutionNode` node
    PreExecutionNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_pre_execution_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_pre_execution_node_t>
    },
    /// The `ProgramNode` node
    ProgramNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_program_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_program_node_t>
    },
    /// The `RangeNode` node
    RangeNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_range_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_range_node_t>
    },
    /// The `RationalNode` node
    RationalNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_rational_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_rational_node_t>
    },
    /// The `RedoNode` node
    RedoNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_redo_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_redo_node_t>
    },
    /// The `RegularExpressionNode` node
    RegularExpressionNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_regular_expression_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_regular_expression_node_t>
    },
    /// The `RequiredKeywordParameterNode` node
    RequiredKeywordParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_required_keyword_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_required_keyword_parameter_node_t>
    },
    /// The `RequiredParameterNode` node
    RequiredParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_required_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_required_parameter_node_t>
    },
    /// The `RescueModifierNode` node
    RescueModifierNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_rescue_modifier_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_rescue_modifier_node_t>
    },
    /// The `RescueNode` node
    RescueNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_rescue_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_rescue_node_t>
    },
    /// The `RestParameterNode` node
    RestParameterNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_rest_parameter_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_rest_parameter_node_t>
    },
    /// The `RetryNode` node
    RetryNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_retry_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_retry_node_t>
    },
    /// The `ReturnNode` node
    ReturnNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_return_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_return_node_t>
    },
    /// The `SelfNode` node
    SelfNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_self_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_self_node_t>
    },
    /// The `ShareableConstantNode` node
    ShareableConstantNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_shareable_constant_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_shareable_constant_node_t>
    },
    /// The `SingletonClassNode` node
    SingletonClassNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_singleton_class_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_singleton_class_node_t>
    },
    /// The `SourceEncodingNode` node
    SourceEncodingNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_source_encoding_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_source_encoding_node_t>
    },
    /// The `SourceFileNode` node
    SourceFileNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_source_file_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_source_file_node_t>
    },
    /// The `SourceLineNode` node
    SourceLineNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_source_line_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_source_line_node_t>
    },
    /// The `SplatNode` node
    SplatNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_splat_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_splat_node_t>
    },
    /// The `StatementsNode` node
    StatementsNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_statements_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_statements_node_t>
    },
    /// The `StringNode` node
    StringNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_string_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_string_node_t>
    },
    /// The `SuperNode` node
    SuperNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_super_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_super_node_t>
    },
    /// The `SymbolNode` node
    SymbolNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_symbol_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_symbol_node_t>
    },
    /// The `TrueNode` node
    TrueNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_true_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_true_node_t>
    },
    /// The `UndefNode` node
    UndefNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_undef_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_undef_node_t>
    },
    /// The `UnlessNode` node
    UnlessNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_unless_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_unless_node_t>
    },
    /// The `UntilNode` node
    UntilNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_until_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_until_node_t>
    },
    /// The `WhenNode` node
    WhenNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_when_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_when_node_t>
    },
    /// The `WhileNode` node
    WhileNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_while_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_while_node_t>
    },
    /// The `XStringNode` node
    XStringNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_x_string_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_x_string_node_t>
    },
    /// The `YieldNode` node
    YieldNode {
        /// The pointer to the associated parser this node came from.
        parser: NonNull<pm_parser_t>,

        /// The raw pointer to the node allocated by prism.
        pointer: *mut pm_yield_node_t,

        /// The marker to indicate the lifetime of the pointer.
        marker: PhantomData<&'pr mut pm_yield_node_t>
    },
}


impl<'pr> Node<'pr> {
    /// Creates a new node from the given pointer.
    ///
    /// # Panics
    ///
    /// Panics if the node type cannot be read.
    ///
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub(crate) fn new(parser: NonNull<pm_parser_t>, node: *mut pm_node_t) -> Self {
        match unsafe { (*node).type_ } {

            PM_ALIAS_GLOBAL_VARIABLE_NODE => Self::AliasGlobalVariableNode { parser, pointer: node.cast::<pm_alias_global_variable_node_t>(), marker: PhantomData },
            PM_ALIAS_METHOD_NODE => Self::AliasMethodNode { parser, pointer: node.cast::<pm_alias_method_node_t>(), marker: PhantomData },
            PM_ALTERNATION_PATTERN_NODE => Self::AlternationPatternNode { parser, pointer: node.cast::<pm_alternation_pattern_node_t>(), marker: PhantomData },
            PM_AND_NODE => Self::AndNode { parser, pointer: node.cast::<pm_and_node_t>(), marker: PhantomData },
            PM_ARGUMENTS_NODE => Self::ArgumentsNode { parser, pointer: node.cast::<pm_arguments_node_t>(), marker: PhantomData },
            PM_ARRAY_NODE => Self::ArrayNode { parser, pointer: node.cast::<pm_array_node_t>(), marker: PhantomData },
            PM_ARRAY_PATTERN_NODE => Self::ArrayPatternNode { parser, pointer: node.cast::<pm_array_pattern_node_t>(), marker: PhantomData },
            PM_ASSOC_NODE => Self::AssocNode { parser, pointer: node.cast::<pm_assoc_node_t>(), marker: PhantomData },
            PM_ASSOC_SPLAT_NODE => Self::AssocSplatNode { parser, pointer: node.cast::<pm_assoc_splat_node_t>(), marker: PhantomData },
            PM_BACK_REFERENCE_READ_NODE => Self::BackReferenceReadNode { parser, pointer: node.cast::<pm_back_reference_read_node_t>(), marker: PhantomData },
            PM_BEGIN_NODE => Self::BeginNode { parser, pointer: node.cast::<pm_begin_node_t>(), marker: PhantomData },
            PM_BLOCK_ARGUMENT_NODE => Self::BlockArgumentNode { parser, pointer: node.cast::<pm_block_argument_node_t>(), marker: PhantomData },
            PM_BLOCK_LOCAL_VARIABLE_NODE => Self::BlockLocalVariableNode { parser, pointer: node.cast::<pm_block_local_variable_node_t>(), marker: PhantomData },
            PM_BLOCK_NODE => Self::BlockNode { parser, pointer: node.cast::<pm_block_node_t>(), marker: PhantomData },
            PM_BLOCK_PARAMETER_NODE => Self::BlockParameterNode { parser, pointer: node.cast::<pm_block_parameter_node_t>(), marker: PhantomData },
            PM_BLOCK_PARAMETERS_NODE => Self::BlockParametersNode { parser, pointer: node.cast::<pm_block_parameters_node_t>(), marker: PhantomData },
            PM_BREAK_NODE => Self::BreakNode { parser, pointer: node.cast::<pm_break_node_t>(), marker: PhantomData },
            PM_CALL_AND_WRITE_NODE => Self::CallAndWriteNode { parser, pointer: node.cast::<pm_call_and_write_node_t>(), marker: PhantomData },
            PM_CALL_NODE => Self::CallNode { parser, pointer: node.cast::<pm_call_node_t>(), marker: PhantomData },
            PM_CALL_OPERATOR_WRITE_NODE => Self::CallOperatorWriteNode { parser, pointer: node.cast::<pm_call_operator_write_node_t>(), marker: PhantomData },
            PM_CALL_OR_WRITE_NODE => Self::CallOrWriteNode { parser, pointer: node.cast::<pm_call_or_write_node_t>(), marker: PhantomData },
            PM_CALL_TARGET_NODE => Self::CallTargetNode { parser, pointer: node.cast::<pm_call_target_node_t>(), marker: PhantomData },
            PM_CAPTURE_PATTERN_NODE => Self::CapturePatternNode { parser, pointer: node.cast::<pm_capture_pattern_node_t>(), marker: PhantomData },
            PM_CASE_MATCH_NODE => Self::CaseMatchNode { parser, pointer: node.cast::<pm_case_match_node_t>(), marker: PhantomData },
            PM_CASE_NODE => Self::CaseNode { parser, pointer: node.cast::<pm_case_node_t>(), marker: PhantomData },
            PM_CLASS_NODE => Self::ClassNode { parser, pointer: node.cast::<pm_class_node_t>(), marker: PhantomData },
            PM_CLASS_VARIABLE_AND_WRITE_NODE => Self::ClassVariableAndWriteNode { parser, pointer: node.cast::<pm_class_variable_and_write_node_t>(), marker: PhantomData },
            PM_CLASS_VARIABLE_OPERATOR_WRITE_NODE => Self::ClassVariableOperatorWriteNode { parser, pointer: node.cast::<pm_class_variable_operator_write_node_t>(), marker: PhantomData },
            PM_CLASS_VARIABLE_OR_WRITE_NODE => Self::ClassVariableOrWriteNode { parser, pointer: node.cast::<pm_class_variable_or_write_node_t>(), marker: PhantomData },
            PM_CLASS_VARIABLE_READ_NODE => Self::ClassVariableReadNode { parser, pointer: node.cast::<pm_class_variable_read_node_t>(), marker: PhantomData },
            PM_CLASS_VARIABLE_TARGET_NODE => Self::ClassVariableTargetNode { parser, pointer: node.cast::<pm_class_variable_target_node_t>(), marker: PhantomData },
            PM_CLASS_VARIABLE_WRITE_NODE => Self::ClassVariableWriteNode { parser, pointer: node.cast::<pm_class_variable_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_AND_WRITE_NODE => Self::ConstantAndWriteNode { parser, pointer: node.cast::<pm_constant_and_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_OPERATOR_WRITE_NODE => Self::ConstantOperatorWriteNode { parser, pointer: node.cast::<pm_constant_operator_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_OR_WRITE_NODE => Self::ConstantOrWriteNode { parser, pointer: node.cast::<pm_constant_or_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_PATH_AND_WRITE_NODE => Self::ConstantPathAndWriteNode { parser, pointer: node.cast::<pm_constant_path_and_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_PATH_NODE => Self::ConstantPathNode { parser, pointer: node.cast::<pm_constant_path_node_t>(), marker: PhantomData },
            PM_CONSTANT_PATH_OPERATOR_WRITE_NODE => Self::ConstantPathOperatorWriteNode { parser, pointer: node.cast::<pm_constant_path_operator_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_PATH_OR_WRITE_NODE => Self::ConstantPathOrWriteNode { parser, pointer: node.cast::<pm_constant_path_or_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_PATH_TARGET_NODE => Self::ConstantPathTargetNode { parser, pointer: node.cast::<pm_constant_path_target_node_t>(), marker: PhantomData },
            PM_CONSTANT_PATH_WRITE_NODE => Self::ConstantPathWriteNode { parser, pointer: node.cast::<pm_constant_path_write_node_t>(), marker: PhantomData },
            PM_CONSTANT_READ_NODE => Self::ConstantReadNode { parser, pointer: node.cast::<pm_constant_read_node_t>(), marker: PhantomData },
            PM_CONSTANT_TARGET_NODE => Self::ConstantTargetNode { parser, pointer: node.cast::<pm_constant_target_node_t>(), marker: PhantomData },
            PM_CONSTANT_WRITE_NODE => Self::ConstantWriteNode { parser, pointer: node.cast::<pm_constant_write_node_t>(), marker: PhantomData },
            PM_DEF_NODE => Self::DefNode { parser, pointer: node.cast::<pm_def_node_t>(), marker: PhantomData },
            PM_DEFINED_NODE => Self::DefinedNode { parser, pointer: node.cast::<pm_defined_node_t>(), marker: PhantomData },
            PM_ELSE_NODE => Self::ElseNode { parser, pointer: node.cast::<pm_else_node_t>(), marker: PhantomData },
            PM_EMBEDDED_STATEMENTS_NODE => Self::EmbeddedStatementsNode { parser, pointer: node.cast::<pm_embedded_statements_node_t>(), marker: PhantomData },
            PM_EMBEDDED_VARIABLE_NODE => Self::EmbeddedVariableNode { parser, pointer: node.cast::<pm_embedded_variable_node_t>(), marker: PhantomData },
            PM_ENSURE_NODE => Self::EnsureNode { parser, pointer: node.cast::<pm_ensure_node_t>(), marker: PhantomData },
            PM_FALSE_NODE => Self::FalseNode { parser, pointer: node.cast::<pm_false_node_t>(), marker: PhantomData },
            PM_FIND_PATTERN_NODE => Self::FindPatternNode { parser, pointer: node.cast::<pm_find_pattern_node_t>(), marker: PhantomData },
            PM_FLIP_FLOP_NODE => Self::FlipFlopNode { parser, pointer: node.cast::<pm_flip_flop_node_t>(), marker: PhantomData },
            PM_FLOAT_NODE => Self::FloatNode { parser, pointer: node.cast::<pm_float_node_t>(), marker: PhantomData },
            PM_FOR_NODE => Self::ForNode { parser, pointer: node.cast::<pm_for_node_t>(), marker: PhantomData },
            PM_FORWARDING_ARGUMENTS_NODE => Self::ForwardingArgumentsNode { parser, pointer: node.cast::<pm_forwarding_arguments_node_t>(), marker: PhantomData },
            PM_FORWARDING_PARAMETER_NODE => Self::ForwardingParameterNode { parser, pointer: node.cast::<pm_forwarding_parameter_node_t>(), marker: PhantomData },
            PM_FORWARDING_SUPER_NODE => Self::ForwardingSuperNode { parser, pointer: node.cast::<pm_forwarding_super_node_t>(), marker: PhantomData },
            PM_GLOBAL_VARIABLE_AND_WRITE_NODE => Self::GlobalVariableAndWriteNode { parser, pointer: node.cast::<pm_global_variable_and_write_node_t>(), marker: PhantomData },
            PM_GLOBAL_VARIABLE_OPERATOR_WRITE_NODE => Self::GlobalVariableOperatorWriteNode { parser, pointer: node.cast::<pm_global_variable_operator_write_node_t>(), marker: PhantomData },
            PM_GLOBAL_VARIABLE_OR_WRITE_NODE => Self::GlobalVariableOrWriteNode { parser, pointer: node.cast::<pm_global_variable_or_write_node_t>(), marker: PhantomData },
            PM_GLOBAL_VARIABLE_READ_NODE => Self::GlobalVariableReadNode { parser, pointer: node.cast::<pm_global_variable_read_node_t>(), marker: PhantomData },
            PM_GLOBAL_VARIABLE_TARGET_NODE => Self::GlobalVariableTargetNode { parser, pointer: node.cast::<pm_global_variable_target_node_t>(), marker: PhantomData },
            PM_GLOBAL_VARIABLE_WRITE_NODE => Self::GlobalVariableWriteNode { parser, pointer: node.cast::<pm_global_variable_write_node_t>(), marker: PhantomData },
            PM_HASH_NODE => Self::HashNode { parser, pointer: node.cast::<pm_hash_node_t>(), marker: PhantomData },
            PM_HASH_PATTERN_NODE => Self::HashPatternNode { parser, pointer: node.cast::<pm_hash_pattern_node_t>(), marker: PhantomData },
            PM_IF_NODE => Self::IfNode { parser, pointer: node.cast::<pm_if_node_t>(), marker: PhantomData },
            PM_IMAGINARY_NODE => Self::ImaginaryNode { parser, pointer: node.cast::<pm_imaginary_node_t>(), marker: PhantomData },
            PM_IMPLICIT_NODE => Self::ImplicitNode { parser, pointer: node.cast::<pm_implicit_node_t>(), marker: PhantomData },
            PM_IMPLICIT_REST_NODE => Self::ImplicitRestNode { parser, pointer: node.cast::<pm_implicit_rest_node_t>(), marker: PhantomData },
            PM_IN_NODE => Self::InNode { parser, pointer: node.cast::<pm_in_node_t>(), marker: PhantomData },
            PM_INDEX_AND_WRITE_NODE => Self::IndexAndWriteNode { parser, pointer: node.cast::<pm_index_and_write_node_t>(), marker: PhantomData },
            PM_INDEX_OPERATOR_WRITE_NODE => Self::IndexOperatorWriteNode { parser, pointer: node.cast::<pm_index_operator_write_node_t>(), marker: PhantomData },
            PM_INDEX_OR_WRITE_NODE => Self::IndexOrWriteNode { parser, pointer: node.cast::<pm_index_or_write_node_t>(), marker: PhantomData },
            PM_INDEX_TARGET_NODE => Self::IndexTargetNode { parser, pointer: node.cast::<pm_index_target_node_t>(), marker: PhantomData },
            PM_INSTANCE_VARIABLE_AND_WRITE_NODE => Self::InstanceVariableAndWriteNode { parser, pointer: node.cast::<pm_instance_variable_and_write_node_t>(), marker: PhantomData },
            PM_INSTANCE_VARIABLE_OPERATOR_WRITE_NODE => Self::InstanceVariableOperatorWriteNode { parser, pointer: node.cast::<pm_instance_variable_operator_write_node_t>(), marker: PhantomData },
            PM_INSTANCE_VARIABLE_OR_WRITE_NODE => Self::InstanceVariableOrWriteNode { parser, pointer: node.cast::<pm_instance_variable_or_write_node_t>(), marker: PhantomData },
            PM_INSTANCE_VARIABLE_READ_NODE => Self::InstanceVariableReadNode { parser, pointer: node.cast::<pm_instance_variable_read_node_t>(), marker: PhantomData },
            PM_INSTANCE_VARIABLE_TARGET_NODE => Self::InstanceVariableTargetNode { parser, pointer: node.cast::<pm_instance_variable_target_node_t>(), marker: PhantomData },
            PM_INSTANCE_VARIABLE_WRITE_NODE => Self::InstanceVariableWriteNode { parser, pointer: node.cast::<pm_instance_variable_write_node_t>(), marker: PhantomData },
            PM_INTEGER_NODE => Self::IntegerNode { parser, pointer: node.cast::<pm_integer_node_t>(), marker: PhantomData },
            PM_INTERPOLATED_MATCH_LAST_LINE_NODE => Self::InterpolatedMatchLastLineNode { parser, pointer: node.cast::<pm_interpolated_match_last_line_node_t>(), marker: PhantomData },
            PM_INTERPOLATED_REGULAR_EXPRESSION_NODE => Self::InterpolatedRegularExpressionNode { parser, pointer: node.cast::<pm_interpolated_regular_expression_node_t>(), marker: PhantomData },
            PM_INTERPOLATED_STRING_NODE => Self::InterpolatedStringNode { parser, pointer: node.cast::<pm_interpolated_string_node_t>(), marker: PhantomData },
            PM_INTERPOLATED_SYMBOL_NODE => Self::InterpolatedSymbolNode { parser, pointer: node.cast::<pm_interpolated_symbol_node_t>(), marker: PhantomData },
            PM_INTERPOLATED_X_STRING_NODE => Self::InterpolatedXStringNode { parser, pointer: node.cast::<pm_interpolated_x_string_node_t>(), marker: PhantomData },
            PM_IT_LOCAL_VARIABLE_READ_NODE => Self::ItLocalVariableReadNode { parser, pointer: node.cast::<pm_it_local_variable_read_node_t>(), marker: PhantomData },
            PM_IT_PARAMETERS_NODE => Self::ItParametersNode { parser, pointer: node.cast::<pm_it_parameters_node_t>(), marker: PhantomData },
            PM_KEYWORD_HASH_NODE => Self::KeywordHashNode { parser, pointer: node.cast::<pm_keyword_hash_node_t>(), marker: PhantomData },
            PM_KEYWORD_REST_PARAMETER_NODE => Self::KeywordRestParameterNode { parser, pointer: node.cast::<pm_keyword_rest_parameter_node_t>(), marker: PhantomData },
            PM_LAMBDA_NODE => Self::LambdaNode { parser, pointer: node.cast::<pm_lambda_node_t>(), marker: PhantomData },
            PM_LOCAL_VARIABLE_AND_WRITE_NODE => Self::LocalVariableAndWriteNode { parser, pointer: node.cast::<pm_local_variable_and_write_node_t>(), marker: PhantomData },
            PM_LOCAL_VARIABLE_OPERATOR_WRITE_NODE => Self::LocalVariableOperatorWriteNode { parser, pointer: node.cast::<pm_local_variable_operator_write_node_t>(), marker: PhantomData },
            PM_LOCAL_VARIABLE_OR_WRITE_NODE => Self::LocalVariableOrWriteNode { parser, pointer: node.cast::<pm_local_variable_or_write_node_t>(), marker: PhantomData },
            PM_LOCAL_VARIABLE_READ_NODE => Self::LocalVariableReadNode { parser, pointer: node.cast::<pm_local_variable_read_node_t>(), marker: PhantomData },
            PM_LOCAL_VARIABLE_TARGET_NODE => Self::LocalVariableTargetNode { parser, pointer: node.cast::<pm_local_variable_target_node_t>(), marker: PhantomData },
            PM_LOCAL_VARIABLE_WRITE_NODE => Self::LocalVariableWriteNode { parser, pointer: node.cast::<pm_local_variable_write_node_t>(), marker: PhantomData },
            PM_MATCH_LAST_LINE_NODE => Self::MatchLastLineNode { parser, pointer: node.cast::<pm_match_last_line_node_t>(), marker: PhantomData },
            PM_MATCH_PREDICATE_NODE => Self::MatchPredicateNode { parser, pointer: node.cast::<pm_match_predicate_node_t>(), marker: PhantomData },
            PM_MATCH_REQUIRED_NODE => Self::MatchRequiredNode { parser, pointer: node.cast::<pm_match_required_node_t>(), marker: PhantomData },
            PM_MATCH_WRITE_NODE => Self::MatchWriteNode { parser, pointer: node.cast::<pm_match_write_node_t>(), marker: PhantomData },
            PM_MISSING_NODE => Self::MissingNode { parser, pointer: node.cast::<pm_missing_node_t>(), marker: PhantomData },
            PM_MODULE_NODE => Self::ModuleNode { parser, pointer: node.cast::<pm_module_node_t>(), marker: PhantomData },
            PM_MULTI_TARGET_NODE => Self::MultiTargetNode { parser, pointer: node.cast::<pm_multi_target_node_t>(), marker: PhantomData },
            PM_MULTI_WRITE_NODE => Self::MultiWriteNode { parser, pointer: node.cast::<pm_multi_write_node_t>(), marker: PhantomData },
            PM_NEXT_NODE => Self::NextNode { parser, pointer: node.cast::<pm_next_node_t>(), marker: PhantomData },
            PM_NIL_NODE => Self::NilNode { parser, pointer: node.cast::<pm_nil_node_t>(), marker: PhantomData },
            PM_NO_KEYWORDS_PARAMETER_NODE => Self::NoKeywordsParameterNode { parser, pointer: node.cast::<pm_no_keywords_parameter_node_t>(), marker: PhantomData },
            PM_NUMBERED_PARAMETERS_NODE => Self::NumberedParametersNode { parser, pointer: node.cast::<pm_numbered_parameters_node_t>(), marker: PhantomData },
            PM_NUMBERED_REFERENCE_READ_NODE => Self::NumberedReferenceReadNode { parser, pointer: node.cast::<pm_numbered_reference_read_node_t>(), marker: PhantomData },
            PM_OPTIONAL_KEYWORD_PARAMETER_NODE => Self::OptionalKeywordParameterNode { parser, pointer: node.cast::<pm_optional_keyword_parameter_node_t>(), marker: PhantomData },
            PM_OPTIONAL_PARAMETER_NODE => Self::OptionalParameterNode { parser, pointer: node.cast::<pm_optional_parameter_node_t>(), marker: PhantomData },
            PM_OR_NODE => Self::OrNode { parser, pointer: node.cast::<pm_or_node_t>(), marker: PhantomData },
            PM_PARAMETERS_NODE => Self::ParametersNode { parser, pointer: node.cast::<pm_parameters_node_t>(), marker: PhantomData },
            PM_PARENTHESES_NODE => Self::ParenthesesNode { parser, pointer: node.cast::<pm_parentheses_node_t>(), marker: PhantomData },
            PM_PINNED_EXPRESSION_NODE => Self::PinnedExpressionNode { parser, pointer: node.cast::<pm_pinned_expression_node_t>(), marker: PhantomData },
            PM_PINNED_VARIABLE_NODE => Self::PinnedVariableNode { parser, pointer: node.cast::<pm_pinned_variable_node_t>(), marker: PhantomData },
            PM_POST_EXECUTION_NODE => Self::PostExecutionNode { parser, pointer: node.cast::<pm_post_execution_node_t>(), marker: PhantomData },
            PM_PRE_EXECUTION_NODE => Self::PreExecutionNode { parser, pointer: node.cast::<pm_pre_execution_node_t>(), marker: PhantomData },
            PM_PROGRAM_NODE => Self::ProgramNode { parser, pointer: node.cast::<pm_program_node_t>(), marker: PhantomData },
            PM_RANGE_NODE => Self::RangeNode { parser, pointer: node.cast::<pm_range_node_t>(), marker: PhantomData },
            PM_RATIONAL_NODE => Self::RationalNode { parser, pointer: node.cast::<pm_rational_node_t>(), marker: PhantomData },
            PM_REDO_NODE => Self::RedoNode { parser, pointer: node.cast::<pm_redo_node_t>(), marker: PhantomData },
            PM_REGULAR_EXPRESSION_NODE => Self::RegularExpressionNode { parser, pointer: node.cast::<pm_regular_expression_node_t>(), marker: PhantomData },
            PM_REQUIRED_KEYWORD_PARAMETER_NODE => Self::RequiredKeywordParameterNode { parser, pointer: node.cast::<pm_required_keyword_parameter_node_t>(), marker: PhantomData },
            PM_REQUIRED_PARAMETER_NODE => Self::RequiredParameterNode { parser, pointer: node.cast::<pm_required_parameter_node_t>(), marker: PhantomData },
            PM_RESCUE_MODIFIER_NODE => Self::RescueModifierNode { parser, pointer: node.cast::<pm_rescue_modifier_node_t>(), marker: PhantomData },
            PM_RESCUE_NODE => Self::RescueNode { parser, pointer: node.cast::<pm_rescue_node_t>(), marker: PhantomData },
            PM_REST_PARAMETER_NODE => Self::RestParameterNode { parser, pointer: node.cast::<pm_rest_parameter_node_t>(), marker: PhantomData },
            PM_RETRY_NODE => Self::RetryNode { parser, pointer: node.cast::<pm_retry_node_t>(), marker: PhantomData },
            PM_RETURN_NODE => Self::ReturnNode { parser, pointer: node.cast::<pm_return_node_t>(), marker: PhantomData },
            PM_SELF_NODE => Self::SelfNode { parser, pointer: node.cast::<pm_self_node_t>(), marker: PhantomData },
            PM_SHAREABLE_CONSTANT_NODE => Self::ShareableConstantNode { parser, pointer: node.cast::<pm_shareable_constant_node_t>(), marker: PhantomData },
            PM_SINGLETON_CLASS_NODE => Self::SingletonClassNode { parser, pointer: node.cast::<pm_singleton_class_node_t>(), marker: PhantomData },
            PM_SOURCE_ENCODING_NODE => Self::SourceEncodingNode { parser, pointer: node.cast::<pm_source_encoding_node_t>(), marker: PhantomData },
            PM_SOURCE_FILE_NODE => Self::SourceFileNode { parser, pointer: node.cast::<pm_source_file_node_t>(), marker: PhantomData },
            PM_SOURCE_LINE_NODE => Self::SourceLineNode { parser, pointer: node.cast::<pm_source_line_node_t>(), marker: PhantomData },
            PM_SPLAT_NODE => Self::SplatNode { parser, pointer: node.cast::<pm_splat_node_t>(), marker: PhantomData },
            PM_STATEMENTS_NODE => Self::StatementsNode { parser, pointer: node.cast::<pm_statements_node_t>(), marker: PhantomData },
            PM_STRING_NODE => Self::StringNode { parser, pointer: node.cast::<pm_string_node_t>(), marker: PhantomData },
            PM_SUPER_NODE => Self::SuperNode { parser, pointer: node.cast::<pm_super_node_t>(), marker: PhantomData },
            PM_SYMBOL_NODE => Self::SymbolNode { parser, pointer: node.cast::<pm_symbol_node_t>(), marker: PhantomData },
            PM_TRUE_NODE => Self::TrueNode { parser, pointer: node.cast::<pm_true_node_t>(), marker: PhantomData },
            PM_UNDEF_NODE => Self::UndefNode { parser, pointer: node.cast::<pm_undef_node_t>(), marker: PhantomData },
            PM_UNLESS_NODE => Self::UnlessNode { parser, pointer: node.cast::<pm_unless_node_t>(), marker: PhantomData },
            PM_UNTIL_NODE => Self::UntilNode { parser, pointer: node.cast::<pm_until_node_t>(), marker: PhantomData },
            PM_WHEN_NODE => Self::WhenNode { parser, pointer: node.cast::<pm_when_node_t>(), marker: PhantomData },
            PM_WHILE_NODE => Self::WhileNode { parser, pointer: node.cast::<pm_while_node_t>(), marker: PhantomData },
            PM_X_STRING_NODE => Self::XStringNode { parser, pointer: node.cast::<pm_x_string_node_t>(), marker: PhantomData },
            PM_YIELD_NODE => Self::YieldNode { parser, pointer: node.cast::<pm_yield_node_t>(), marker: PhantomData },
            _ => panic!("Unknown node type: {}", unsafe { (*node).type_ })
        }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        match *self {
            Self::AliasGlobalVariableNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::AliasMethodNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::AlternationPatternNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::AndNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ArgumentsNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ArrayNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ArrayPatternNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::AssocNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::AssocSplatNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BackReferenceReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BeginNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BlockArgumentNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BlockLocalVariableNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BlockNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BlockParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BlockParametersNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::BreakNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CallAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CallNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CallOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CallOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CallTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CapturePatternNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CaseMatchNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::CaseNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassVariableAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassVariableOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassVariableOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassVariableReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassVariableTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ClassVariableWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantPathAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantPathNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantPathOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantPathOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantPathTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantPathWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ConstantWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::DefNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::DefinedNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ElseNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::EmbeddedStatementsNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::EmbeddedVariableNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::EnsureNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::FalseNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::FindPatternNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::FlipFlopNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::FloatNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ForNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ForwardingArgumentsNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ForwardingParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ForwardingSuperNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::GlobalVariableAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::GlobalVariableOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::GlobalVariableOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::GlobalVariableReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::GlobalVariableTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::GlobalVariableWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::HashNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::HashPatternNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::IfNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ImaginaryNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ImplicitNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ImplicitRestNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::IndexAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::IndexOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::IndexOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::IndexTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InstanceVariableAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InstanceVariableOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InstanceVariableOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InstanceVariableReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InstanceVariableTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InstanceVariableWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::IntegerNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InterpolatedMatchLastLineNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InterpolatedRegularExpressionNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InterpolatedStringNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InterpolatedSymbolNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::InterpolatedXStringNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ItLocalVariableReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ItParametersNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::KeywordHashNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::KeywordRestParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LambdaNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LocalVariableAndWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LocalVariableOperatorWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LocalVariableOrWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LocalVariableReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LocalVariableTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::LocalVariableWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MatchLastLineNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MatchPredicateNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MatchRequiredNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MatchWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MissingNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ModuleNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MultiTargetNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::MultiWriteNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::NextNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::NilNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::NoKeywordsParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::NumberedParametersNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::NumberedReferenceReadNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::OptionalKeywordParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::OptionalParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::OrNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ParametersNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ParenthesesNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::PinnedExpressionNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::PinnedVariableNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::PostExecutionNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::PreExecutionNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ProgramNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RangeNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RationalNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RedoNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RegularExpressionNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RequiredKeywordParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RequiredParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RescueModifierNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RescueNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RestParameterNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::RetryNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ReturnNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SelfNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::ShareableConstantNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SingletonClassNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SourceEncodingNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SourceFileNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SourceLineNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SplatNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::StatementsNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::StringNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SuperNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::SymbolNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::TrueNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::UndefNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::UnlessNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::UntilNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::WhenNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::WhileNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::XStringNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
            Self::YieldNode { pointer, parser, .. } => Location::new(parser, unsafe { &((*pointer.cast::<pm_node_t>()).location) }),
        }
    }

    /// Returns the node as a `AliasGlobalVariableNode`.
    #[must_use]
    pub const fn as_alias_global_variable_node(&self) -> Option<AliasGlobalVariableNode<'pr>> {
        match *self {
            Self::AliasGlobalVariableNode { parser, pointer, marker } => Some(AliasGlobalVariableNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `AliasMethodNode`.
    #[must_use]
    pub const fn as_alias_method_node(&self) -> Option<AliasMethodNode<'pr>> {
        match *self {
            Self::AliasMethodNode { parser, pointer, marker } => Some(AliasMethodNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `AlternationPatternNode`.
    #[must_use]
    pub const fn as_alternation_pattern_node(&self) -> Option<AlternationPatternNode<'pr>> {
        match *self {
            Self::AlternationPatternNode { parser, pointer, marker } => Some(AlternationPatternNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `AndNode`.
    #[must_use]
    pub const fn as_and_node(&self) -> Option<AndNode<'pr>> {
        match *self {
            Self::AndNode { parser, pointer, marker } => Some(AndNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ArgumentsNode`.
    #[must_use]
    pub const fn as_arguments_node(&self) -> Option<ArgumentsNode<'pr>> {
        match *self {
            Self::ArgumentsNode { parser, pointer, marker } => Some(ArgumentsNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ArrayNode`.
    #[must_use]
    pub const fn as_array_node(&self) -> Option<ArrayNode<'pr>> {
        match *self {
            Self::ArrayNode { parser, pointer, marker } => Some(ArrayNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ArrayPatternNode`.
    #[must_use]
    pub const fn as_array_pattern_node(&self) -> Option<ArrayPatternNode<'pr>> {
        match *self {
            Self::ArrayPatternNode { parser, pointer, marker } => Some(ArrayPatternNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `AssocNode`.
    #[must_use]
    pub const fn as_assoc_node(&self) -> Option<AssocNode<'pr>> {
        match *self {
            Self::AssocNode { parser, pointer, marker } => Some(AssocNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `AssocSplatNode`.
    #[must_use]
    pub const fn as_assoc_splat_node(&self) -> Option<AssocSplatNode<'pr>> {
        match *self {
            Self::AssocSplatNode { parser, pointer, marker } => Some(AssocSplatNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BackReferenceReadNode`.
    #[must_use]
    pub const fn as_back_reference_read_node(&self) -> Option<BackReferenceReadNode<'pr>> {
        match *self {
            Self::BackReferenceReadNode { parser, pointer, marker } => Some(BackReferenceReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BeginNode`.
    #[must_use]
    pub const fn as_begin_node(&self) -> Option<BeginNode<'pr>> {
        match *self {
            Self::BeginNode { parser, pointer, marker } => Some(BeginNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BlockArgumentNode`.
    #[must_use]
    pub const fn as_block_argument_node(&self) -> Option<BlockArgumentNode<'pr>> {
        match *self {
            Self::BlockArgumentNode { parser, pointer, marker } => Some(BlockArgumentNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BlockLocalVariableNode`.
    #[must_use]
    pub const fn as_block_local_variable_node(&self) -> Option<BlockLocalVariableNode<'pr>> {
        match *self {
            Self::BlockLocalVariableNode { parser, pointer, marker } => Some(BlockLocalVariableNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BlockNode`.
    #[must_use]
    pub const fn as_block_node(&self) -> Option<BlockNode<'pr>> {
        match *self {
            Self::BlockNode { parser, pointer, marker } => Some(BlockNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BlockParameterNode`.
    #[must_use]
    pub const fn as_block_parameter_node(&self) -> Option<BlockParameterNode<'pr>> {
        match *self {
            Self::BlockParameterNode { parser, pointer, marker } => Some(BlockParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BlockParametersNode`.
    #[must_use]
    pub const fn as_block_parameters_node(&self) -> Option<BlockParametersNode<'pr>> {
        match *self {
            Self::BlockParametersNode { parser, pointer, marker } => Some(BlockParametersNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `BreakNode`.
    #[must_use]
    pub const fn as_break_node(&self) -> Option<BreakNode<'pr>> {
        match *self {
            Self::BreakNode { parser, pointer, marker } => Some(BreakNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CallAndWriteNode`.
    #[must_use]
    pub const fn as_call_and_write_node(&self) -> Option<CallAndWriteNode<'pr>> {
        match *self {
            Self::CallAndWriteNode { parser, pointer, marker } => Some(CallAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CallNode`.
    #[must_use]
    pub const fn as_call_node(&self) -> Option<CallNode<'pr>> {
        match *self {
            Self::CallNode { parser, pointer, marker } => Some(CallNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CallOperatorWriteNode`.
    #[must_use]
    pub const fn as_call_operator_write_node(&self) -> Option<CallOperatorWriteNode<'pr>> {
        match *self {
            Self::CallOperatorWriteNode { parser, pointer, marker } => Some(CallOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CallOrWriteNode`.
    #[must_use]
    pub const fn as_call_or_write_node(&self) -> Option<CallOrWriteNode<'pr>> {
        match *self {
            Self::CallOrWriteNode { parser, pointer, marker } => Some(CallOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CallTargetNode`.
    #[must_use]
    pub const fn as_call_target_node(&self) -> Option<CallTargetNode<'pr>> {
        match *self {
            Self::CallTargetNode { parser, pointer, marker } => Some(CallTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CapturePatternNode`.
    #[must_use]
    pub const fn as_capture_pattern_node(&self) -> Option<CapturePatternNode<'pr>> {
        match *self {
            Self::CapturePatternNode { parser, pointer, marker } => Some(CapturePatternNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CaseMatchNode`.
    #[must_use]
    pub const fn as_case_match_node(&self) -> Option<CaseMatchNode<'pr>> {
        match *self {
            Self::CaseMatchNode { parser, pointer, marker } => Some(CaseMatchNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `CaseNode`.
    #[must_use]
    pub const fn as_case_node(&self) -> Option<CaseNode<'pr>> {
        match *self {
            Self::CaseNode { parser, pointer, marker } => Some(CaseNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassNode`.
    #[must_use]
    pub const fn as_class_node(&self) -> Option<ClassNode<'pr>> {
        match *self {
            Self::ClassNode { parser, pointer, marker } => Some(ClassNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassVariableAndWriteNode`.
    #[must_use]
    pub const fn as_class_variable_and_write_node(&self) -> Option<ClassVariableAndWriteNode<'pr>> {
        match *self {
            Self::ClassVariableAndWriteNode { parser, pointer, marker } => Some(ClassVariableAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassVariableOperatorWriteNode`.
    #[must_use]
    pub const fn as_class_variable_operator_write_node(&self) -> Option<ClassVariableOperatorWriteNode<'pr>> {
        match *self {
            Self::ClassVariableOperatorWriteNode { parser, pointer, marker } => Some(ClassVariableOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassVariableOrWriteNode`.
    #[must_use]
    pub const fn as_class_variable_or_write_node(&self) -> Option<ClassVariableOrWriteNode<'pr>> {
        match *self {
            Self::ClassVariableOrWriteNode { parser, pointer, marker } => Some(ClassVariableOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassVariableReadNode`.
    #[must_use]
    pub const fn as_class_variable_read_node(&self) -> Option<ClassVariableReadNode<'pr>> {
        match *self {
            Self::ClassVariableReadNode { parser, pointer, marker } => Some(ClassVariableReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassVariableTargetNode`.
    #[must_use]
    pub const fn as_class_variable_target_node(&self) -> Option<ClassVariableTargetNode<'pr>> {
        match *self {
            Self::ClassVariableTargetNode { parser, pointer, marker } => Some(ClassVariableTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ClassVariableWriteNode`.
    #[must_use]
    pub const fn as_class_variable_write_node(&self) -> Option<ClassVariableWriteNode<'pr>> {
        match *self {
            Self::ClassVariableWriteNode { parser, pointer, marker } => Some(ClassVariableWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantAndWriteNode`.
    #[must_use]
    pub const fn as_constant_and_write_node(&self) -> Option<ConstantAndWriteNode<'pr>> {
        match *self {
            Self::ConstantAndWriteNode { parser, pointer, marker } => Some(ConstantAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantOperatorWriteNode`.
    #[must_use]
    pub const fn as_constant_operator_write_node(&self) -> Option<ConstantOperatorWriteNode<'pr>> {
        match *self {
            Self::ConstantOperatorWriteNode { parser, pointer, marker } => Some(ConstantOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantOrWriteNode`.
    #[must_use]
    pub const fn as_constant_or_write_node(&self) -> Option<ConstantOrWriteNode<'pr>> {
        match *self {
            Self::ConstantOrWriteNode { parser, pointer, marker } => Some(ConstantOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantPathAndWriteNode`.
    #[must_use]
    pub const fn as_constant_path_and_write_node(&self) -> Option<ConstantPathAndWriteNode<'pr>> {
        match *self {
            Self::ConstantPathAndWriteNode { parser, pointer, marker } => Some(ConstantPathAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantPathNode`.
    #[must_use]
    pub const fn as_constant_path_node(&self) -> Option<ConstantPathNode<'pr>> {
        match *self {
            Self::ConstantPathNode { parser, pointer, marker } => Some(ConstantPathNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantPathOperatorWriteNode`.
    #[must_use]
    pub const fn as_constant_path_operator_write_node(&self) -> Option<ConstantPathOperatorWriteNode<'pr>> {
        match *self {
            Self::ConstantPathOperatorWriteNode { parser, pointer, marker } => Some(ConstantPathOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantPathOrWriteNode`.
    #[must_use]
    pub const fn as_constant_path_or_write_node(&self) -> Option<ConstantPathOrWriteNode<'pr>> {
        match *self {
            Self::ConstantPathOrWriteNode { parser, pointer, marker } => Some(ConstantPathOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantPathTargetNode`.
    #[must_use]
    pub const fn as_constant_path_target_node(&self) -> Option<ConstantPathTargetNode<'pr>> {
        match *self {
            Self::ConstantPathTargetNode { parser, pointer, marker } => Some(ConstantPathTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantPathWriteNode`.
    #[must_use]
    pub const fn as_constant_path_write_node(&self) -> Option<ConstantPathWriteNode<'pr>> {
        match *self {
            Self::ConstantPathWriteNode { parser, pointer, marker } => Some(ConstantPathWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantReadNode`.
    #[must_use]
    pub const fn as_constant_read_node(&self) -> Option<ConstantReadNode<'pr>> {
        match *self {
            Self::ConstantReadNode { parser, pointer, marker } => Some(ConstantReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantTargetNode`.
    #[must_use]
    pub const fn as_constant_target_node(&self) -> Option<ConstantTargetNode<'pr>> {
        match *self {
            Self::ConstantTargetNode { parser, pointer, marker } => Some(ConstantTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ConstantWriteNode`.
    #[must_use]
    pub const fn as_constant_write_node(&self) -> Option<ConstantWriteNode<'pr>> {
        match *self {
            Self::ConstantWriteNode { parser, pointer, marker } => Some(ConstantWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `DefNode`.
    #[must_use]
    pub const fn as_def_node(&self) -> Option<DefNode<'pr>> {
        match *self {
            Self::DefNode { parser, pointer, marker } => Some(DefNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `DefinedNode`.
    #[must_use]
    pub const fn as_defined_node(&self) -> Option<DefinedNode<'pr>> {
        match *self {
            Self::DefinedNode { parser, pointer, marker } => Some(DefinedNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ElseNode`.
    #[must_use]
    pub const fn as_else_node(&self) -> Option<ElseNode<'pr>> {
        match *self {
            Self::ElseNode { parser, pointer, marker } => Some(ElseNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `EmbeddedStatementsNode`.
    #[must_use]
    pub const fn as_embedded_statements_node(&self) -> Option<EmbeddedStatementsNode<'pr>> {
        match *self {
            Self::EmbeddedStatementsNode { parser, pointer, marker } => Some(EmbeddedStatementsNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `EmbeddedVariableNode`.
    #[must_use]
    pub const fn as_embedded_variable_node(&self) -> Option<EmbeddedVariableNode<'pr>> {
        match *self {
            Self::EmbeddedVariableNode { parser, pointer, marker } => Some(EmbeddedVariableNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `EnsureNode`.
    #[must_use]
    pub const fn as_ensure_node(&self) -> Option<EnsureNode<'pr>> {
        match *self {
            Self::EnsureNode { parser, pointer, marker } => Some(EnsureNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `FalseNode`.
    #[must_use]
    pub const fn as_false_node(&self) -> Option<FalseNode<'pr>> {
        match *self {
            Self::FalseNode { parser, pointer, marker } => Some(FalseNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `FindPatternNode`.
    #[must_use]
    pub const fn as_find_pattern_node(&self) -> Option<FindPatternNode<'pr>> {
        match *self {
            Self::FindPatternNode { parser, pointer, marker } => Some(FindPatternNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `FlipFlopNode`.
    #[must_use]
    pub const fn as_flip_flop_node(&self) -> Option<FlipFlopNode<'pr>> {
        match *self {
            Self::FlipFlopNode { parser, pointer, marker } => Some(FlipFlopNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `FloatNode`.
    #[must_use]
    pub const fn as_float_node(&self) -> Option<FloatNode<'pr>> {
        match *self {
            Self::FloatNode { parser, pointer, marker } => Some(FloatNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ForNode`.
    #[must_use]
    pub const fn as_for_node(&self) -> Option<ForNode<'pr>> {
        match *self {
            Self::ForNode { parser, pointer, marker } => Some(ForNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ForwardingArgumentsNode`.
    #[must_use]
    pub const fn as_forwarding_arguments_node(&self) -> Option<ForwardingArgumentsNode<'pr>> {
        match *self {
            Self::ForwardingArgumentsNode { parser, pointer, marker } => Some(ForwardingArgumentsNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ForwardingParameterNode`.
    #[must_use]
    pub const fn as_forwarding_parameter_node(&self) -> Option<ForwardingParameterNode<'pr>> {
        match *self {
            Self::ForwardingParameterNode { parser, pointer, marker } => Some(ForwardingParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ForwardingSuperNode`.
    #[must_use]
    pub const fn as_forwarding_super_node(&self) -> Option<ForwardingSuperNode<'pr>> {
        match *self {
            Self::ForwardingSuperNode { parser, pointer, marker } => Some(ForwardingSuperNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `GlobalVariableAndWriteNode`.
    #[must_use]
    pub const fn as_global_variable_and_write_node(&self) -> Option<GlobalVariableAndWriteNode<'pr>> {
        match *self {
            Self::GlobalVariableAndWriteNode { parser, pointer, marker } => Some(GlobalVariableAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `GlobalVariableOperatorWriteNode`.
    #[must_use]
    pub const fn as_global_variable_operator_write_node(&self) -> Option<GlobalVariableOperatorWriteNode<'pr>> {
        match *self {
            Self::GlobalVariableOperatorWriteNode { parser, pointer, marker } => Some(GlobalVariableOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `GlobalVariableOrWriteNode`.
    #[must_use]
    pub const fn as_global_variable_or_write_node(&self) -> Option<GlobalVariableOrWriteNode<'pr>> {
        match *self {
            Self::GlobalVariableOrWriteNode { parser, pointer, marker } => Some(GlobalVariableOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `GlobalVariableReadNode`.
    #[must_use]
    pub const fn as_global_variable_read_node(&self) -> Option<GlobalVariableReadNode<'pr>> {
        match *self {
            Self::GlobalVariableReadNode { parser, pointer, marker } => Some(GlobalVariableReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `GlobalVariableTargetNode`.
    #[must_use]
    pub const fn as_global_variable_target_node(&self) -> Option<GlobalVariableTargetNode<'pr>> {
        match *self {
            Self::GlobalVariableTargetNode { parser, pointer, marker } => Some(GlobalVariableTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `GlobalVariableWriteNode`.
    #[must_use]
    pub const fn as_global_variable_write_node(&self) -> Option<GlobalVariableWriteNode<'pr>> {
        match *self {
            Self::GlobalVariableWriteNode { parser, pointer, marker } => Some(GlobalVariableWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `HashNode`.
    #[must_use]
    pub const fn as_hash_node(&self) -> Option<HashNode<'pr>> {
        match *self {
            Self::HashNode { parser, pointer, marker } => Some(HashNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `HashPatternNode`.
    #[must_use]
    pub const fn as_hash_pattern_node(&self) -> Option<HashPatternNode<'pr>> {
        match *self {
            Self::HashPatternNode { parser, pointer, marker } => Some(HashPatternNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `IfNode`.
    #[must_use]
    pub const fn as_if_node(&self) -> Option<IfNode<'pr>> {
        match *self {
            Self::IfNode { parser, pointer, marker } => Some(IfNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ImaginaryNode`.
    #[must_use]
    pub const fn as_imaginary_node(&self) -> Option<ImaginaryNode<'pr>> {
        match *self {
            Self::ImaginaryNode { parser, pointer, marker } => Some(ImaginaryNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ImplicitNode`.
    #[must_use]
    pub const fn as_implicit_node(&self) -> Option<ImplicitNode<'pr>> {
        match *self {
            Self::ImplicitNode { parser, pointer, marker } => Some(ImplicitNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ImplicitRestNode`.
    #[must_use]
    pub const fn as_implicit_rest_node(&self) -> Option<ImplicitRestNode<'pr>> {
        match *self {
            Self::ImplicitRestNode { parser, pointer, marker } => Some(ImplicitRestNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InNode`.
    #[must_use]
    pub const fn as_in_node(&self) -> Option<InNode<'pr>> {
        match *self {
            Self::InNode { parser, pointer, marker } => Some(InNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `IndexAndWriteNode`.
    #[must_use]
    pub const fn as_index_and_write_node(&self) -> Option<IndexAndWriteNode<'pr>> {
        match *self {
            Self::IndexAndWriteNode { parser, pointer, marker } => Some(IndexAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `IndexOperatorWriteNode`.
    #[must_use]
    pub const fn as_index_operator_write_node(&self) -> Option<IndexOperatorWriteNode<'pr>> {
        match *self {
            Self::IndexOperatorWriteNode { parser, pointer, marker } => Some(IndexOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `IndexOrWriteNode`.
    #[must_use]
    pub const fn as_index_or_write_node(&self) -> Option<IndexOrWriteNode<'pr>> {
        match *self {
            Self::IndexOrWriteNode { parser, pointer, marker } => Some(IndexOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `IndexTargetNode`.
    #[must_use]
    pub const fn as_index_target_node(&self) -> Option<IndexTargetNode<'pr>> {
        match *self {
            Self::IndexTargetNode { parser, pointer, marker } => Some(IndexTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InstanceVariableAndWriteNode`.
    #[must_use]
    pub const fn as_instance_variable_and_write_node(&self) -> Option<InstanceVariableAndWriteNode<'pr>> {
        match *self {
            Self::InstanceVariableAndWriteNode { parser, pointer, marker } => Some(InstanceVariableAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InstanceVariableOperatorWriteNode`.
    #[must_use]
    pub const fn as_instance_variable_operator_write_node(&self) -> Option<InstanceVariableOperatorWriteNode<'pr>> {
        match *self {
            Self::InstanceVariableOperatorWriteNode { parser, pointer, marker } => Some(InstanceVariableOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InstanceVariableOrWriteNode`.
    #[must_use]
    pub const fn as_instance_variable_or_write_node(&self) -> Option<InstanceVariableOrWriteNode<'pr>> {
        match *self {
            Self::InstanceVariableOrWriteNode { parser, pointer, marker } => Some(InstanceVariableOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InstanceVariableReadNode`.
    #[must_use]
    pub const fn as_instance_variable_read_node(&self) -> Option<InstanceVariableReadNode<'pr>> {
        match *self {
            Self::InstanceVariableReadNode { parser, pointer, marker } => Some(InstanceVariableReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InstanceVariableTargetNode`.
    #[must_use]
    pub const fn as_instance_variable_target_node(&self) -> Option<InstanceVariableTargetNode<'pr>> {
        match *self {
            Self::InstanceVariableTargetNode { parser, pointer, marker } => Some(InstanceVariableTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InstanceVariableWriteNode`.
    #[must_use]
    pub const fn as_instance_variable_write_node(&self) -> Option<InstanceVariableWriteNode<'pr>> {
        match *self {
            Self::InstanceVariableWriteNode { parser, pointer, marker } => Some(InstanceVariableWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `IntegerNode`.
    #[must_use]
    pub const fn as_integer_node(&self) -> Option<IntegerNode<'pr>> {
        match *self {
            Self::IntegerNode { parser, pointer, marker } => Some(IntegerNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InterpolatedMatchLastLineNode`.
    #[must_use]
    pub const fn as_interpolated_match_last_line_node(&self) -> Option<InterpolatedMatchLastLineNode<'pr>> {
        match *self {
            Self::InterpolatedMatchLastLineNode { parser, pointer, marker } => Some(InterpolatedMatchLastLineNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InterpolatedRegularExpressionNode`.
    #[must_use]
    pub const fn as_interpolated_regular_expression_node(&self) -> Option<InterpolatedRegularExpressionNode<'pr>> {
        match *self {
            Self::InterpolatedRegularExpressionNode { parser, pointer, marker } => Some(InterpolatedRegularExpressionNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InterpolatedStringNode`.
    #[must_use]
    pub const fn as_interpolated_string_node(&self) -> Option<InterpolatedStringNode<'pr>> {
        match *self {
            Self::InterpolatedStringNode { parser, pointer, marker } => Some(InterpolatedStringNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InterpolatedSymbolNode`.
    #[must_use]
    pub const fn as_interpolated_symbol_node(&self) -> Option<InterpolatedSymbolNode<'pr>> {
        match *self {
            Self::InterpolatedSymbolNode { parser, pointer, marker } => Some(InterpolatedSymbolNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `InterpolatedXStringNode`.
    #[must_use]
    pub const fn as_interpolated_x_string_node(&self) -> Option<InterpolatedXStringNode<'pr>> {
        match *self {
            Self::InterpolatedXStringNode { parser, pointer, marker } => Some(InterpolatedXStringNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ItLocalVariableReadNode`.
    #[must_use]
    pub const fn as_it_local_variable_read_node(&self) -> Option<ItLocalVariableReadNode<'pr>> {
        match *self {
            Self::ItLocalVariableReadNode { parser, pointer, marker } => Some(ItLocalVariableReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ItParametersNode`.
    #[must_use]
    pub const fn as_it_parameters_node(&self) -> Option<ItParametersNode<'pr>> {
        match *self {
            Self::ItParametersNode { parser, pointer, marker } => Some(ItParametersNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `KeywordHashNode`.
    #[must_use]
    pub const fn as_keyword_hash_node(&self) -> Option<KeywordHashNode<'pr>> {
        match *self {
            Self::KeywordHashNode { parser, pointer, marker } => Some(KeywordHashNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `KeywordRestParameterNode`.
    #[must_use]
    pub const fn as_keyword_rest_parameter_node(&self) -> Option<KeywordRestParameterNode<'pr>> {
        match *self {
            Self::KeywordRestParameterNode { parser, pointer, marker } => Some(KeywordRestParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LambdaNode`.
    #[must_use]
    pub const fn as_lambda_node(&self) -> Option<LambdaNode<'pr>> {
        match *self {
            Self::LambdaNode { parser, pointer, marker } => Some(LambdaNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LocalVariableAndWriteNode`.
    #[must_use]
    pub const fn as_local_variable_and_write_node(&self) -> Option<LocalVariableAndWriteNode<'pr>> {
        match *self {
            Self::LocalVariableAndWriteNode { parser, pointer, marker } => Some(LocalVariableAndWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LocalVariableOperatorWriteNode`.
    #[must_use]
    pub const fn as_local_variable_operator_write_node(&self) -> Option<LocalVariableOperatorWriteNode<'pr>> {
        match *self {
            Self::LocalVariableOperatorWriteNode { parser, pointer, marker } => Some(LocalVariableOperatorWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LocalVariableOrWriteNode`.
    #[must_use]
    pub const fn as_local_variable_or_write_node(&self) -> Option<LocalVariableOrWriteNode<'pr>> {
        match *self {
            Self::LocalVariableOrWriteNode { parser, pointer, marker } => Some(LocalVariableOrWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LocalVariableReadNode`.
    #[must_use]
    pub const fn as_local_variable_read_node(&self) -> Option<LocalVariableReadNode<'pr>> {
        match *self {
            Self::LocalVariableReadNode { parser, pointer, marker } => Some(LocalVariableReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LocalVariableTargetNode`.
    #[must_use]
    pub const fn as_local_variable_target_node(&self) -> Option<LocalVariableTargetNode<'pr>> {
        match *self {
            Self::LocalVariableTargetNode { parser, pointer, marker } => Some(LocalVariableTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `LocalVariableWriteNode`.
    #[must_use]
    pub const fn as_local_variable_write_node(&self) -> Option<LocalVariableWriteNode<'pr>> {
        match *self {
            Self::LocalVariableWriteNode { parser, pointer, marker } => Some(LocalVariableWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MatchLastLineNode`.
    #[must_use]
    pub const fn as_match_last_line_node(&self) -> Option<MatchLastLineNode<'pr>> {
        match *self {
            Self::MatchLastLineNode { parser, pointer, marker } => Some(MatchLastLineNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MatchPredicateNode`.
    #[must_use]
    pub const fn as_match_predicate_node(&self) -> Option<MatchPredicateNode<'pr>> {
        match *self {
            Self::MatchPredicateNode { parser, pointer, marker } => Some(MatchPredicateNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MatchRequiredNode`.
    #[must_use]
    pub const fn as_match_required_node(&self) -> Option<MatchRequiredNode<'pr>> {
        match *self {
            Self::MatchRequiredNode { parser, pointer, marker } => Some(MatchRequiredNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MatchWriteNode`.
    #[must_use]
    pub const fn as_match_write_node(&self) -> Option<MatchWriteNode<'pr>> {
        match *self {
            Self::MatchWriteNode { parser, pointer, marker } => Some(MatchWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MissingNode`.
    #[must_use]
    pub const fn as_missing_node(&self) -> Option<MissingNode<'pr>> {
        match *self {
            Self::MissingNode { parser, pointer, marker } => Some(MissingNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ModuleNode`.
    #[must_use]
    pub const fn as_module_node(&self) -> Option<ModuleNode<'pr>> {
        match *self {
            Self::ModuleNode { parser, pointer, marker } => Some(ModuleNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MultiTargetNode`.
    #[must_use]
    pub const fn as_multi_target_node(&self) -> Option<MultiTargetNode<'pr>> {
        match *self {
            Self::MultiTargetNode { parser, pointer, marker } => Some(MultiTargetNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `MultiWriteNode`.
    #[must_use]
    pub const fn as_multi_write_node(&self) -> Option<MultiWriteNode<'pr>> {
        match *self {
            Self::MultiWriteNode { parser, pointer, marker } => Some(MultiWriteNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `NextNode`.
    #[must_use]
    pub const fn as_next_node(&self) -> Option<NextNode<'pr>> {
        match *self {
            Self::NextNode { parser, pointer, marker } => Some(NextNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `NilNode`.
    #[must_use]
    pub const fn as_nil_node(&self) -> Option<NilNode<'pr>> {
        match *self {
            Self::NilNode { parser, pointer, marker } => Some(NilNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `NoKeywordsParameterNode`.
    #[must_use]
    pub const fn as_no_keywords_parameter_node(&self) -> Option<NoKeywordsParameterNode<'pr>> {
        match *self {
            Self::NoKeywordsParameterNode { parser, pointer, marker } => Some(NoKeywordsParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `NumberedParametersNode`.
    #[must_use]
    pub const fn as_numbered_parameters_node(&self) -> Option<NumberedParametersNode<'pr>> {
        match *self {
            Self::NumberedParametersNode { parser, pointer, marker } => Some(NumberedParametersNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `NumberedReferenceReadNode`.
    #[must_use]
    pub const fn as_numbered_reference_read_node(&self) -> Option<NumberedReferenceReadNode<'pr>> {
        match *self {
            Self::NumberedReferenceReadNode { parser, pointer, marker } => Some(NumberedReferenceReadNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `OptionalKeywordParameterNode`.
    #[must_use]
    pub const fn as_optional_keyword_parameter_node(&self) -> Option<OptionalKeywordParameterNode<'pr>> {
        match *self {
            Self::OptionalKeywordParameterNode { parser, pointer, marker } => Some(OptionalKeywordParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `OptionalParameterNode`.
    #[must_use]
    pub const fn as_optional_parameter_node(&self) -> Option<OptionalParameterNode<'pr>> {
        match *self {
            Self::OptionalParameterNode { parser, pointer, marker } => Some(OptionalParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `OrNode`.
    #[must_use]
    pub const fn as_or_node(&self) -> Option<OrNode<'pr>> {
        match *self {
            Self::OrNode { parser, pointer, marker } => Some(OrNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ParametersNode`.
    #[must_use]
    pub const fn as_parameters_node(&self) -> Option<ParametersNode<'pr>> {
        match *self {
            Self::ParametersNode { parser, pointer, marker } => Some(ParametersNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ParenthesesNode`.
    #[must_use]
    pub const fn as_parentheses_node(&self) -> Option<ParenthesesNode<'pr>> {
        match *self {
            Self::ParenthesesNode { parser, pointer, marker } => Some(ParenthesesNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `PinnedExpressionNode`.
    #[must_use]
    pub const fn as_pinned_expression_node(&self) -> Option<PinnedExpressionNode<'pr>> {
        match *self {
            Self::PinnedExpressionNode { parser, pointer, marker } => Some(PinnedExpressionNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `PinnedVariableNode`.
    #[must_use]
    pub const fn as_pinned_variable_node(&self) -> Option<PinnedVariableNode<'pr>> {
        match *self {
            Self::PinnedVariableNode { parser, pointer, marker } => Some(PinnedVariableNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `PostExecutionNode`.
    #[must_use]
    pub const fn as_post_execution_node(&self) -> Option<PostExecutionNode<'pr>> {
        match *self {
            Self::PostExecutionNode { parser, pointer, marker } => Some(PostExecutionNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `PreExecutionNode`.
    #[must_use]
    pub const fn as_pre_execution_node(&self) -> Option<PreExecutionNode<'pr>> {
        match *self {
            Self::PreExecutionNode { parser, pointer, marker } => Some(PreExecutionNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ProgramNode`.
    #[must_use]
    pub const fn as_program_node(&self) -> Option<ProgramNode<'pr>> {
        match *self {
            Self::ProgramNode { parser, pointer, marker } => Some(ProgramNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RangeNode`.
    #[must_use]
    pub const fn as_range_node(&self) -> Option<RangeNode<'pr>> {
        match *self {
            Self::RangeNode { parser, pointer, marker } => Some(RangeNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RationalNode`.
    #[must_use]
    pub const fn as_rational_node(&self) -> Option<RationalNode<'pr>> {
        match *self {
            Self::RationalNode { parser, pointer, marker } => Some(RationalNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RedoNode`.
    #[must_use]
    pub const fn as_redo_node(&self) -> Option<RedoNode<'pr>> {
        match *self {
            Self::RedoNode { parser, pointer, marker } => Some(RedoNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RegularExpressionNode`.
    #[must_use]
    pub const fn as_regular_expression_node(&self) -> Option<RegularExpressionNode<'pr>> {
        match *self {
            Self::RegularExpressionNode { parser, pointer, marker } => Some(RegularExpressionNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RequiredKeywordParameterNode`.
    #[must_use]
    pub const fn as_required_keyword_parameter_node(&self) -> Option<RequiredKeywordParameterNode<'pr>> {
        match *self {
            Self::RequiredKeywordParameterNode { parser, pointer, marker } => Some(RequiredKeywordParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RequiredParameterNode`.
    #[must_use]
    pub const fn as_required_parameter_node(&self) -> Option<RequiredParameterNode<'pr>> {
        match *self {
            Self::RequiredParameterNode { parser, pointer, marker } => Some(RequiredParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RescueModifierNode`.
    #[must_use]
    pub const fn as_rescue_modifier_node(&self) -> Option<RescueModifierNode<'pr>> {
        match *self {
            Self::RescueModifierNode { parser, pointer, marker } => Some(RescueModifierNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RescueNode`.
    #[must_use]
    pub const fn as_rescue_node(&self) -> Option<RescueNode<'pr>> {
        match *self {
            Self::RescueNode { parser, pointer, marker } => Some(RescueNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RestParameterNode`.
    #[must_use]
    pub const fn as_rest_parameter_node(&self) -> Option<RestParameterNode<'pr>> {
        match *self {
            Self::RestParameterNode { parser, pointer, marker } => Some(RestParameterNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `RetryNode`.
    #[must_use]
    pub const fn as_retry_node(&self) -> Option<RetryNode<'pr>> {
        match *self {
            Self::RetryNode { parser, pointer, marker } => Some(RetryNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ReturnNode`.
    #[must_use]
    pub const fn as_return_node(&self) -> Option<ReturnNode<'pr>> {
        match *self {
            Self::ReturnNode { parser, pointer, marker } => Some(ReturnNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SelfNode`.
    #[must_use]
    pub const fn as_self_node(&self) -> Option<SelfNode<'pr>> {
        match *self {
            Self::SelfNode { parser, pointer, marker } => Some(SelfNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `ShareableConstantNode`.
    #[must_use]
    pub const fn as_shareable_constant_node(&self) -> Option<ShareableConstantNode<'pr>> {
        match *self {
            Self::ShareableConstantNode { parser, pointer, marker } => Some(ShareableConstantNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SingletonClassNode`.
    #[must_use]
    pub const fn as_singleton_class_node(&self) -> Option<SingletonClassNode<'pr>> {
        match *self {
            Self::SingletonClassNode { parser, pointer, marker } => Some(SingletonClassNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SourceEncodingNode`.
    #[must_use]
    pub const fn as_source_encoding_node(&self) -> Option<SourceEncodingNode<'pr>> {
        match *self {
            Self::SourceEncodingNode { parser, pointer, marker } => Some(SourceEncodingNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SourceFileNode`.
    #[must_use]
    pub const fn as_source_file_node(&self) -> Option<SourceFileNode<'pr>> {
        match *self {
            Self::SourceFileNode { parser, pointer, marker } => Some(SourceFileNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SourceLineNode`.
    #[must_use]
    pub const fn as_source_line_node(&self) -> Option<SourceLineNode<'pr>> {
        match *self {
            Self::SourceLineNode { parser, pointer, marker } => Some(SourceLineNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SplatNode`.
    #[must_use]
    pub const fn as_splat_node(&self) -> Option<SplatNode<'pr>> {
        match *self {
            Self::SplatNode { parser, pointer, marker } => Some(SplatNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `StatementsNode`.
    #[must_use]
    pub const fn as_statements_node(&self) -> Option<StatementsNode<'pr>> {
        match *self {
            Self::StatementsNode { parser, pointer, marker } => Some(StatementsNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `StringNode`.
    #[must_use]
    pub const fn as_string_node(&self) -> Option<StringNode<'pr>> {
        match *self {
            Self::StringNode { parser, pointer, marker } => Some(StringNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SuperNode`.
    #[must_use]
    pub const fn as_super_node(&self) -> Option<SuperNode<'pr>> {
        match *self {
            Self::SuperNode { parser, pointer, marker } => Some(SuperNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `SymbolNode`.
    #[must_use]
    pub const fn as_symbol_node(&self) -> Option<SymbolNode<'pr>> {
        match *self {
            Self::SymbolNode { parser, pointer, marker } => Some(SymbolNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `TrueNode`.
    #[must_use]
    pub const fn as_true_node(&self) -> Option<TrueNode<'pr>> {
        match *self {
            Self::TrueNode { parser, pointer, marker } => Some(TrueNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `UndefNode`.
    #[must_use]
    pub const fn as_undef_node(&self) -> Option<UndefNode<'pr>> {
        match *self {
            Self::UndefNode { parser, pointer, marker } => Some(UndefNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `UnlessNode`.
    #[must_use]
    pub const fn as_unless_node(&self) -> Option<UnlessNode<'pr>> {
        match *self {
            Self::UnlessNode { parser, pointer, marker } => Some(UnlessNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `UntilNode`.
    #[must_use]
    pub const fn as_until_node(&self) -> Option<UntilNode<'pr>> {
        match *self {
            Self::UntilNode { parser, pointer, marker } => Some(UntilNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `WhenNode`.
    #[must_use]
    pub const fn as_when_node(&self) -> Option<WhenNode<'pr>> {
        match *self {
            Self::WhenNode { parser, pointer, marker } => Some(WhenNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `WhileNode`.
    #[must_use]
    pub const fn as_while_node(&self) -> Option<WhileNode<'pr>> {
        match *self {
            Self::WhileNode { parser, pointer, marker } => Some(WhileNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `XStringNode`.
    #[must_use]
    pub const fn as_x_string_node(&self) -> Option<XStringNode<'pr>> {
        match *self {
            Self::XStringNode { parser, pointer, marker } => Some(XStringNode { parser, pointer, marker }),
            _ => None
        }
    }
    /// Returns the node as a `YieldNode`.
    #[must_use]
    pub const fn as_yield_node(&self) -> Option<YieldNode<'pr>> {
        match *self {
            Self::YieldNode { parser, pointer, marker } => Some(YieldNode { parser, pointer, marker }),
            _ => None
        }
    }
}

impl std::fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AliasGlobalVariableNode { parser, pointer, marker } => write!(f, "{:?}", AliasGlobalVariableNode { parser, pointer, marker }),
            Self::AliasMethodNode { parser, pointer, marker } => write!(f, "{:?}", AliasMethodNode { parser, pointer, marker }),
            Self::AlternationPatternNode { parser, pointer, marker } => write!(f, "{:?}", AlternationPatternNode { parser, pointer, marker }),
            Self::AndNode { parser, pointer, marker } => write!(f, "{:?}", AndNode { parser, pointer, marker }),
            Self::ArgumentsNode { parser, pointer, marker } => write!(f, "{:?}", ArgumentsNode { parser, pointer, marker }),
            Self::ArrayNode { parser, pointer, marker } => write!(f, "{:?}", ArrayNode { parser, pointer, marker }),
            Self::ArrayPatternNode { parser, pointer, marker } => write!(f, "{:?}", ArrayPatternNode { parser, pointer, marker }),
            Self::AssocNode { parser, pointer, marker } => write!(f, "{:?}", AssocNode { parser, pointer, marker }),
            Self::AssocSplatNode { parser, pointer, marker } => write!(f, "{:?}", AssocSplatNode { parser, pointer, marker }),
            Self::BackReferenceReadNode { parser, pointer, marker } => write!(f, "{:?}", BackReferenceReadNode { parser, pointer, marker }),
            Self::BeginNode { parser, pointer, marker } => write!(f, "{:?}", BeginNode { parser, pointer, marker }),
            Self::BlockArgumentNode { parser, pointer, marker } => write!(f, "{:?}", BlockArgumentNode { parser, pointer, marker }),
            Self::BlockLocalVariableNode { parser, pointer, marker } => write!(f, "{:?}", BlockLocalVariableNode { parser, pointer, marker }),
            Self::BlockNode { parser, pointer, marker } => write!(f, "{:?}", BlockNode { parser, pointer, marker }),
            Self::BlockParameterNode { parser, pointer, marker } => write!(f, "{:?}", BlockParameterNode { parser, pointer, marker }),
            Self::BlockParametersNode { parser, pointer, marker } => write!(f, "{:?}", BlockParametersNode { parser, pointer, marker }),
            Self::BreakNode { parser, pointer, marker } => write!(f, "{:?}", BreakNode { parser, pointer, marker }),
            Self::CallAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", CallAndWriteNode { parser, pointer, marker }),
            Self::CallNode { parser, pointer, marker } => write!(f, "{:?}", CallNode { parser, pointer, marker }),
            Self::CallOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", CallOperatorWriteNode { parser, pointer, marker }),
            Self::CallOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", CallOrWriteNode { parser, pointer, marker }),
            Self::CallTargetNode { parser, pointer, marker } => write!(f, "{:?}", CallTargetNode { parser, pointer, marker }),
            Self::CapturePatternNode { parser, pointer, marker } => write!(f, "{:?}", CapturePatternNode { parser, pointer, marker }),
            Self::CaseMatchNode { parser, pointer, marker } => write!(f, "{:?}", CaseMatchNode { parser, pointer, marker }),
            Self::CaseNode { parser, pointer, marker } => write!(f, "{:?}", CaseNode { parser, pointer, marker }),
            Self::ClassNode { parser, pointer, marker } => write!(f, "{:?}", ClassNode { parser, pointer, marker }),
            Self::ClassVariableAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", ClassVariableAndWriteNode { parser, pointer, marker }),
            Self::ClassVariableOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", ClassVariableOperatorWriteNode { parser, pointer, marker }),
            Self::ClassVariableOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", ClassVariableOrWriteNode { parser, pointer, marker }),
            Self::ClassVariableReadNode { parser, pointer, marker } => write!(f, "{:?}", ClassVariableReadNode { parser, pointer, marker }),
            Self::ClassVariableTargetNode { parser, pointer, marker } => write!(f, "{:?}", ClassVariableTargetNode { parser, pointer, marker }),
            Self::ClassVariableWriteNode { parser, pointer, marker } => write!(f, "{:?}", ClassVariableWriteNode { parser, pointer, marker }),
            Self::ConstantAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantAndWriteNode { parser, pointer, marker }),
            Self::ConstantOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantOperatorWriteNode { parser, pointer, marker }),
            Self::ConstantOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantOrWriteNode { parser, pointer, marker }),
            Self::ConstantPathAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantPathAndWriteNode { parser, pointer, marker }),
            Self::ConstantPathNode { parser, pointer, marker } => write!(f, "{:?}", ConstantPathNode { parser, pointer, marker }),
            Self::ConstantPathOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantPathOperatorWriteNode { parser, pointer, marker }),
            Self::ConstantPathOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantPathOrWriteNode { parser, pointer, marker }),
            Self::ConstantPathTargetNode { parser, pointer, marker } => write!(f, "{:?}", ConstantPathTargetNode { parser, pointer, marker }),
            Self::ConstantPathWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantPathWriteNode { parser, pointer, marker }),
            Self::ConstantReadNode { parser, pointer, marker } => write!(f, "{:?}", ConstantReadNode { parser, pointer, marker }),
            Self::ConstantTargetNode { parser, pointer, marker } => write!(f, "{:?}", ConstantTargetNode { parser, pointer, marker }),
            Self::ConstantWriteNode { parser, pointer, marker } => write!(f, "{:?}", ConstantWriteNode { parser, pointer, marker }),
            Self::DefNode { parser, pointer, marker } => write!(f, "{:?}", DefNode { parser, pointer, marker }),
            Self::DefinedNode { parser, pointer, marker } => write!(f, "{:?}", DefinedNode { parser, pointer, marker }),
            Self::ElseNode { parser, pointer, marker } => write!(f, "{:?}", ElseNode { parser, pointer, marker }),
            Self::EmbeddedStatementsNode { parser, pointer, marker } => write!(f, "{:?}", EmbeddedStatementsNode { parser, pointer, marker }),
            Self::EmbeddedVariableNode { parser, pointer, marker } => write!(f, "{:?}", EmbeddedVariableNode { parser, pointer, marker }),
            Self::EnsureNode { parser, pointer, marker } => write!(f, "{:?}", EnsureNode { parser, pointer, marker }),
            Self::FalseNode { parser, pointer, marker } => write!(f, "{:?}", FalseNode { parser, pointer, marker }),
            Self::FindPatternNode { parser, pointer, marker } => write!(f, "{:?}", FindPatternNode { parser, pointer, marker }),
            Self::FlipFlopNode { parser, pointer, marker } => write!(f, "{:?}", FlipFlopNode { parser, pointer, marker }),
            Self::FloatNode { parser, pointer, marker } => write!(f, "{:?}", FloatNode { parser, pointer, marker }),
            Self::ForNode { parser, pointer, marker } => write!(f, "{:?}", ForNode { parser, pointer, marker }),
            Self::ForwardingArgumentsNode { parser, pointer, marker } => write!(f, "{:?}", ForwardingArgumentsNode { parser, pointer, marker }),
            Self::ForwardingParameterNode { parser, pointer, marker } => write!(f, "{:?}", ForwardingParameterNode { parser, pointer, marker }),
            Self::ForwardingSuperNode { parser, pointer, marker } => write!(f, "{:?}", ForwardingSuperNode { parser, pointer, marker }),
            Self::GlobalVariableAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", GlobalVariableAndWriteNode { parser, pointer, marker }),
            Self::GlobalVariableOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", GlobalVariableOperatorWriteNode { parser, pointer, marker }),
            Self::GlobalVariableOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", GlobalVariableOrWriteNode { parser, pointer, marker }),
            Self::GlobalVariableReadNode { parser, pointer, marker } => write!(f, "{:?}", GlobalVariableReadNode { parser, pointer, marker }),
            Self::GlobalVariableTargetNode { parser, pointer, marker } => write!(f, "{:?}", GlobalVariableTargetNode { parser, pointer, marker }),
            Self::GlobalVariableWriteNode { parser, pointer, marker } => write!(f, "{:?}", GlobalVariableWriteNode { parser, pointer, marker }),
            Self::HashNode { parser, pointer, marker } => write!(f, "{:?}", HashNode { parser, pointer, marker }),
            Self::HashPatternNode { parser, pointer, marker } => write!(f, "{:?}", HashPatternNode { parser, pointer, marker }),
            Self::IfNode { parser, pointer, marker } => write!(f, "{:?}", IfNode { parser, pointer, marker }),
            Self::ImaginaryNode { parser, pointer, marker } => write!(f, "{:?}", ImaginaryNode { parser, pointer, marker }),
            Self::ImplicitNode { parser, pointer, marker } => write!(f, "{:?}", ImplicitNode { parser, pointer, marker }),
            Self::ImplicitRestNode { parser, pointer, marker } => write!(f, "{:?}", ImplicitRestNode { parser, pointer, marker }),
            Self::InNode { parser, pointer, marker } => write!(f, "{:?}", InNode { parser, pointer, marker }),
            Self::IndexAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", IndexAndWriteNode { parser, pointer, marker }),
            Self::IndexOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", IndexOperatorWriteNode { parser, pointer, marker }),
            Self::IndexOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", IndexOrWriteNode { parser, pointer, marker }),
            Self::IndexTargetNode { parser, pointer, marker } => write!(f, "{:?}", IndexTargetNode { parser, pointer, marker }),
            Self::InstanceVariableAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", InstanceVariableAndWriteNode { parser, pointer, marker }),
            Self::InstanceVariableOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", InstanceVariableOperatorWriteNode { parser, pointer, marker }),
            Self::InstanceVariableOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", InstanceVariableOrWriteNode { parser, pointer, marker }),
            Self::InstanceVariableReadNode { parser, pointer, marker } => write!(f, "{:?}", InstanceVariableReadNode { parser, pointer, marker }),
            Self::InstanceVariableTargetNode { parser, pointer, marker } => write!(f, "{:?}", InstanceVariableTargetNode { parser, pointer, marker }),
            Self::InstanceVariableWriteNode { parser, pointer, marker } => write!(f, "{:?}", InstanceVariableWriteNode { parser, pointer, marker }),
            Self::IntegerNode { parser, pointer, marker } => write!(f, "{:?}", IntegerNode { parser, pointer, marker }),
            Self::InterpolatedMatchLastLineNode { parser, pointer, marker } => write!(f, "{:?}", InterpolatedMatchLastLineNode { parser, pointer, marker }),
            Self::InterpolatedRegularExpressionNode { parser, pointer, marker } => write!(f, "{:?}", InterpolatedRegularExpressionNode { parser, pointer, marker }),
            Self::InterpolatedStringNode { parser, pointer, marker } => write!(f, "{:?}", InterpolatedStringNode { parser, pointer, marker }),
            Self::InterpolatedSymbolNode { parser, pointer, marker } => write!(f, "{:?}", InterpolatedSymbolNode { parser, pointer, marker }),
            Self::InterpolatedXStringNode { parser, pointer, marker } => write!(f, "{:?}", InterpolatedXStringNode { parser, pointer, marker }),
            Self::ItLocalVariableReadNode { parser, pointer, marker } => write!(f, "{:?}", ItLocalVariableReadNode { parser, pointer, marker }),
            Self::ItParametersNode { parser, pointer, marker } => write!(f, "{:?}", ItParametersNode { parser, pointer, marker }),
            Self::KeywordHashNode { parser, pointer, marker } => write!(f, "{:?}", KeywordHashNode { parser, pointer, marker }),
            Self::KeywordRestParameterNode { parser, pointer, marker } => write!(f, "{:?}", KeywordRestParameterNode { parser, pointer, marker }),
            Self::LambdaNode { parser, pointer, marker } => write!(f, "{:?}", LambdaNode { parser, pointer, marker }),
            Self::LocalVariableAndWriteNode { parser, pointer, marker } => write!(f, "{:?}", LocalVariableAndWriteNode { parser, pointer, marker }),
            Self::LocalVariableOperatorWriteNode { parser, pointer, marker } => write!(f, "{:?}", LocalVariableOperatorWriteNode { parser, pointer, marker }),
            Self::LocalVariableOrWriteNode { parser, pointer, marker } => write!(f, "{:?}", LocalVariableOrWriteNode { parser, pointer, marker }),
            Self::LocalVariableReadNode { parser, pointer, marker } => write!(f, "{:?}", LocalVariableReadNode { parser, pointer, marker }),
            Self::LocalVariableTargetNode { parser, pointer, marker } => write!(f, "{:?}", LocalVariableTargetNode { parser, pointer, marker }),
            Self::LocalVariableWriteNode { parser, pointer, marker } => write!(f, "{:?}", LocalVariableWriteNode { parser, pointer, marker }),
            Self::MatchLastLineNode { parser, pointer, marker } => write!(f, "{:?}", MatchLastLineNode { parser, pointer, marker }),
            Self::MatchPredicateNode { parser, pointer, marker } => write!(f, "{:?}", MatchPredicateNode { parser, pointer, marker }),
            Self::MatchRequiredNode { parser, pointer, marker } => write!(f, "{:?}", MatchRequiredNode { parser, pointer, marker }),
            Self::MatchWriteNode { parser, pointer, marker } => write!(f, "{:?}", MatchWriteNode { parser, pointer, marker }),
            Self::MissingNode { parser, pointer, marker } => write!(f, "{:?}", MissingNode { parser, pointer, marker }),
            Self::ModuleNode { parser, pointer, marker } => write!(f, "{:?}", ModuleNode { parser, pointer, marker }),
            Self::MultiTargetNode { parser, pointer, marker } => write!(f, "{:?}", MultiTargetNode { parser, pointer, marker }),
            Self::MultiWriteNode { parser, pointer, marker } => write!(f, "{:?}", MultiWriteNode { parser, pointer, marker }),
            Self::NextNode { parser, pointer, marker } => write!(f, "{:?}", NextNode { parser, pointer, marker }),
            Self::NilNode { parser, pointer, marker } => write!(f, "{:?}", NilNode { parser, pointer, marker }),
            Self::NoKeywordsParameterNode { parser, pointer, marker } => write!(f, "{:?}", NoKeywordsParameterNode { parser, pointer, marker }),
            Self::NumberedParametersNode { parser, pointer, marker } => write!(f, "{:?}", NumberedParametersNode { parser, pointer, marker }),
            Self::NumberedReferenceReadNode { parser, pointer, marker } => write!(f, "{:?}", NumberedReferenceReadNode { parser, pointer, marker }),
            Self::OptionalKeywordParameterNode { parser, pointer, marker } => write!(f, "{:?}", OptionalKeywordParameterNode { parser, pointer, marker }),
            Self::OptionalParameterNode { parser, pointer, marker } => write!(f, "{:?}", OptionalParameterNode { parser, pointer, marker }),
            Self::OrNode { parser, pointer, marker } => write!(f, "{:?}", OrNode { parser, pointer, marker }),
            Self::ParametersNode { parser, pointer, marker } => write!(f, "{:?}", ParametersNode { parser, pointer, marker }),
            Self::ParenthesesNode { parser, pointer, marker } => write!(f, "{:?}", ParenthesesNode { parser, pointer, marker }),
            Self::PinnedExpressionNode { parser, pointer, marker } => write!(f, "{:?}", PinnedExpressionNode { parser, pointer, marker }),
            Self::PinnedVariableNode { parser, pointer, marker } => write!(f, "{:?}", PinnedVariableNode { parser, pointer, marker }),
            Self::PostExecutionNode { parser, pointer, marker } => write!(f, "{:?}", PostExecutionNode { parser, pointer, marker }),
            Self::PreExecutionNode { parser, pointer, marker } => write!(f, "{:?}", PreExecutionNode { parser, pointer, marker }),
            Self::ProgramNode { parser, pointer, marker } => write!(f, "{:?}", ProgramNode { parser, pointer, marker }),
            Self::RangeNode { parser, pointer, marker } => write!(f, "{:?}", RangeNode { parser, pointer, marker }),
            Self::RationalNode { parser, pointer, marker } => write!(f, "{:?}", RationalNode { parser, pointer, marker }),
            Self::RedoNode { parser, pointer, marker } => write!(f, "{:?}", RedoNode { parser, pointer, marker }),
            Self::RegularExpressionNode { parser, pointer, marker } => write!(f, "{:?}", RegularExpressionNode { parser, pointer, marker }),
            Self::RequiredKeywordParameterNode { parser, pointer, marker } => write!(f, "{:?}", RequiredKeywordParameterNode { parser, pointer, marker }),
            Self::RequiredParameterNode { parser, pointer, marker } => write!(f, "{:?}", RequiredParameterNode { parser, pointer, marker }),
            Self::RescueModifierNode { parser, pointer, marker } => write!(f, "{:?}", RescueModifierNode { parser, pointer, marker }),
            Self::RescueNode { parser, pointer, marker } => write!(f, "{:?}", RescueNode { parser, pointer, marker }),
            Self::RestParameterNode { parser, pointer, marker } => write!(f, "{:?}", RestParameterNode { parser, pointer, marker }),
            Self::RetryNode { parser, pointer, marker } => write!(f, "{:?}", RetryNode { parser, pointer, marker }),
            Self::ReturnNode { parser, pointer, marker } => write!(f, "{:?}", ReturnNode { parser, pointer, marker }),
            Self::SelfNode { parser, pointer, marker } => write!(f, "{:?}", SelfNode { parser, pointer, marker }),
            Self::ShareableConstantNode { parser, pointer, marker } => write!(f, "{:?}", ShareableConstantNode { parser, pointer, marker }),
            Self::SingletonClassNode { parser, pointer, marker } => write!(f, "{:?}", SingletonClassNode { parser, pointer, marker }),
            Self::SourceEncodingNode { parser, pointer, marker } => write!(f, "{:?}", SourceEncodingNode { parser, pointer, marker }),
            Self::SourceFileNode { parser, pointer, marker } => write!(f, "{:?}", SourceFileNode { parser, pointer, marker }),
            Self::SourceLineNode { parser, pointer, marker } => write!(f, "{:?}", SourceLineNode { parser, pointer, marker }),
            Self::SplatNode { parser, pointer, marker } => write!(f, "{:?}", SplatNode { parser, pointer, marker }),
            Self::StatementsNode { parser, pointer, marker } => write!(f, "{:?}", StatementsNode { parser, pointer, marker }),
            Self::StringNode { parser, pointer, marker } => write!(f, "{:?}", StringNode { parser, pointer, marker }),
            Self::SuperNode { parser, pointer, marker } => write!(f, "{:?}", SuperNode { parser, pointer, marker }),
            Self::SymbolNode { parser, pointer, marker } => write!(f, "{:?}", SymbolNode { parser, pointer, marker }),
            Self::TrueNode { parser, pointer, marker } => write!(f, "{:?}", TrueNode { parser, pointer, marker }),
            Self::UndefNode { parser, pointer, marker } => write!(f, "{:?}", UndefNode { parser, pointer, marker }),
            Self::UnlessNode { parser, pointer, marker } => write!(f, "{:?}", UnlessNode { parser, pointer, marker }),
            Self::UntilNode { parser, pointer, marker } => write!(f, "{:?}", UntilNode { parser, pointer, marker }),
            Self::WhenNode { parser, pointer, marker } => write!(f, "{:?}", WhenNode { parser, pointer, marker }),
            Self::WhileNode { parser, pointer, marker } => write!(f, "{:?}", WhileNode { parser, pointer, marker }),
            Self::XStringNode { parser, pointer, marker } => write!(f, "{:?}", XStringNode { parser, pointer, marker }),
            Self::YieldNode { parser, pointer, marker } => write!(f, "{:?}", YieldNode { parser, pointer, marker }),
        }
    }
}

/// Represents the use of the `alias` keyword to alias a global variable.
/// 
/// ```ruby
/// alias $foo $bar
/// ^^^^^^^^^^^^^^^
/// ```
pub struct AliasGlobalVariableNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_alias_global_variable_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_alias_global_variable_node_t>
}

impl<'pr> AliasGlobalVariableNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::AliasGlobalVariableNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `new_name` param
    #[must_use]
    pub fn new_name(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).new_name };
        Node::new(self.parser, node)
    }

    /// Returns the `old_name` param
    #[must_use]
    pub fn old_name(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).old_name };
        Node::new(self.parser, node)
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for AliasGlobalVariableNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AliasGlobalVariableNode({:?}, {:?}, {:?})", self.new_name(), self.old_name(), self.keyword_loc())
    }
}

/// Represents the use of the `alias` keyword to alias a method.
/// 
/// ```ruby
/// alias foo bar
/// ^^^^^^^^^^^^^
/// ```
pub struct AliasMethodNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_alias_method_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_alias_method_node_t>
}

impl<'pr> AliasMethodNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::AliasMethodNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `new_name` param
    #[must_use]
    pub fn new_name(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).new_name };
        Node::new(self.parser, node)
    }

    /// Returns the `old_name` param
    #[must_use]
    pub fn old_name(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).old_name };
        Node::new(self.parser, node)
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for AliasMethodNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AliasMethodNode({:?}, {:?}, {:?})", self.new_name(), self.old_name(), self.keyword_loc())
    }
}

/// Represents an alternation pattern in pattern matching.
/// 
/// ```ruby
/// foo => bar | baz
///        ^^^^^^^^^
/// ```
pub struct AlternationPatternNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_alternation_pattern_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_alternation_pattern_node_t>
}

impl<'pr> AlternationPatternNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::AlternationPatternNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `left` param
    #[must_use]
    pub fn left(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).left };
        Node::new(self.parser, node)
    }

    /// Returns the `right` param
    #[must_use]
    pub fn right(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).right };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for AlternationPatternNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlternationPatternNode({:?}, {:?}, {:?})", self.left(), self.right(), self.operator_loc())
    }
}

/// Represents the use of the `&&` operator or the `and` keyword.
/// 
/// ```ruby
/// left and right
/// ^^^^^^^^^^^^^^
/// ```
pub struct AndNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_and_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_and_node_t>
}

impl<'pr> AndNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::AndNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `left` param
    #[must_use]
    pub fn left(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).left };
        Node::new(self.parser, node)
    }

    /// Returns the `right` param
    #[must_use]
    pub fn right(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).right };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for AndNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AndNode({:?}, {:?}, {:?})", self.left(), self.right(), self.operator_loc())
    }
}

/// Represents a set of arguments to a method or a keyword.
/// 
/// ```ruby
/// return foo, bar, baz
///        ^^^^^^^^^^^^^
/// ```
pub struct ArgumentsNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_arguments_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_arguments_node_t>
}

impl<'pr> ArgumentsNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ArgumentsNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// if the arguments contain forwarding
    #[must_use]
    pub fn is_contains_forwarding(&self) -> bool {
        (self.flags() & PM_ARGUMENTS_NODE_FLAGS_CONTAINS_FORWARDING) != 0
    }

    /// if the arguments contain keywords
    #[must_use]
    pub fn is_contains_keywords(&self) -> bool {
        (self.flags() & PM_ARGUMENTS_NODE_FLAGS_CONTAINS_KEYWORDS) != 0
    }

    /// if the arguments contain a keyword splat
    #[must_use]
    pub fn is_contains_keyword_splat(&self) -> bool {
        (self.flags() & PM_ARGUMENTS_NODE_FLAGS_CONTAINS_KEYWORD_SPLAT) != 0
    }

    /// if the arguments contain a splat
    #[must_use]
    pub fn is_contains_splat(&self) -> bool {
        (self.flags() & PM_ARGUMENTS_NODE_FLAGS_CONTAINS_SPLAT) != 0
    }

    /// if the arguments contain multiple splats
    #[must_use]
    pub fn is_contains_multiple_splats(&self) -> bool {
        (self.flags() & PM_ARGUMENTS_NODE_FLAGS_CONTAINS_MULTIPLE_SPLATS) != 0
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).arguments };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }
}

impl std::fmt::Debug for ArgumentsNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgumentsNode({:?})", self.arguments())
    }
}

/// Represents an array literal. This can be a regular array using brackets or a special array using % like %w or %i.
/// 
/// ```ruby
/// [1, 2, 3]
/// ^^^^^^^^^
/// ```
pub struct ArrayNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_array_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_array_node_t>
}

impl<'pr> ArrayNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ArrayNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// if array contains splat nodes
    #[must_use]
    pub fn is_contains_splat(&self) -> bool {
        (self.flags() & PM_ARRAY_NODE_FLAGS_CONTAINS_SPLAT) != 0
    }

    /// Returns the `elements` param
    #[must_use]
    pub fn elements(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).elements };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for ArrayNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayNode({:?}, {:?}, {:?})", self.elements(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents an array pattern in pattern matching.
/// 
/// ```ruby
/// foo in 1, 2
/// ^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in [1, 2]
/// ^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in *bar
/// ^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in Bar[]
/// ^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in Bar[1, 2, 3]
/// ^^^^^^^^^^^^^^^^^^^
/// ```
pub struct ArrayPatternNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_array_pattern_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_array_pattern_node_t>
}

impl<'pr> ArrayPatternNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ArrayPatternNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `constant` param
    #[must_use]
    pub fn constant(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).constant };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `requireds` param
    #[must_use]
    pub fn requireds(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).requireds };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `rest` param
    #[must_use]
    pub fn rest(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).rest };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `posts` param
    #[must_use]
    pub fn posts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).posts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for ArrayPatternNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayPatternNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.constant(), self.requireds(), self.rest(), self.posts(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents a hash key/value pair.
/// 
/// ```ruby
/// { a => b }
///   ^^^^^^
/// ```
pub struct AssocNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_assoc_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_assoc_node_t>
}

impl<'pr> AssocNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::AssocNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `key` param
    #[must_use]
    pub fn key(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).key };
        Node::new(self.parser, node)
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for AssocNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssocNode({:?}, {:?}, {:?})", self.key(), self.value(), self.operator_loc())
    }
}

/// Represents a splat in a hash literal.
/// 
/// ```ruby
/// { **foo }
///   ^^^^^
/// ```
pub struct AssocSplatNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_assoc_splat_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_assoc_splat_node_t>
}

impl<'pr> AssocSplatNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::AssocSplatNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for AssocSplatNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssocSplatNode({:?}, {:?})", self.value(), self.operator_loc())
    }
}

/// Represents reading a reference to a field in the previous match.
/// 
/// ```ruby
/// $'
/// ^^
/// ```
pub struct BackReferenceReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_back_reference_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_back_reference_read_node_t>
}

impl<'pr> BackReferenceReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BackReferenceReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for BackReferenceReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BackReferenceReadNode({:?})", self.name())
    }
}

/// Represents a begin statement.
/// 
/// ```ruby
/// begin
///   foo
/// end
/// ^^^^^
/// ```
pub struct BeginNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_begin_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_begin_node_t>
}

impl<'pr> BeginNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BeginNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `begin_keyword_loc` param
    #[must_use]
    pub fn begin_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).begin_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `rescue_clause` param
    #[must_use]
    pub fn rescue_clause(&self) -> Option<RescueNode<'pr>> {
        let node: *mut pm_rescue_node_t = unsafe { (*self.pointer).rescue_clause };
        if node.is_null() {
            None
        } else {
            Some(RescueNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `else_clause` param
    #[must_use]
    pub fn else_clause(&self) -> Option<ElseNode<'pr>> {
        let node: *mut pm_else_node_t = unsafe { (*self.pointer).else_clause };
        if node.is_null() {
            None
        } else {
            Some(ElseNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `ensure_clause` param
    #[must_use]
    pub fn ensure_clause(&self) -> Option<EnsureNode<'pr>> {
        let node: *mut pm_ensure_node_t = unsafe { (*self.pointer).ensure_clause };
        if node.is_null() {
            None
        } else {
            Some(EnsureNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for BeginNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BeginNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.begin_keyword_loc(), self.statements(), self.rescue_clause(), self.else_clause(), self.ensure_clause(), self.end_keyword_loc())
    }
}

/// Represents a block argument using `&`.
/// 
/// ```ruby
/// bar(&args)
/// ^^^^^^^^^^
/// ```
pub struct BlockArgumentNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_block_argument_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_block_argument_node_t>
}

impl<'pr> BlockArgumentNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BlockArgumentNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `expression` param
    #[must_use]
    pub fn expression(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).expression };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for BlockArgumentNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockArgumentNode({:?}, {:?})", self.expression(), self.operator_loc())
    }
}

/// Represents a block local variable.
/// 
/// ```ruby
/// a { |; b| }
///        ^
/// ```
pub struct BlockLocalVariableNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_block_local_variable_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_block_local_variable_node_t>
}

impl<'pr> BlockLocalVariableNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BlockLocalVariableNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for BlockLocalVariableNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockLocalVariableNode({:?})", self.name())
    }
}

/// Represents a block of ruby code.
/// 
/// ```ruby
/// [1, 2, 3].each { |i| puts x }
///                ^^^^^^^^^^^^^^
/// ```
pub struct BlockNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_block_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_block_node_t>
}

impl<'pr> BlockNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BlockNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `parameters` param
    #[must_use]
    pub fn parameters(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).parameters };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for BlockNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockNode({:?}, {:?}, {:?}, {:?}, {:?})", self.locals(), self.parameters(), self.body(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents a block parameter of a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(&b)
///       ^^
/// end
/// ```
pub struct BlockParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_block_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_block_parameter_node_t>
}

impl<'pr> BlockParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BlockParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> Option<ConstantId<'pr>> {
        let id = unsafe { (*self.pointer).name };
        if id == 0 {
            None
        } else {
            Some(ConstantId::new(self.parser, id))
        }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for BlockParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockParameterNode({:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc())
    }
}

/// Represents a block's parameters declaration.
/// 
/// ```ruby
/// -> (a, b = 1; local) { }
///    ^^^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo do |a, b = 1; local|
///        ^^^^^^^^^^^^^^^^^
/// end
/// ```
pub struct BlockParametersNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_block_parameters_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_block_parameters_node_t>
}

impl<'pr> BlockParametersNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BlockParametersNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `parameters` param
    #[must_use]
    pub fn parameters(&self) -> Option<ParametersNode<'pr>> {
        let node: *mut pm_parameters_node_t = unsafe { (*self.pointer).parameters };
        if node.is_null() {
            None
        } else {
            Some(ParametersNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).locals };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for BlockParametersNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockParametersNode({:?}, {:?}, {:?}, {:?})", self.parameters(), self.locals(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents the use of the `break` keyword.
/// 
/// ```ruby
/// break foo
/// ^^^^^^^^^
/// ```
pub struct BreakNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_break_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_break_node_t>
}

impl<'pr> BreakNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::BreakNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for BreakNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BreakNode({:?}, {:?})", self.arguments(), self.keyword_loc())
    }
}

/// Represents the use of the `&&=` operator on a call.
/// 
/// ```ruby
/// foo.bar &&= value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct CallAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_call_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_call_and_write_node_t>
}

impl<'pr> CallAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CallAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `message_loc` param
    #[must_use]
    pub fn message_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).message_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `read_name` param
    #[must_use]
    pub fn read_name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).read_name })
    }

    /// Returns the `write_name` param
    #[must_use]
    pub fn write_name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).write_name })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for CallAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallAndWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.message_loc(), self.read_name(), self.write_name(), self.operator_loc(), self.value())
    }
}

/// Represents a method call, in all of the various forms that can take.
/// 
/// ```ruby
/// foo
/// ^^^
/// ```
/// 
/// ```ruby
/// foo()
/// ^^^^^
/// ```
/// 
/// ```ruby
/// +foo
/// ^^^^
/// ```
/// 
/// ```ruby
/// foo + bar
/// ^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo.bar
/// ^^^^^^^
/// ```
/// 
/// ```ruby
/// foo&.bar
/// ^^^^^^^^
/// ```
pub struct CallNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_call_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_call_node_t>
}

impl<'pr> CallNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CallNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `message_loc` param
    #[must_use]
    pub fn message_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).message_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `equal_loc` param
    #[must_use]
    pub fn equal_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).equal_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }
}

impl std::fmt::Debug for CallNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.name(), self.message_loc(), self.opening_loc(), self.arguments(), self.closing_loc(), self.equal_loc(), self.block())
    }
}

/// Represents the use of an assignment operator on a call.
/// 
/// ```ruby
/// foo.bar += baz
/// ^^^^^^^^^^^^^^
/// ```
pub struct CallOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_call_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_call_operator_write_node_t>
}

impl<'pr> CallOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CallOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `message_loc` param
    #[must_use]
    pub fn message_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).message_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `read_name` param
    #[must_use]
    pub fn read_name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).read_name })
    }

    /// Returns the `write_name` param
    #[must_use]
    pub fn write_name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).write_name })
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for CallOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.message_loc(), self.read_name(), self.write_name(), self.binary_operator(), self.binary_operator_loc(), self.value())
    }
}

/// Represents the use of the `||=` operator on a call.
/// 
/// ```ruby
/// foo.bar ||= value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct CallOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_call_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_call_or_write_node_t>
}

impl<'pr> CallOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CallOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `message_loc` param
    #[must_use]
    pub fn message_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).message_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `read_name` param
    #[must_use]
    pub fn read_name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).read_name })
    }

    /// Returns the `write_name` param
    #[must_use]
    pub fn write_name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).write_name })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for CallOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallOrWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.message_loc(), self.read_name(), self.write_name(), self.operator_loc(), self.value())
    }
}

/// Represents assigning to a method call.
/// 
/// ```ruby
/// foo.bar, = 1
/// ^^^^^^^
/// ```
/// 
/// ```ruby
/// begin
/// rescue => foo.bar
///           ^^^^^^^
/// end
/// ```
/// 
/// ```ruby
/// for foo.bar in baz do end
///     ^^^^^^^
/// ```
pub struct CallTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_call_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_call_target_node_t>
}

impl<'pr> CallTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CallTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        Node::new(self.parser, node)
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `message_loc` param
    #[must_use]
    pub fn message_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).message_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for CallTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallTargetNode({:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.name(), self.message_loc())
    }
}

/// Represents assigning to a local variable in pattern matching.
/// 
/// ```ruby
/// foo => [bar => baz]
///        ^^^^^^^^^^^^
/// ```
pub struct CapturePatternNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_capture_pattern_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_capture_pattern_node_t>
}

impl<'pr> CapturePatternNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CapturePatternNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `target` param
    #[must_use]
    pub fn target(&self) -> LocalVariableTargetNode<'pr> {
        let node: *mut pm_local_variable_target_node_t = unsafe { (*self.pointer).target };
        LocalVariableTargetNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for CapturePatternNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CapturePatternNode({:?}, {:?}, {:?})", self.value(), self.target(), self.operator_loc())
    }
}

/// Represents the use of a case statement for pattern matching.
/// 
/// ```ruby
/// case true
/// in false
/// end
/// ^^^^^^^^^
/// ```
pub struct CaseMatchNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_case_match_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_case_match_node_t>
}

impl<'pr> CaseMatchNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CaseMatchNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `predicate` param
    #[must_use]
    pub fn predicate(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).predicate };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `conditions` param
    #[must_use]
    pub fn conditions(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).conditions };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `else_clause` param
    #[must_use]
    pub fn else_clause(&self) -> Option<ElseNode<'pr>> {
        let node: *mut pm_else_node_t = unsafe { (*self.pointer).else_clause };
        if node.is_null() {
            None
        } else {
            Some(ElseNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `case_keyword_loc` param
    #[must_use]
    pub fn case_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).case_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for CaseMatchNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CaseMatchNode({:?}, {:?}, {:?}, {:?}, {:?})", self.predicate(), self.conditions(), self.else_clause(), self.case_keyword_loc(), self.end_keyword_loc())
    }
}

/// Represents the use of a case statement.
/// 
/// ```ruby
/// case true
/// when false
/// end
/// ^^^^^^^^^^
/// ```
pub struct CaseNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_case_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_case_node_t>
}

impl<'pr> CaseNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::CaseNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `predicate` param
    #[must_use]
    pub fn predicate(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).predicate };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `conditions` param
    #[must_use]
    pub fn conditions(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).conditions };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `else_clause` param
    #[must_use]
    pub fn else_clause(&self) -> Option<ElseNode<'pr>> {
        let node: *mut pm_else_node_t = unsafe { (*self.pointer).else_clause };
        if node.is_null() {
            None
        } else {
            Some(ElseNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `case_keyword_loc` param
    #[must_use]
    pub fn case_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).case_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for CaseNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CaseNode({:?}, {:?}, {:?}, {:?}, {:?})", self.predicate(), self.conditions(), self.else_clause(), self.case_keyword_loc(), self.end_keyword_loc())
    }
}

/// Represents a class declaration involving the `class` keyword.
/// 
/// ```ruby
/// class Foo end
/// ^^^^^^^^^^^^^
/// ```
pub struct ClassNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_node_t>
}

impl<'pr> ClassNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `class_keyword_loc` param
    #[must_use]
    pub fn class_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).class_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `constant_path` param
    #[must_use]
    pub fn constant_path(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).constant_path };
        Node::new(self.parser, node)
    }

    /// Returns the `inheritance_operator_loc` param
    #[must_use]
    pub fn inheritance_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).inheritance_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `superclass` param
    #[must_use]
    pub fn superclass(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).superclass };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for ClassNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.locals(), self.class_keyword_loc(), self.constant_path(), self.inheritance_operator_loc(), self.superclass(), self.body(), self.end_keyword_loc(), self.name())
    }
}

/// Represents the use of the `&&=` operator for assignment to a class variable.
/// 
/// ```ruby
/// @@target &&= value
/// ^^^^^^^^^^^^^^^^^^
/// ```
pub struct ClassVariableAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_variable_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_variable_and_write_node_t>
}

impl<'pr> ClassVariableAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassVariableAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ClassVariableAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassVariableAndWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents assigning to a class variable using an operator that isn't `=`.
/// 
/// ```ruby
/// @@target += value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct ClassVariableOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_variable_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_variable_operator_write_node_t>
}

impl<'pr> ClassVariableOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassVariableOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }
}

impl std::fmt::Debug for ClassVariableOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassVariableOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.binary_operator_loc(), self.value(), self.binary_operator())
    }
}

/// Represents the use of the `||=` operator for assignment to a class variable.
/// 
/// ```ruby
/// @@target ||= value
/// ^^^^^^^^^^^^^^^^^^
/// ```
pub struct ClassVariableOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_variable_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_variable_or_write_node_t>
}

impl<'pr> ClassVariableOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassVariableOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ClassVariableOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassVariableOrWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents referencing a class variable.
/// 
/// ```ruby
/// @@foo
/// ^^^^^
/// ```
pub struct ClassVariableReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_variable_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_variable_read_node_t>
}

impl<'pr> ClassVariableReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassVariableReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for ClassVariableReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassVariableReadNode({:?})", self.name())
    }
}

/// Represents writing to a class variable in a context that doesn't have an explicit value.
/// 
/// ```ruby
/// @@foo, @@bar = baz
/// ^^^^^  ^^^^^
/// ```
pub struct ClassVariableTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_variable_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_variable_target_node_t>
}

impl<'pr> ClassVariableTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassVariableTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for ClassVariableTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassVariableTargetNode({:?})", self.name())
    }
}

/// Represents writing to a class variable.
/// 
/// ```ruby
/// @@foo = 1
/// ^^^^^^^^^
/// ```
pub struct ClassVariableWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_class_variable_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_class_variable_write_node_t>
}

impl<'pr> ClassVariableWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ClassVariableWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for ClassVariableWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassVariableWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.value(), self.operator_loc())
    }
}

/// Represents the use of the `&&=` operator for assignment to a constant.
/// 
/// ```ruby
/// Target &&= value
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct ConstantAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_and_write_node_t>
}

impl<'pr> ConstantAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ConstantAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantAndWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents assigning to a constant using an operator that isn't `=`.
/// 
/// ```ruby
/// Target += value
/// ^^^^^^^^^^^^^^^
/// ```
pub struct ConstantOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_operator_write_node_t>
}

impl<'pr> ConstantOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }
}

impl std::fmt::Debug for ConstantOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.binary_operator_loc(), self.value(), self.binary_operator())
    }
}

/// Represents the use of the `||=` operator for assignment to a constant.
/// 
/// ```ruby
/// Target ||= value
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct ConstantOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_or_write_node_t>
}

impl<'pr> ConstantOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ConstantOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantOrWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents the use of the `&&=` operator for assignment to a constant path.
/// 
/// ```ruby
/// Parent::Child &&= value
/// ^^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct ConstantPathAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_path_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_path_and_write_node_t>
}

impl<'pr> ConstantPathAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantPathAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `target` param
    #[must_use]
    pub fn target(&self) -> ConstantPathNode<'pr> {
        let node: *mut pm_constant_path_node_t = unsafe { (*self.pointer).target };
        ConstantPathNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ConstantPathAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantPathAndWriteNode({:?}, {:?}, {:?})", self.target(), self.operator_loc(), self.value())
    }
}

/// Represents accessing a constant through a path of `::` operators.
/// 
/// ```ruby
/// Foo::Bar
/// ^^^^^^^^
/// ```
pub struct ConstantPathNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_path_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_path_node_t>
}

impl<'pr> ConstantPathNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantPathNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `parent` param
    #[must_use]
    pub fn parent(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).parent };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> Option<ConstantId<'pr>> {
        let id = unsafe { (*self.pointer).name };
        if id == 0 {
            None
        } else {
            Some(ConstantId::new(self.parser, id))
        }
    }

    /// Returns the `delimiter_loc` param
    #[must_use]
    pub fn delimiter_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).delimiter_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for ConstantPathNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantPathNode({:?}, {:?}, {:?}, {:?})", self.parent(), self.name(), self.delimiter_loc(), self.name_loc())
    }
}

/// Represents assigning to a constant path using an operator that isn't `=`.
/// 
/// ```ruby
/// Parent::Child += value
/// ^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct ConstantPathOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_path_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_path_operator_write_node_t>
}

impl<'pr> ConstantPathOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantPathOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `target` param
    #[must_use]
    pub fn target(&self) -> ConstantPathNode<'pr> {
        let node: *mut pm_constant_path_node_t = unsafe { (*self.pointer).target };
        ConstantPathNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }
}

impl std::fmt::Debug for ConstantPathOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantPathOperatorWriteNode({:?}, {:?}, {:?}, {:?})", self.target(), self.binary_operator_loc(), self.value(), self.binary_operator())
    }
}

/// Represents the use of the `||=` operator for assignment to a constant path.
/// 
/// ```ruby
/// Parent::Child ||= value
/// ^^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct ConstantPathOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_path_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_path_or_write_node_t>
}

impl<'pr> ConstantPathOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantPathOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `target` param
    #[must_use]
    pub fn target(&self) -> ConstantPathNode<'pr> {
        let node: *mut pm_constant_path_node_t = unsafe { (*self.pointer).target };
        ConstantPathNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ConstantPathOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantPathOrWriteNode({:?}, {:?}, {:?})", self.target(), self.operator_loc(), self.value())
    }
}

/// Represents writing to a constant path in a context that doesn't have an explicit value.
/// 
/// ```ruby
/// Foo::Foo, Bar::Bar = baz
/// ^^^^^^^^  ^^^^^^^^
/// ```
pub struct ConstantPathTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_path_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_path_target_node_t>
}

impl<'pr> ConstantPathTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantPathTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `parent` param
    #[must_use]
    pub fn parent(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).parent };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> Option<ConstantId<'pr>> {
        let id = unsafe { (*self.pointer).name };
        if id == 0 {
            None
        } else {
            Some(ConstantId::new(self.parser, id))
        }
    }

    /// Returns the `delimiter_loc` param
    #[must_use]
    pub fn delimiter_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).delimiter_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for ConstantPathTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantPathTargetNode({:?}, {:?}, {:?}, {:?})", self.parent(), self.name(), self.delimiter_loc(), self.name_loc())
    }
}

/// Represents writing to a constant path.
/// 
/// ```ruby
/// ::Foo = 1
/// ^^^^^^^^^
/// ```
/// 
/// ```ruby
/// Foo::Bar = 1
/// ^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// ::Foo::Bar = 1
/// ^^^^^^^^^^^^^^
/// ```
pub struct ConstantPathWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_path_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_path_write_node_t>
}

impl<'pr> ConstantPathWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantPathWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `target` param
    #[must_use]
    pub fn target(&self) -> ConstantPathNode<'pr> {
        let node: *mut pm_constant_path_node_t = unsafe { (*self.pointer).target };
        ConstantPathNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ConstantPathWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantPathWriteNode({:?}, {:?}, {:?})", self.target(), self.operator_loc(), self.value())
    }
}

/// Represents referencing a constant.
/// 
/// ```ruby
/// Foo
/// ^^^
/// ```
pub struct ConstantReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_read_node_t>
}

impl<'pr> ConstantReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for ConstantReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantReadNode({:?})", self.name())
    }
}

/// Represents writing to a constant in a context that doesn't have an explicit value.
/// 
/// ```ruby
/// Foo, Bar = baz
/// ^^^  ^^^
/// ```
pub struct ConstantTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_target_node_t>
}

impl<'pr> ConstantTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for ConstantTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantTargetNode({:?})", self.name())
    }
}

/// Represents writing to a constant.
/// 
/// ```ruby
/// Foo = 1
/// ^^^^^^^
/// ```
pub struct ConstantWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_constant_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_constant_write_node_t>
}

impl<'pr> ConstantWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ConstantWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for ConstantWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.value(), self.operator_loc())
    }
}

/// Represents a method definition.
/// 
/// ```ruby
/// def method
/// end
/// ^^^^^^^^^^
/// ```
pub struct DefNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_def_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_def_node_t>
}

impl<'pr> DefNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::DefNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `parameters` param
    #[must_use]
    pub fn parameters(&self) -> Option<ParametersNode<'pr>> {
        let node: *mut pm_parameters_node_t = unsafe { (*self.pointer).parameters };
        if node.is_null() {
            None
        } else {
            Some(ParametersNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `def_keyword_loc` param
    #[must_use]
    pub fn def_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).def_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `equal_loc` param
    #[must_use]
    pub fn equal_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).equal_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for DefNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DefNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.receiver(), self.parameters(), self.body(), self.locals(), self.def_keyword_loc(), self.operator_loc(), self.lparen_loc(), self.rparen_loc(), self.equal_loc(), self.end_keyword_loc())
    }
}

/// Represents the use of the `defined?` keyword.
/// 
/// ```ruby
/// defined?(a)
/// ^^^^^^^^^^^
/// ```
pub struct DefinedNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_defined_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_defined_node_t>
}

impl<'pr> DefinedNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::DefinedNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for DefinedNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DefinedNode({:?}, {:?}, {:?}, {:?})", self.lparen_loc(), self.value(), self.rparen_loc(), self.keyword_loc())
    }
}

/// Represents an `else` clause in a `case`, `if`, or `unless` statement.
/// 
/// ```ruby
/// if a then b else c end
///             ^^^^^^^^^^
/// ```
pub struct ElseNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_else_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_else_node_t>
}

impl<'pr> ElseNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ElseNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `else_keyword_loc` param
    #[must_use]
    pub fn else_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).else_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for ElseNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ElseNode({:?}, {:?}, {:?})", self.else_keyword_loc(), self.statements(), self.end_keyword_loc())
    }
}

/// Represents an interpolated set of statements.
/// 
/// ```ruby
/// "foo #{bar}"
///      ^^^^^^
/// ```
pub struct EmbeddedStatementsNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_embedded_statements_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_embedded_statements_node_t>
}

impl<'pr> EmbeddedStatementsNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::EmbeddedStatementsNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for EmbeddedStatementsNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmbeddedStatementsNode({:?}, {:?}, {:?})", self.opening_loc(), self.statements(), self.closing_loc())
    }
}

/// Represents an interpolated variable.
/// 
/// ```ruby
/// "foo #@bar"
///      ^^^^^
/// ```
pub struct EmbeddedVariableNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_embedded_variable_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_embedded_variable_node_t>
}

impl<'pr> EmbeddedVariableNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::EmbeddedVariableNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `variable` param
    #[must_use]
    pub fn variable(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).variable };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for EmbeddedVariableNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmbeddedVariableNode({:?}, {:?})", self.operator_loc(), self.variable())
    }
}

/// Represents an `ensure` clause in a `begin` statement.
/// 
/// ```ruby
/// begin
///   foo
/// ensure
/// ^^^^^^
///   bar
/// end
/// ```
pub struct EnsureNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_ensure_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_ensure_node_t>
}

impl<'pr> EnsureNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::EnsureNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `ensure_keyword_loc` param
    #[must_use]
    pub fn ensure_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).ensure_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for EnsureNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnsureNode({:?}, {:?}, {:?})", self.ensure_keyword_loc(), self.statements(), self.end_keyword_loc())
    }
}

/// Represents the use of the literal `false` keyword.
/// 
/// ```ruby
/// false
/// ^^^^^
/// ```
pub struct FalseNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_false_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_false_node_t>
}

impl<'pr> FalseNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::FalseNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for FalseNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FalseNode()")
    }
}

/// Represents a find pattern in pattern matching.
/// 
/// ```ruby
/// foo in *bar, baz, *qux
///        ^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in [*bar, baz, *qux]
///        ^^^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in Foo(*bar, baz, *qux)
///        ^^^^^^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo => *bar, baz, *qux
///        ^^^^^^^^^^^^^^^
/// ```
pub struct FindPatternNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_find_pattern_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_find_pattern_node_t>
}

impl<'pr> FindPatternNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::FindPatternNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `constant` param
    #[must_use]
    pub fn constant(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).constant };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `left` param
    #[must_use]
    pub fn left(&self) -> SplatNode<'pr> {
        let node: *mut pm_splat_node_t = unsafe { (*self.pointer).left };
        SplatNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `requireds` param
    #[must_use]
    pub fn requireds(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).requireds };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `right` param
    #[must_use]
    pub fn right(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).right };
        Node::new(self.parser, node)
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for FindPatternNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FindPatternNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.constant(), self.left(), self.requireds(), self.right(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents the use of the `..` or `...` operators to create flip flops.
/// 
/// ```ruby
/// baz if foo .. bar
///        ^^^^^^^^^^
/// ```
pub struct FlipFlopNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_flip_flop_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_flip_flop_node_t>
}

impl<'pr> FlipFlopNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::FlipFlopNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// ... operator
    #[must_use]
    pub fn is_exclude_end(&self) -> bool {
        (self.flags() & PM_RANGE_FLAGS_EXCLUDE_END) != 0
    }

    /// Returns the `left` param
    #[must_use]
    pub fn left(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).left };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `right` param
    #[must_use]
    pub fn right(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).right };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for FlipFlopNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlipFlopNode({:?}, {:?}, {:?})", self.left(), self.right(), self.operator_loc())
    }
}

/// Represents a floating point number literal.
/// 
/// ```ruby
/// 1.0
/// ^^^
/// ```
pub struct FloatNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_float_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_float_node_t>
}

impl<'pr> FloatNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::FloatNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> f64 {
        unsafe { (*self.pointer).value }
    }
}

impl std::fmt::Debug for FloatNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FloatNode({:?})", self.value())
    }
}

/// Represents the use of the `for` keyword.
/// 
/// ```ruby
/// for i in a end
/// ^^^^^^^^^^^^^^
/// ```
pub struct ForNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_for_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_for_node_t>
}

impl<'pr> ForNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ForNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `index` param
    #[must_use]
    pub fn index(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).index };
        Node::new(self.parser, node)
    }

    /// Returns the `collection` param
    #[must_use]
    pub fn collection(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).collection };
        Node::new(self.parser, node)
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `for_keyword_loc` param
    #[must_use]
    pub fn for_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).for_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `in_keyword_loc` param
    #[must_use]
    pub fn in_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).in_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `do_keyword_loc` param
    #[must_use]
    pub fn do_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).do_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for ForNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.index(), self.collection(), self.statements(), self.for_keyword_loc(), self.in_keyword_loc(), self.do_keyword_loc(), self.end_keyword_loc())
    }
}

/// Represents forwarding all arguments to this method to another method.
/// 
/// ```ruby
/// def foo(...)
///   bar(...)
///       ^^^
/// end
/// ```
pub struct ForwardingArgumentsNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_forwarding_arguments_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_forwarding_arguments_node_t>
}

impl<'pr> ForwardingArgumentsNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ForwardingArgumentsNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for ForwardingArgumentsNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForwardingArgumentsNode()")
    }
}

/// Represents the use of the forwarding parameter in a method, block, or lambda declaration.
/// 
/// ```ruby
/// def foo(...)
///         ^^^
/// end
/// ```
pub struct ForwardingParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_forwarding_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_forwarding_parameter_node_t>
}

impl<'pr> ForwardingParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ForwardingParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for ForwardingParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForwardingParameterNode()")
    }
}

/// Represents the use of the `super` keyword without parentheses or arguments, but which might have a block.
/// 
/// ```ruby
/// super
/// ^^^^^
/// ```
/// 
/// ```ruby
/// super { 123 }
/// ^^^^^^^^^^^^^
/// ```
/// 
/// If it has any other arguments, it would be a `SuperNode` instead.
pub struct ForwardingSuperNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_forwarding_super_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_forwarding_super_node_t>
}

impl<'pr> ForwardingSuperNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ForwardingSuperNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<BlockNode<'pr>> {
        let node: *mut pm_block_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(BlockNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for ForwardingSuperNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ForwardingSuperNode({:?})", self.block())
    }
}

/// Represents the use of the `&&=` operator for assignment to a global variable.
/// 
/// ```ruby
/// $target &&= value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct GlobalVariableAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_global_variable_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_global_variable_and_write_node_t>
}

impl<'pr> GlobalVariableAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::GlobalVariableAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for GlobalVariableAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalVariableAndWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents assigning to a global variable using an operator that isn't `=`.
/// 
/// ```ruby
/// $target += value
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct GlobalVariableOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_global_variable_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_global_variable_operator_write_node_t>
}

impl<'pr> GlobalVariableOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::GlobalVariableOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }
}

impl std::fmt::Debug for GlobalVariableOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalVariableOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.binary_operator_loc(), self.value(), self.binary_operator())
    }
}

/// Represents the use of the `||=` operator for assignment to a global variable.
/// 
/// ```ruby
/// $target ||= value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct GlobalVariableOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_global_variable_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_global_variable_or_write_node_t>
}

impl<'pr> GlobalVariableOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::GlobalVariableOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for GlobalVariableOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalVariableOrWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents referencing a global variable.
/// 
/// ```ruby
/// $foo
/// ^^^^
/// ```
pub struct GlobalVariableReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_global_variable_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_global_variable_read_node_t>
}

impl<'pr> GlobalVariableReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::GlobalVariableReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for GlobalVariableReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalVariableReadNode({:?})", self.name())
    }
}

/// Represents writing to a global variable in a context that doesn't have an explicit value.
/// 
/// ```ruby
/// $foo, $bar = baz
/// ^^^^  ^^^^
/// ```
pub struct GlobalVariableTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_global_variable_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_global_variable_target_node_t>
}

impl<'pr> GlobalVariableTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::GlobalVariableTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for GlobalVariableTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalVariableTargetNode({:?})", self.name())
    }
}

/// Represents writing to a global variable.
/// 
/// ```ruby
/// $foo = 1
/// ^^^^^^^^
/// ```
pub struct GlobalVariableWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_global_variable_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_global_variable_write_node_t>
}

impl<'pr> GlobalVariableWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::GlobalVariableWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for GlobalVariableWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalVariableWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.value(), self.operator_loc())
    }
}

/// Represents a hash literal.
/// 
/// ```ruby
/// { a => b }
/// ^^^^^^^^^^
/// ```
pub struct HashNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_hash_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_hash_node_t>
}

impl<'pr> HashNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::HashNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `elements` param
    #[must_use]
    pub fn elements(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).elements };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for HashNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashNode({:?}, {:?}, {:?})", self.opening_loc(), self.elements(), self.closing_loc())
    }
}

/// Represents a hash pattern in pattern matching.
/// 
/// ```ruby
/// foo => { a: 1, b: 2 }
///        ^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo => { a: 1, b: 2, **c }
///        ^^^^^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo => Bar[a: 1, b: 2]
///        ^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo in { a: 1, b: 2 }
///        ^^^^^^^^^^^^^^
/// ```
pub struct HashPatternNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_hash_pattern_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_hash_pattern_node_t>
}

impl<'pr> HashPatternNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::HashPatternNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `constant` param
    #[must_use]
    pub fn constant(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).constant };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `elements` param
    #[must_use]
    pub fn elements(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).elements };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `rest` param
    #[must_use]
    pub fn rest(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).rest };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for HashPatternNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashPatternNode({:?}, {:?}, {:?}, {:?}, {:?})", self.constant(), self.elements(), self.rest(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents the use of the `if` keyword, either in the block form or the modifier form, or a ternary expression.
/// 
/// ```ruby
/// bar if foo
/// ^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// if foo then bar end
/// ^^^^^^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// foo ? bar : baz
/// ^^^^^^^^^^^^^^^
/// ```
pub struct IfNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_if_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_if_node_t>
}

impl<'pr> IfNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::IfNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `if_keyword_loc` param
    #[must_use]
    pub fn if_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).if_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `predicate` param
    #[must_use]
    pub fn predicate(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).predicate };
        Node::new(self.parser, node)
    }

    /// Returns the `then_keyword_loc` param
    #[must_use]
    pub fn then_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).then_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `subsequent` param
    #[must_use]
    pub fn subsequent(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).subsequent };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for IfNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IfNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.if_keyword_loc(), self.predicate(), self.then_keyword_loc(), self.statements(), self.subsequent(), self.end_keyword_loc())
    }
}

/// Represents an imaginary number literal.
/// 
/// ```ruby
/// 1.0i
/// ^^^^
/// ```
pub struct ImaginaryNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_imaginary_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_imaginary_node_t>
}

impl<'pr> ImaginaryNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ImaginaryNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `numeric` param
    #[must_use]
    pub fn numeric(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).numeric };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ImaginaryNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImaginaryNode({:?})", self.numeric())
    }
}

/// Represents a node that is implicitly being added to the tree but doesn't correspond directly to a node in the source.
/// 
/// ```ruby
/// { foo: }
///   ^^^^
/// ```
/// 
/// ```ruby
/// { Foo: }
///   ^^^^
/// ```
/// 
/// ```ruby
/// foo in { bar: }
///          ^^^^
/// ```
pub struct ImplicitNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_implicit_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_implicit_node_t>
}

impl<'pr> ImplicitNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ImplicitNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ImplicitNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImplicitNode({:?})", self.value())
    }
}

/// Represents using a trailing comma to indicate an implicit rest parameter.
/// 
/// ```ruby
/// foo { |bar,| }
///           ^
/// ```
/// 
/// ```ruby
/// foo in [bar,]
///            ^
/// ```
/// 
/// ```ruby
/// for foo, in bar do end
///        ^
/// ```
/// 
/// ```ruby
/// foo, = bar
///    ^
/// ```
pub struct ImplicitRestNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_implicit_rest_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_implicit_rest_node_t>
}

impl<'pr> ImplicitRestNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ImplicitRestNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for ImplicitRestNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImplicitRestNode()")
    }
}

/// Represents the use of the `in` keyword in a case statement.
/// 
/// ```ruby
/// case a; in b then c end
///         ^^^^^^^^^^^
/// ```
pub struct InNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_in_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_in_node_t>
}

impl<'pr> InNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `pattern` param
    #[must_use]
    pub fn pattern(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).pattern };
        Node::new(self.parser, node)
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `in_loc` param
    #[must_use]
    pub fn in_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).in_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `then_loc` param
    #[must_use]
    pub fn then_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).then_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for InNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InNode({:?}, {:?}, {:?}, {:?})", self.pattern(), self.statements(), self.in_loc(), self.then_loc())
    }
}

/// Represents the use of the `&&=` operator on a call to the `[]` method.
/// 
/// ```ruby
/// foo.bar[baz] &&= value
/// ^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct IndexAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_index_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_index_and_write_node_t>
}

impl<'pr> IndexAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::IndexAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<BlockArgumentNode<'pr>> {
        let node: *mut pm_block_argument_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(BlockArgumentNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for IndexAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexAndWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.opening_loc(), self.arguments(), self.closing_loc(), self.block(), self.operator_loc(), self.value())
    }
}

/// Represents the use of an assignment operator on a call to `[]`.
/// 
/// ```ruby
/// foo.bar[baz] += value
/// ^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct IndexOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_index_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_index_operator_write_node_t>
}

impl<'pr> IndexOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::IndexOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<BlockArgumentNode<'pr>> {
        let node: *mut pm_block_argument_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(BlockArgumentNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for IndexOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.opening_loc(), self.arguments(), self.closing_loc(), self.block(), self.binary_operator(), self.binary_operator_loc(), self.value())
    }
}

/// Represents the use of the `||=` operator on a call to `[]`.
/// 
/// ```ruby
/// foo.bar[baz] ||= value
/// ^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct IndexOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_index_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_index_or_write_node_t>
}

impl<'pr> IndexOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::IndexOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `call_operator_loc` param
    #[must_use]
    pub fn call_operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).call_operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<BlockArgumentNode<'pr>> {
        let node: *mut pm_block_argument_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(BlockArgumentNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for IndexOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexOrWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.call_operator_loc(), self.opening_loc(), self.arguments(), self.closing_loc(), self.block(), self.operator_loc(), self.value())
    }
}

/// Represents assigning to an index.
/// 
/// ```ruby
/// foo[bar], = 1
/// ^^^^^^^^
/// ```
/// 
/// ```ruby
/// begin
/// rescue => foo[bar]
///           ^^^^^^^^
/// end
/// ```
/// 
/// ```ruby
/// for foo[bar] in baz do end
///     ^^^^^^^^
/// ```
pub struct IndexTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_index_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_index_target_node_t>
}

impl<'pr> IndexTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::IndexTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// &. operator
    #[must_use]
    pub fn is_safe_navigation(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_SAFE_NAVIGATION) != 0
    }

    /// a call that could have been a local variable
    #[must_use]
    pub fn is_variable_call(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_VARIABLE_CALL) != 0
    }

    /// a call that is an attribute write, so the value being written should be returned
    #[must_use]
    pub fn is_attribute_write(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_ATTRIBUTE_WRITE) != 0
    }

    /// a call that ignores method visibility
    #[must_use]
    pub fn is_ignore_visibility(&self) -> bool {
        (self.flags() & PM_CALL_NODE_FLAGS_IGNORE_VISIBILITY) != 0
    }

    /// Returns the `receiver` param
    #[must_use]
    pub fn receiver(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).receiver };
        Node::new(self.parser, node)
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<BlockArgumentNode<'pr>> {
        let node: *mut pm_block_argument_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(BlockArgumentNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for IndexTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexTargetNode({:?}, {:?}, {:?}, {:?}, {:?})", self.receiver(), self.opening_loc(), self.arguments(), self.closing_loc(), self.block())
    }
}

/// Represents the use of the `&&=` operator for assignment to an instance variable.
/// 
/// ```ruby
/// @target &&= value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct InstanceVariableAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_instance_variable_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_instance_variable_and_write_node_t>
}

impl<'pr> InstanceVariableAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InstanceVariableAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for InstanceVariableAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceVariableAndWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents assigning to an instance variable using an operator that isn't `=`.
/// 
/// ```ruby
/// @target += value
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct InstanceVariableOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_instance_variable_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_instance_variable_operator_write_node_t>
}

impl<'pr> InstanceVariableOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InstanceVariableOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }
}

impl std::fmt::Debug for InstanceVariableOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceVariableOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.binary_operator_loc(), self.value(), self.binary_operator())
    }
}

/// Represents the use of the `||=` operator for assignment to an instance variable.
/// 
/// ```ruby
/// @target ||= value
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct InstanceVariableOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_instance_variable_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_instance_variable_or_write_node_t>
}

impl<'pr> InstanceVariableOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InstanceVariableOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for InstanceVariableOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceVariableOrWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents referencing an instance variable.
/// 
/// ```ruby
/// @foo
/// ^^^^
/// ```
pub struct InstanceVariableReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_instance_variable_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_instance_variable_read_node_t>
}

impl<'pr> InstanceVariableReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InstanceVariableReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for InstanceVariableReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceVariableReadNode({:?})", self.name())
    }
}

/// Represents writing to an instance variable in a context that doesn't have an explicit value.
/// 
/// ```ruby
/// @foo, @bar = baz
/// ^^^^  ^^^^
/// ```
pub struct InstanceVariableTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_instance_variable_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_instance_variable_target_node_t>
}

impl<'pr> InstanceVariableTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InstanceVariableTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for InstanceVariableTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceVariableTargetNode({:?})", self.name())
    }
}

/// Represents writing to an instance variable.
/// 
/// ```ruby
/// @foo = 1
/// ^^^^^^^^
/// ```
pub struct InstanceVariableWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_instance_variable_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_instance_variable_write_node_t>
}

impl<'pr> InstanceVariableWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InstanceVariableWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for InstanceVariableWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstanceVariableWriteNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.value(), self.operator_loc())
    }
}

/// Represents an integer number literal.
/// 
/// ```ruby
/// 1
/// ^
/// ```
pub struct IntegerNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_integer_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_integer_node_t>
}

impl<'pr> IntegerNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::IntegerNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// 0b prefix
    #[must_use]
    pub fn is_binary(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_BINARY) != 0
    }

    /// 0d or no prefix
    #[must_use]
    pub fn is_decimal(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_DECIMAL) != 0
    }

    /// 0o or 0 prefix
    #[must_use]
    pub fn is_octal(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_OCTAL) != 0
    }

    /// 0x prefix
    #[must_use]
    pub fn is_hexadecimal(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_HEXADECIMAL) != 0
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Integer<'pr> {
        Integer::new(unsafe { &raw const(*self.pointer).value })
    }
}

impl std::fmt::Debug for IntegerNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntegerNode({:?})", self.value())
    }
}

/// Represents a regular expression literal that contains interpolation that is being used in the predicate of a conditional to implicitly match against the last line read by an IO object.
/// 
/// ```ruby
/// if /foo #{bar} baz/ then end
///    ^^^^^^^^^^^^^^^^
/// ```
pub struct InterpolatedMatchLastLineNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_interpolated_match_last_line_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_interpolated_match_last_line_node_t>
}

impl<'pr> InterpolatedMatchLastLineNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InterpolatedMatchLastLineNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// i - ignores the case of characters when matching
    #[must_use]
    pub fn is_ignore_case(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_IGNORE_CASE) != 0
    }

    /// x - ignores whitespace and allows comments in regular expressions
    #[must_use]
    pub fn is_extended(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EXTENDED) != 0
    }

    /// m - allows $ to match the end of lines within strings
    #[must_use]
    pub fn is_multi_line(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_MULTI_LINE) != 0
    }

    /// o - only interpolates values into the regular expression once
    #[must_use]
    pub fn is_once(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ONCE) != 0
    }

    /// e - forces the EUC-JP encoding
    #[must_use]
    pub fn is_euc_jp(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EUC_JP) != 0
    }

    /// n - forces the ASCII-8BIT encoding
    #[must_use]
    pub fn is_ascii_8bit(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ASCII_8BIT) != 0
    }

    /// s - forces the Windows-31J encoding
    #[must_use]
    pub fn is_windows_31j(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_WINDOWS_31J) != 0
    }

    /// u - forces the UTF-8 encoding
    #[must_use]
    pub fn is_utf_8(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_UTF_8) != 0
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// internal bytes forced the encoding to US-ASCII
    #[must_use]
    pub fn is_forced_us_ascii_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_US_ASCII_ENCODING) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `parts` param
    #[must_use]
    pub fn parts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).parts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for InterpolatedMatchLastLineNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterpolatedMatchLastLineNode({:?}, {:?}, {:?})", self.opening_loc(), self.parts(), self.closing_loc())
    }
}

/// Represents a regular expression literal that contains interpolation.
/// 
/// ```ruby
/// /foo #{bar} baz/
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct InterpolatedRegularExpressionNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_interpolated_regular_expression_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_interpolated_regular_expression_node_t>
}

impl<'pr> InterpolatedRegularExpressionNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InterpolatedRegularExpressionNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// i - ignores the case of characters when matching
    #[must_use]
    pub fn is_ignore_case(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_IGNORE_CASE) != 0
    }

    /// x - ignores whitespace and allows comments in regular expressions
    #[must_use]
    pub fn is_extended(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EXTENDED) != 0
    }

    /// m - allows $ to match the end of lines within strings
    #[must_use]
    pub fn is_multi_line(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_MULTI_LINE) != 0
    }

    /// o - only interpolates values into the regular expression once
    #[must_use]
    pub fn is_once(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ONCE) != 0
    }

    /// e - forces the EUC-JP encoding
    #[must_use]
    pub fn is_euc_jp(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EUC_JP) != 0
    }

    /// n - forces the ASCII-8BIT encoding
    #[must_use]
    pub fn is_ascii_8bit(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ASCII_8BIT) != 0
    }

    /// s - forces the Windows-31J encoding
    #[must_use]
    pub fn is_windows_31j(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_WINDOWS_31J) != 0
    }

    /// u - forces the UTF-8 encoding
    #[must_use]
    pub fn is_utf_8(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_UTF_8) != 0
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// internal bytes forced the encoding to US-ASCII
    #[must_use]
    pub fn is_forced_us_ascii_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_US_ASCII_ENCODING) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `parts` param
    #[must_use]
    pub fn parts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).parts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for InterpolatedRegularExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterpolatedRegularExpressionNode({:?}, {:?}, {:?})", self.opening_loc(), self.parts(), self.closing_loc())
    }
}

/// Represents a string literal that contains interpolation.
/// 
/// ```ruby
/// "foo #{bar} baz"
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct InterpolatedStringNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_interpolated_string_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_interpolated_string_node_t>
}

impl<'pr> InterpolatedStringNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InterpolatedStringNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// frozen by virtue of a `frozen_string_literal: true` comment or `--enable-frozen-string-literal`; only for adjacent string literals like `'a' 'b'`
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        (self.flags() & PM_INTERPOLATED_STRING_NODE_FLAGS_FROZEN) != 0
    }

    /// mutable by virtue of a `frozen_string_literal: false` comment or `--disable-frozen-string-literal`; only for adjacent string literals like `'a' 'b'`
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        (self.flags() & PM_INTERPOLATED_STRING_NODE_FLAGS_MUTABLE) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `parts` param
    #[must_use]
    pub fn parts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).parts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for InterpolatedStringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterpolatedStringNode({:?}, {:?}, {:?})", self.opening_loc(), self.parts(), self.closing_loc())
    }
}

/// Represents a symbol literal that contains interpolation.
/// 
/// ```ruby
/// :"foo #{bar} baz"
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct InterpolatedSymbolNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_interpolated_symbol_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_interpolated_symbol_node_t>
}

impl<'pr> InterpolatedSymbolNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InterpolatedSymbolNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `parts` param
    #[must_use]
    pub fn parts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).parts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for InterpolatedSymbolNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterpolatedSymbolNode({:?}, {:?}, {:?})", self.opening_loc(), self.parts(), self.closing_loc())
    }
}

/// Represents an xstring literal that contains interpolation.
/// 
/// ```ruby
/// `foo #{bar} baz`
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct InterpolatedXStringNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_interpolated_x_string_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_interpolated_x_string_node_t>
}

impl<'pr> InterpolatedXStringNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::InterpolatedXStringNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `parts` param
    #[must_use]
    pub fn parts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).parts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for InterpolatedXStringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InterpolatedXStringNode({:?}, {:?}, {:?})", self.opening_loc(), self.parts(), self.closing_loc())
    }
}

/// Represents reading from the implicit `it` local variable.
/// 
/// ```ruby
/// -> { it }
///      ^^
/// ```
pub struct ItLocalVariableReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_it_local_variable_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_it_local_variable_read_node_t>
}

impl<'pr> ItLocalVariableReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ItLocalVariableReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for ItLocalVariableReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItLocalVariableReadNode()")
    }
}

/// Represents an implicit set of parameters through the use of the `it` keyword within a block or lambda.
/// 
/// ```ruby
/// -> { it + it }
/// ^^^^^^^^^^^^^^
/// ```
pub struct ItParametersNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_it_parameters_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_it_parameters_node_t>
}

impl<'pr> ItParametersNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ItParametersNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for ItParametersNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ItParametersNode()")
    }
}

/// Represents a hash literal without opening and closing braces.
/// 
/// ```ruby
/// foo(a: b)
///     ^^^^
/// ```
pub struct KeywordHashNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_keyword_hash_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_keyword_hash_node_t>
}

impl<'pr> KeywordHashNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::KeywordHashNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a keyword hash which only has `AssocNode` elements all with symbol keys, which means the elements can be treated as keyword arguments
    #[must_use]
    pub fn is_symbol_keys(&self) -> bool {
        (self.flags() & PM_KEYWORD_HASH_NODE_FLAGS_SYMBOL_KEYS) != 0
    }

    /// Returns the `elements` param
    #[must_use]
    pub fn elements(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).elements };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }
}

impl std::fmt::Debug for KeywordHashNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeywordHashNode({:?})", self.elements())
    }
}

/// Represents a keyword rest parameter to a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(**b)
///       ^^^
/// end
/// ```
pub struct KeywordRestParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_keyword_rest_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_keyword_rest_parameter_node_t>
}

impl<'pr> KeywordRestParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::KeywordRestParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> Option<ConstantId<'pr>> {
        let id = unsafe { (*self.pointer).name };
        if id == 0 {
            None
        } else {
            Some(ConstantId::new(self.parser, id))
        }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for KeywordRestParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeywordRestParameterNode({:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc())
    }
}

/// Represents using a lambda literal (not the lambda method call).
/// 
/// ```ruby
/// ->(value) { value * 2 }
/// ^^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct LambdaNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_lambda_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_lambda_node_t>
}

impl<'pr> LambdaNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LambdaNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `parameters` param
    #[must_use]
    pub fn parameters(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).parameters };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }
}

impl std::fmt::Debug for LambdaNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LambdaNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.locals(), self.operator_loc(), self.opening_loc(), self.closing_loc(), self.parameters(), self.body())
    }
}

/// Represents the use of the `&&=` operator for assignment to a local variable.
/// 
/// ```ruby
/// target &&= value
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct LocalVariableAndWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_local_variable_and_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_local_variable_and_write_node_t>
}

impl<'pr> LocalVariableAndWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LocalVariableAndWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `depth` param
    #[must_use]
    pub fn depth(&self) -> u32 {
        unsafe { (*self.pointer).depth }
    }
}

impl std::fmt::Debug for LocalVariableAndWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalVariableAndWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name_loc(), self.operator_loc(), self.value(), self.name(), self.depth())
    }
}

/// Represents assigning to a local variable using an operator that isn't `=`.
/// 
/// ```ruby
/// target += value
/// ^^^^^^^^^^^^^^^
/// ```
pub struct LocalVariableOperatorWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_local_variable_operator_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_local_variable_operator_write_node_t>
}

impl<'pr> LocalVariableOperatorWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LocalVariableOperatorWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `binary_operator_loc` param
    #[must_use]
    pub fn binary_operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).binary_operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `binary_operator` param
    #[must_use]
    pub fn binary_operator(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).binary_operator })
    }

    /// Returns the `depth` param
    #[must_use]
    pub fn depth(&self) -> u32 {
        unsafe { (*self.pointer).depth }
    }
}

impl std::fmt::Debug for LocalVariableOperatorWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalVariableOperatorWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.name_loc(), self.binary_operator_loc(), self.value(), self.name(), self.binary_operator(), self.depth())
    }
}

/// Represents the use of the `||=` operator for assignment to a local variable.
/// 
/// ```ruby
/// target ||= value
/// ^^^^^^^^^^^^^^^^
/// ```
pub struct LocalVariableOrWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_local_variable_or_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_local_variable_or_write_node_t>
}

impl<'pr> LocalVariableOrWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LocalVariableOrWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `depth` param
    #[must_use]
    pub fn depth(&self) -> u32 {
        unsafe { (*self.pointer).depth }
    }
}

impl std::fmt::Debug for LocalVariableOrWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalVariableOrWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name_loc(), self.operator_loc(), self.value(), self.name(), self.depth())
    }
}

/// Represents reading a local variable. Note that this requires that a local variable of the same name has already been written to in the same scope, otherwise it is parsed as a method call.
/// 
/// ```ruby
/// foo
/// ^^^
/// ```
pub struct LocalVariableReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_local_variable_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_local_variable_read_node_t>
}

impl<'pr> LocalVariableReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LocalVariableReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `depth` param
    #[must_use]
    pub fn depth(&self) -> u32 {
        unsafe { (*self.pointer).depth }
    }
}

impl std::fmt::Debug for LocalVariableReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalVariableReadNode({:?}, {:?})", self.name(), self.depth())
    }
}

/// Represents writing to a local variable in a context that doesn't have an explicit value.
/// 
/// ```ruby
/// foo, bar = baz
/// ^^^  ^^^
/// ```
/// 
/// ```ruby
/// foo => baz
///        ^^^
/// ```
pub struct LocalVariableTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_local_variable_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_local_variable_target_node_t>
}

impl<'pr> LocalVariableTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LocalVariableTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `depth` param
    #[must_use]
    pub fn depth(&self) -> u32 {
        unsafe { (*self.pointer).depth }
    }
}

impl std::fmt::Debug for LocalVariableTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalVariableTargetNode({:?}, {:?})", self.name(), self.depth())
    }
}

/// Represents writing to a local variable.
/// 
/// ```ruby
/// foo = 1
/// ^^^^^^^
/// ```
pub struct LocalVariableWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_local_variable_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_local_variable_write_node_t>
}

impl<'pr> LocalVariableWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::LocalVariableWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `depth` param
    #[must_use]
    pub fn depth(&self) -> u32 {
        unsafe { (*self.pointer).depth }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for LocalVariableWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalVariableWriteNode({:?}, {:?}, {:?}, {:?}, {:?})", self.name(), self.depth(), self.name_loc(), self.value(), self.operator_loc())
    }
}

/// Represents a regular expression literal used in the predicate of a conditional to implicitly match against the last line read by an IO object.
/// 
/// ```ruby
/// if /foo/i then end
///    ^^^^^^
/// ```
pub struct MatchLastLineNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_match_last_line_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_match_last_line_node_t>
}

impl<'pr> MatchLastLineNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MatchLastLineNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// i - ignores the case of characters when matching
    #[must_use]
    pub fn is_ignore_case(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_IGNORE_CASE) != 0
    }

    /// x - ignores whitespace and allows comments in regular expressions
    #[must_use]
    pub fn is_extended(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EXTENDED) != 0
    }

    /// m - allows $ to match the end of lines within strings
    #[must_use]
    pub fn is_multi_line(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_MULTI_LINE) != 0
    }

    /// o - only interpolates values into the regular expression once
    #[must_use]
    pub fn is_once(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ONCE) != 0
    }

    /// e - forces the EUC-JP encoding
    #[must_use]
    pub fn is_euc_jp(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EUC_JP) != 0
    }

    /// n - forces the ASCII-8BIT encoding
    #[must_use]
    pub fn is_ascii_8bit(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ASCII_8BIT) != 0
    }

    /// s - forces the Windows-31J encoding
    #[must_use]
    pub fn is_windows_31j(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_WINDOWS_31J) != 0
    }

    /// u - forces the UTF-8 encoding
    #[must_use]
    pub fn is_utf_8(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_UTF_8) != 0
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// internal bytes forced the encoding to US-ASCII
    #[must_use]
    pub fn is_forced_us_ascii_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_US_ASCII_ENCODING) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `content_loc` param
    #[must_use]
    pub fn content_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).content_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `unescaped` param
    #[must_use]
    pub fn unescaped(&self) -> &[u8] {
        unsafe {
            let source = (*self.pointer).unescaped.source;
            if source.is_null() {
                return &[];
            }
            let length = (*self.pointer).unescaped.length;
            std::slice::from_raw_parts(source, length)
        }
    }
}

impl std::fmt::Debug for MatchLastLineNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatchLastLineNode({:?}, {:?}, {:?}, {:?})", self.opening_loc(), self.content_loc(), self.closing_loc(), self.unescaped())
    }
}

/// Represents the use of the modifier `in` operator.
/// 
/// ```ruby
/// foo in bar
/// ^^^^^^^^^^
/// ```
pub struct MatchPredicateNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_match_predicate_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_match_predicate_node_t>
}

impl<'pr> MatchPredicateNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MatchPredicateNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `pattern` param
    #[must_use]
    pub fn pattern(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).pattern };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for MatchPredicateNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatchPredicateNode({:?}, {:?}, {:?})", self.value(), self.pattern(), self.operator_loc())
    }
}

/// Represents the use of the `=>` operator.
/// 
/// ```ruby
/// foo => bar
/// ^^^^^^^^^^
/// ```
pub struct MatchRequiredNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_match_required_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_match_required_node_t>
}

impl<'pr> MatchRequiredNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MatchRequiredNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }

    /// Returns the `pattern` param
    #[must_use]
    pub fn pattern(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).pattern };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for MatchRequiredNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatchRequiredNode({:?}, {:?}, {:?})", self.value(), self.pattern(), self.operator_loc())
    }
}

/// Represents writing local variables using a regular expression match with named capture groups.
/// 
/// ```ruby
/// /(?<foo>bar)/ =~ baz
/// ^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct MatchWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_match_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_match_write_node_t>
}

impl<'pr> MatchWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MatchWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `call` param
    #[must_use]
    pub fn call(&self) -> CallNode<'pr> {
        let node: *mut pm_call_node_t = unsafe { (*self.pointer).call };
        CallNode { parser: self.parser, pointer: node, marker: PhantomData }
    }

    /// Returns the `targets` param
    #[must_use]
    pub fn targets(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).targets };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }
}

impl std::fmt::Debug for MatchWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MatchWriteNode({:?}, {:?})", self.call(), self.targets())
    }
}

/// Represents a node that is missing from the source and results in a syntax error.
pub struct MissingNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_missing_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_missing_node_t>
}

impl<'pr> MissingNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MissingNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for MissingNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MissingNode()")
    }
}

/// Represents a module declaration involving the `module` keyword.
/// 
/// ```ruby
/// module Foo end
/// ^^^^^^^^^^^^^^
/// ```
pub struct ModuleNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_module_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_module_node_t>
}

impl<'pr> ModuleNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ModuleNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `module_keyword_loc` param
    #[must_use]
    pub fn module_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).module_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `constant_path` param
    #[must_use]
    pub fn constant_path(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).constant_path };
        Node::new(self.parser, node)
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for ModuleNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.locals(), self.module_keyword_loc(), self.constant_path(), self.body(), self.end_keyword_loc(), self.name())
    }
}

/// Represents a multi-target expression.
/// 
/// ```ruby
/// a, (b, c) = 1, 2, 3
///    ^^^^^^
/// ```
/// 
/// This can be a part of `MultiWriteNode` as above, or the target of a `for` loop
/// 
/// ```ruby
/// for a, b in [[1, 2], [3, 4]]
///     ^^^^
/// ```
pub struct MultiTargetNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_multi_target_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_multi_target_node_t>
}

impl<'pr> MultiTargetNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MultiTargetNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `lefts` param
    #[must_use]
    pub fn lefts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).lefts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `rest` param
    #[must_use]
    pub fn rest(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).rest };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `rights` param
    #[must_use]
    pub fn rights(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).rights };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for MultiTargetNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiTargetNode({:?}, {:?}, {:?}, {:?}, {:?})", self.lefts(), self.rest(), self.rights(), self.lparen_loc(), self.rparen_loc())
    }
}

/// Represents a write to a multi-target expression.
/// 
/// ```ruby
/// a, b, c = 1, 2, 3
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct MultiWriteNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_multi_write_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_multi_write_node_t>
}

impl<'pr> MultiWriteNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::MultiWriteNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `lefts` param
    #[must_use]
    pub fn lefts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).lefts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `rest` param
    #[must_use]
    pub fn rest(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).rest };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `rights` param
    #[must_use]
    pub fn rights(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).rights };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for MultiWriteNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiWriteNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.lefts(), self.rest(), self.rights(), self.lparen_loc(), self.rparen_loc(), self.operator_loc(), self.value())
    }
}

/// Represents the use of the `next` keyword.
/// 
/// ```ruby
/// next 1
/// ^^^^^^
/// ```
pub struct NextNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_next_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_next_node_t>
}

impl<'pr> NextNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::NextNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for NextNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NextNode({:?}, {:?})", self.arguments(), self.keyword_loc())
    }
}

/// Represents the use of the `nil` keyword.
/// 
/// ```ruby
/// nil
/// ^^^
/// ```
pub struct NilNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_nil_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_nil_node_t>
}

impl<'pr> NilNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::NilNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for NilNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NilNode()")
    }
}

/// Represents the use of `**nil` inside method arguments.
/// 
/// ```ruby
/// def a(**nil)
///       ^^^^^
/// end
/// ```
pub struct NoKeywordsParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_no_keywords_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_no_keywords_parameter_node_t>
}

impl<'pr> NoKeywordsParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::NoKeywordsParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for NoKeywordsParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoKeywordsParameterNode({:?}, {:?})", self.operator_loc(), self.keyword_loc())
    }
}

/// Represents an implicit set of parameters through the use of numbered parameters within a block or lambda.
/// 
/// ```ruby
/// -> { _1 + _2 }
/// ^^^^^^^^^^^^^^
/// ```
pub struct NumberedParametersNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_numbered_parameters_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_numbered_parameters_node_t>
}

impl<'pr> NumberedParametersNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::NumberedParametersNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `maximum` param
    #[must_use]
    pub fn maximum(&self) -> u8 {
        unsafe { (*self.pointer).maximum }
    }
}

impl std::fmt::Debug for NumberedParametersNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumberedParametersNode({:?})", self.maximum())
    }
}

/// Represents reading a numbered reference to a capture in the previous match.
/// 
/// ```ruby
/// $1
/// ^^
/// ```
pub struct NumberedReferenceReadNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_numbered_reference_read_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_numbered_reference_read_node_t>
}

impl<'pr> NumberedReferenceReadNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::NumberedReferenceReadNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `number` param
    #[must_use]
    pub fn number(&self) -> u32 {
        unsafe { (*self.pointer).number }
    }
}

impl std::fmt::Debug for NumberedReferenceReadNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumberedReferenceReadNode({:?})", self.number())
    }
}

/// Represents an optional keyword parameter to a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(b: 1)
///       ^^^^
/// end
/// ```
pub struct OptionalKeywordParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_optional_keyword_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_optional_keyword_parameter_node_t>
}

impl<'pr> OptionalKeywordParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::OptionalKeywordParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for OptionalKeywordParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptionalKeywordParameterNode({:?}, {:?}, {:?})", self.name(), self.name_loc(), self.value())
    }
}

/// Represents an optional parameter to a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(b = 1)
///       ^^^^^
/// end
/// ```
pub struct OptionalParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_optional_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_optional_parameter_node_t>
}

impl<'pr> OptionalParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::OptionalParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `value` param
    #[must_use]
    pub fn value(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).value };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for OptionalParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptionalParameterNode({:?}, {:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc(), self.value())
    }
}

/// Represents the use of the `||` operator or the `or` keyword.
/// 
/// ```ruby
/// left or right
/// ^^^^^^^^^^^^^
/// ```
pub struct OrNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_or_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_or_node_t>
}

impl<'pr> OrNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::OrNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `left` param
    #[must_use]
    pub fn left(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).left };
        Node::new(self.parser, node)
    }

    /// Returns the `right` param
    #[must_use]
    pub fn right(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).right };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for OrNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrNode({:?}, {:?}, {:?})", self.left(), self.right(), self.operator_loc())
    }
}

/// Represents the list of parameters on a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(b, c, d)
///       ^^^^^^^
/// end
/// ```
pub struct ParametersNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_parameters_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_parameters_node_t>
}

impl<'pr> ParametersNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ParametersNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `requireds` param
    #[must_use]
    pub fn requireds(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).requireds };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `optionals` param
    #[must_use]
    pub fn optionals(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).optionals };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `rest` param
    #[must_use]
    pub fn rest(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).rest };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `posts` param
    #[must_use]
    pub fn posts(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).posts };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `keywords` param
    #[must_use]
    pub fn keywords(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).keywords };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `keyword_rest` param
    #[must_use]
    pub fn keyword_rest(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).keyword_rest };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<BlockParameterNode<'pr>> {
        let node: *mut pm_block_parameter_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(BlockParameterNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for ParametersNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParametersNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.requireds(), self.optionals(), self.rest(), self.posts(), self.keywords(), self.keyword_rest(), self.block())
    }
}

/// Represents a parenthesized expression
/// 
/// ```ruby
/// (10 + 34)
/// ^^^^^^^^^
/// ```
pub struct ParenthesesNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_parentheses_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_parentheses_node_t>
}

impl<'pr> ParenthesesNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ParenthesesNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// parentheses that contain multiple potentially void statements
    #[must_use]
    pub fn is_multiple_statements(&self) -> bool {
        (self.flags() & PM_PARENTHESES_NODE_FLAGS_MULTIPLE_STATEMENTS) != 0
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for ParenthesesNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParenthesesNode({:?}, {:?}, {:?})", self.body(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents the use of the `^` operator for pinning an expression in a pattern matching expression.
/// 
/// ```ruby
/// foo in ^(bar)
///        ^^^^^^
/// ```
pub struct PinnedExpressionNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_pinned_expression_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_pinned_expression_node_t>
}

impl<'pr> PinnedExpressionNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::PinnedExpressionNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `expression` param
    #[must_use]
    pub fn expression(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).expression };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for PinnedExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PinnedExpressionNode({:?}, {:?}, {:?}, {:?})", self.expression(), self.operator_loc(), self.lparen_loc(), self.rparen_loc())
    }
}

/// Represents the use of the `^` operator for pinning a variable in a pattern matching expression.
/// 
/// ```ruby
/// foo in ^bar
///        ^^^^
/// ```
pub struct PinnedVariableNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_pinned_variable_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_pinned_variable_node_t>
}

impl<'pr> PinnedVariableNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::PinnedVariableNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `variable` param
    #[must_use]
    pub fn variable(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).variable };
        Node::new(self.parser, node)
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for PinnedVariableNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PinnedVariableNode({:?}, {:?})", self.variable(), self.operator_loc())
    }
}

/// Represents the use of the `END` keyword.
/// 
/// ```ruby
/// END { foo }
/// ^^^^^^^^^^^
/// ```
pub struct PostExecutionNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_post_execution_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_post_execution_node_t>
}

impl<'pr> PostExecutionNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::PostExecutionNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for PostExecutionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostExecutionNode({:?}, {:?}, {:?}, {:?})", self.statements(), self.keyword_loc(), self.opening_loc(), self.closing_loc())
    }
}

/// Represents the use of the `BEGIN` keyword.
/// 
/// ```ruby
/// BEGIN { foo }
/// ^^^^^^^^^^^^^
/// ```
pub struct PreExecutionNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_pre_execution_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_pre_execution_node_t>
}

impl<'pr> PreExecutionNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::PreExecutionNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for PreExecutionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PreExecutionNode({:?}, {:?}, {:?}, {:?})", self.statements(), self.keyword_loc(), self.opening_loc(), self.closing_loc())
    }
}

/// The top level node of any parse tree.
pub struct ProgramNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_program_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_program_node_t>
}

impl<'pr> ProgramNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ProgramNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> StatementsNode<'pr> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        StatementsNode { parser: self.parser, pointer: node, marker: PhantomData }
    }
}

impl std::fmt::Debug for ProgramNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProgramNode({:?}, {:?})", self.locals(), self.statements())
    }
}

/// Represents the use of the `..` or `...` operators.
/// 
/// ```ruby
/// 1..2
/// ^^^^
/// ```
/// 
/// ```ruby
/// c if a =~ /left/ ... b =~ /right/
///      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct RangeNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_range_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_range_node_t>
}

impl<'pr> RangeNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RangeNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// ... operator
    #[must_use]
    pub fn is_exclude_end(&self) -> bool {
        (self.flags() & PM_RANGE_FLAGS_EXCLUDE_END) != 0
    }

    /// Returns the `left` param
    #[must_use]
    pub fn left(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).left };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `right` param
    #[must_use]
    pub fn right(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).right };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for RangeNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RangeNode({:?}, {:?}, {:?})", self.left(), self.right(), self.operator_loc())
    }
}

/// Represents a rational number literal.
/// 
/// ```ruby
/// 1.0r
/// ^^^^
/// ```
pub struct RationalNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_rational_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_rational_node_t>
}

impl<'pr> RationalNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RationalNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// 0b prefix
    #[must_use]
    pub fn is_binary(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_BINARY) != 0
    }

    /// 0d or no prefix
    #[must_use]
    pub fn is_decimal(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_DECIMAL) != 0
    }

    /// 0o or 0 prefix
    #[must_use]
    pub fn is_octal(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_OCTAL) != 0
    }

    /// 0x prefix
    #[must_use]
    pub fn is_hexadecimal(&self) -> bool {
        (self.flags() & PM_INTEGER_BASE_FLAGS_HEXADECIMAL) != 0
    }

    /// Returns the `numerator` param
    #[must_use]
    pub fn numerator(&self) -> Integer<'pr> {
        Integer::new(unsafe { &raw const(*self.pointer).numerator })
    }

    /// Returns the `denominator` param
    #[must_use]
    pub fn denominator(&self) -> Integer<'pr> {
        Integer::new(unsafe { &raw const(*self.pointer).denominator })
    }
}

impl std::fmt::Debug for RationalNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RationalNode({:?}, {:?})", self.numerator(), self.denominator())
    }
}

/// Represents the use of the `redo` keyword.
/// 
/// ```ruby
/// redo
/// ^^^^
/// ```
pub struct RedoNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_redo_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_redo_node_t>
}

impl<'pr> RedoNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RedoNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for RedoNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RedoNode()")
    }
}

/// Represents a regular expression literal with no interpolation.
/// 
/// ```ruby
/// /foo/i
/// ^^^^^^
/// ```
pub struct RegularExpressionNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_regular_expression_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_regular_expression_node_t>
}

impl<'pr> RegularExpressionNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RegularExpressionNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// i - ignores the case of characters when matching
    #[must_use]
    pub fn is_ignore_case(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_IGNORE_CASE) != 0
    }

    /// x - ignores whitespace and allows comments in regular expressions
    #[must_use]
    pub fn is_extended(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EXTENDED) != 0
    }

    /// m - allows $ to match the end of lines within strings
    #[must_use]
    pub fn is_multi_line(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_MULTI_LINE) != 0
    }

    /// o - only interpolates values into the regular expression once
    #[must_use]
    pub fn is_once(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ONCE) != 0
    }

    /// e - forces the EUC-JP encoding
    #[must_use]
    pub fn is_euc_jp(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_EUC_JP) != 0
    }

    /// n - forces the ASCII-8BIT encoding
    #[must_use]
    pub fn is_ascii_8bit(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_ASCII_8BIT) != 0
    }

    /// s - forces the Windows-31J encoding
    #[must_use]
    pub fn is_windows_31j(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_WINDOWS_31J) != 0
    }

    /// u - forces the UTF-8 encoding
    #[must_use]
    pub fn is_utf_8(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_UTF_8) != 0
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// internal bytes forced the encoding to US-ASCII
    #[must_use]
    pub fn is_forced_us_ascii_encoding(&self) -> bool {
        (self.flags() & PM_REGULAR_EXPRESSION_FLAGS_FORCED_US_ASCII_ENCODING) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `content_loc` param
    #[must_use]
    pub fn content_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).content_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `unescaped` param
    #[must_use]
    pub fn unescaped(&self) -> &[u8] {
        unsafe {
            let source = (*self.pointer).unescaped.source;
            if source.is_null() {
                return &[];
            }
            let length = (*self.pointer).unescaped.length;
            std::slice::from_raw_parts(source, length)
        }
    }
}

impl std::fmt::Debug for RegularExpressionNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RegularExpressionNode({:?}, {:?}, {:?}, {:?})", self.opening_loc(), self.content_loc(), self.closing_loc(), self.unescaped())
    }
}

/// Represents a required keyword parameter to a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(b: )
///       ^^
/// end
/// ```
pub struct RequiredKeywordParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_required_keyword_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_required_keyword_parameter_node_t>
}

impl<'pr> RequiredKeywordParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RequiredKeywordParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for RequiredKeywordParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequiredKeywordParameterNode({:?}, {:?})", self.name(), self.name_loc())
    }
}

/// Represents a required parameter to a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(b)
///       ^
/// end
/// ```
pub struct RequiredParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_required_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_required_parameter_node_t>
}

impl<'pr> RequiredParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RequiredParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> ConstantId<'pr> {
        ConstantId::new(self.parser, unsafe { (*self.pointer).name })
    }
}

impl std::fmt::Debug for RequiredParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequiredParameterNode({:?})", self.name())
    }
}

/// Represents an expression modified with a rescue.
/// 
/// ```ruby
/// foo rescue nil
/// ^^^^^^^^^^^^^^
/// ```
pub struct RescueModifierNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_rescue_modifier_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_rescue_modifier_node_t>
}

impl<'pr> RescueModifierNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RescueModifierNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `expression` param
    #[must_use]
    pub fn expression(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).expression };
        Node::new(self.parser, node)
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `rescue_expression` param
    #[must_use]
    pub fn rescue_expression(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).rescue_expression };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for RescueModifierNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RescueModifierNode({:?}, {:?}, {:?})", self.expression(), self.keyword_loc(), self.rescue_expression())
    }
}

/// Represents a rescue statement.
/// 
/// ```ruby
/// begin
/// rescue Foo, *splat, Bar => ex
///   foo
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// end
/// ```
/// 
/// `Foo, *splat, Bar` are in the `exceptions` field. `ex` is in the `reference` field.
pub struct RescueNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_rescue_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_rescue_node_t>
}

impl<'pr> RescueNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RescueNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `exceptions` param
    #[must_use]
    pub fn exceptions(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).exceptions };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `reference` param
    #[must_use]
    pub fn reference(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).reference };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `then_keyword_loc` param
    #[must_use]
    pub fn then_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).then_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `subsequent` param
    #[must_use]
    pub fn subsequent(&self) -> Option<RescueNode<'pr>> {
        let node: *mut pm_rescue_node_t = unsafe { (*self.pointer).subsequent };
        if node.is_null() {
            None
        } else {
            Some(RescueNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for RescueNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RescueNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.exceptions(), self.operator_loc(), self.reference(), self.then_keyword_loc(), self.statements(), self.subsequent())
    }
}

/// Represents a rest parameter to a method, block, or lambda definition.
/// 
/// ```ruby
/// def a(*b)
///       ^^
/// end
/// ```
pub struct RestParameterNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_rest_parameter_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_rest_parameter_node_t>
}

impl<'pr> RestParameterNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RestParameterNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a parameter name that has been repeated in the method signature
    #[must_use]
    pub fn is_repeated_parameter(&self) -> bool {
        (self.flags() & PM_PARAMETER_FLAGS_REPEATED_PARAMETER) != 0
    }

    /// Returns the `name` param
    #[must_use]
    pub fn name(&self) -> Option<ConstantId<'pr>> {
        let id = unsafe { (*self.pointer).name };
        if id == 0 {
            None
        } else {
            Some(ConstantId::new(self.parser, id))
        }
    }

    /// Returns the `name_loc` param
    #[must_use]
    pub fn name_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).name_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for RestParameterNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RestParameterNode({:?}, {:?}, {:?})", self.name(), self.name_loc(), self.operator_loc())
    }
}

/// Represents the use of the `retry` keyword.
/// 
/// ```ruby
/// retry
/// ^^^^^
/// ```
pub struct RetryNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_retry_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_retry_node_t>
}

impl<'pr> RetryNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::RetryNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for RetryNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RetryNode()")
    }
}

/// Represents the use of the `return` keyword.
/// 
/// ```ruby
/// return 1
/// ^^^^^^^^
/// ```
pub struct ReturnNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_return_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_return_node_t>
}

impl<'pr> ReturnNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ReturnNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for ReturnNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReturnNode({:?}, {:?})", self.keyword_loc(), self.arguments())
    }
}

/// Represents the `self` keyword.
/// 
/// ```ruby
/// self
/// ^^^^
/// ```
pub struct SelfNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_self_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_self_node_t>
}

impl<'pr> SelfNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SelfNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for SelfNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelfNode()")
    }
}

/// This node wraps a constant write to indicate that when the value is written, it should have its shareability state modified.
/// 
/// ```ruby
/// # shareable_constant_value: literal
/// C = { a: 1 }
/// ^^^^^^^^^^^^
/// ```
pub struct ShareableConstantNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_shareable_constant_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_shareable_constant_node_t>
}

impl<'pr> ShareableConstantNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::ShareableConstantNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// constant writes that should be modified with shareable constant value literal
    #[must_use]
    pub fn is_literal(&self) -> bool {
        (self.flags() & PM_SHAREABLE_CONSTANT_NODE_FLAGS_LITERAL) != 0
    }

    /// constant writes that should be modified with shareable constant value experimental everything
    #[must_use]
    pub fn is_experimental_everything(&self) -> bool {
        (self.flags() & PM_SHAREABLE_CONSTANT_NODE_FLAGS_EXPERIMENTAL_EVERYTHING) != 0
    }

    /// constant writes that should be modified with shareable constant value experimental copy
    #[must_use]
    pub fn is_experimental_copy(&self) -> bool {
        (self.flags() & PM_SHAREABLE_CONSTANT_NODE_FLAGS_EXPERIMENTAL_COPY) != 0
    }

    /// Returns the `write` param
    #[must_use]
    pub fn write(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).write };
        Node::new(self.parser, node)
    }
}

impl std::fmt::Debug for ShareableConstantNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShareableConstantNode({:?})", self.write())
    }
}

/// Represents a singleton class declaration involving the `class` keyword.
/// 
/// ```ruby
/// class << self end
/// ^^^^^^^^^^^^^^^^^
/// ```
pub struct SingletonClassNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_singleton_class_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_singleton_class_node_t>
}

impl<'pr> SingletonClassNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SingletonClassNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `locals` param
    #[must_use]
    pub fn locals(&self) -> ConstantList<'pr> {
        let pointer: *mut pm_constant_id_list_t = unsafe { &raw mut (*self.pointer).locals };
        ConstantList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `class_keyword_loc` param
    #[must_use]
    pub fn class_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).class_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `expression` param
    #[must_use]
    pub fn expression(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).expression };
        Node::new(self.parser, node)
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).body };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for SingletonClassNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SingletonClassNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.locals(), self.class_keyword_loc(), self.operator_loc(), self.expression(), self.body(), self.end_keyword_loc())
    }
}

/// Represents the use of the `__ENCODING__` keyword.
/// 
/// ```ruby
/// __ENCODING__
/// ^^^^^^^^^^^^
/// ```
pub struct SourceEncodingNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_source_encoding_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_source_encoding_node_t>
}

impl<'pr> SourceEncodingNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SourceEncodingNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for SourceEncodingNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceEncodingNode()")
    }
}

/// Represents the use of the `__FILE__` keyword.
/// 
/// ```ruby
/// __FILE__
/// ^^^^^^^^
/// ```
pub struct SourceFileNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_source_file_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_source_file_node_t>
}

impl<'pr> SourceFileNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SourceFileNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// frozen by virtue of a `frozen_string_literal: true` comment or `--enable-frozen-string-literal`
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_FROZEN) != 0
    }

    /// mutable by virtue of a `frozen_string_literal: false` comment or `--disable-frozen-string-literal`
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_MUTABLE) != 0
    }

    /// Returns the `filepath` param
    #[must_use]
    pub fn filepath(&self) -> &[u8] {
        unsafe {
            let source = (*self.pointer).filepath.source;
            if source.is_null() {
                return &[];
            }
            let length = (*self.pointer).filepath.length;
            std::slice::from_raw_parts(source, length)
        }
    }
}

impl std::fmt::Debug for SourceFileNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceFileNode({:?})", self.filepath())
    }
}

/// Represents the use of the `__LINE__` keyword.
/// 
/// ```ruby
/// __LINE__
/// ^^^^^^^^
/// ```
pub struct SourceLineNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_source_line_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_source_line_node_t>
}

impl<'pr> SourceLineNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SourceLineNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for SourceLineNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceLineNode()")
    }
}

/// Represents the use of the splat operator.
/// 
/// ```ruby
/// [*a]
///  ^^
/// ```
pub struct SplatNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_splat_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_splat_node_t>
}

impl<'pr> SplatNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SplatNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `operator_loc` param
    #[must_use]
    pub fn operator_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).operator_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `expression` param
    #[must_use]
    pub fn expression(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).expression };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }
}

impl std::fmt::Debug for SplatNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SplatNode({:?}, {:?})", self.operator_loc(), self.expression())
    }
}

/// Represents a set of statements contained within some scope.
/// 
/// ```ruby
/// foo; bar; baz
/// ^^^^^^^^^^^^^
/// ```
pub struct StatementsNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_statements_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_statements_node_t>
}

impl<'pr> StatementsNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::StatementsNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `body` param
    #[must_use]
    pub fn body(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).body };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }
}

impl std::fmt::Debug for StatementsNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatementsNode({:?})", self.body())
    }
}

/// Represents a string literal, a string contained within a `%w` list, or plain string content within an interpolated string.
/// 
/// ```ruby
/// "foo"
/// ^^^^^
/// ```
/// 
/// ```ruby
/// %w[foo]
///    ^^^
/// ```
/// 
/// ```ruby
/// "foo #{bar} baz"
///  ^^^^      ^^^^
/// ```
pub struct StringNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_string_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_string_node_t>
}

impl<'pr> StringNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::StringNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// frozen by virtue of a `frozen_string_literal: true` comment or `--enable-frozen-string-literal`
    #[must_use]
    pub fn is_frozen(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_FROZEN) != 0
    }

    /// mutable by virtue of a `frozen_string_literal: false` comment or `--disable-frozen-string-literal`
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        (self.flags() & PM_STRING_FLAGS_MUTABLE) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `content_loc` param
    #[must_use]
    pub fn content_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).content_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `unescaped` param
    #[must_use]
    pub fn unescaped(&self) -> &[u8] {
        unsafe {
            let source = (*self.pointer).unescaped.source;
            if source.is_null() {
                return &[];
            }
            let length = (*self.pointer).unescaped.length;
            std::slice::from_raw_parts(source, length)
        }
    }
}

impl std::fmt::Debug for StringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringNode({:?}, {:?}, {:?}, {:?})", self.opening_loc(), self.content_loc(), self.closing_loc(), self.unescaped())
    }
}

/// Represents the use of the `super` keyword with parentheses or arguments.
/// 
/// ```ruby
/// super()
/// ^^^^^^^
/// ```
/// 
/// ```ruby
/// super foo, bar
/// ^^^^^^^^^^^^^^
/// ```
/// 
/// If no arguments are provided (except for a block), it would be a `ForwardingSuperNode` instead.
pub struct SuperNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_super_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_super_node_t>
}

impl<'pr> SuperNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SuperNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `block` param
    #[must_use]
    pub fn block(&self) -> Option<Node<'pr>> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).block };
        if node.is_null() {
            None
        } else {
            Some(Node::new(self.parser, node))
        }
    }
}

impl std::fmt::Debug for SuperNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperNode({:?}, {:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.lparen_loc(), self.arguments(), self.rparen_loc(), self.block())
    }
}

/// Represents a symbol literal or a symbol contained within a `%i` list.
/// 
/// ```ruby
/// :foo
/// ^^^^
/// ```
/// 
/// ```ruby
/// %i[foo]
///    ^^^
/// ```
pub struct SymbolNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_symbol_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_symbol_node_t>
}

impl<'pr> SymbolNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::SymbolNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_SYMBOL_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_SYMBOL_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// internal bytes forced the encoding to US-ASCII
    #[must_use]
    pub fn is_forced_us_ascii_encoding(&self) -> bool {
        (self.flags() & PM_SYMBOL_FLAGS_FORCED_US_ASCII_ENCODING) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `value_loc` param
    #[must_use]
    pub fn value_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).value_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `unescaped` param
    #[must_use]
    pub fn unescaped(&self) -> &[u8] {
        unsafe {
            let source = (*self.pointer).unescaped.source;
            if source.is_null() {
                return &[];
            }
            let length = (*self.pointer).unescaped.length;
            std::slice::from_raw_parts(source, length)
        }
    }
}

impl std::fmt::Debug for SymbolNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SymbolNode({:?}, {:?}, {:?}, {:?})", self.opening_loc(), self.value_loc(), self.closing_loc(), self.unescaped())
    }
}

/// Represents the use of the literal `true` keyword.
/// 
/// ```ruby
/// true
/// ^^^^
/// ```
pub struct TrueNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_true_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_true_node_t>
}

impl<'pr> TrueNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::TrueNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }
}

impl std::fmt::Debug for TrueNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrueNode()")
    }
}

/// Represents the use of the `undef` keyword.
/// 
/// ```ruby
/// undef :foo, :bar, :baz
/// ^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct UndefNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_undef_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_undef_node_t>
}

impl<'pr> UndefNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::UndefNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `names` param
    #[must_use]
    pub fn names(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).names };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }
}

impl std::fmt::Debug for UndefNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UndefNode({:?}, {:?})", self.names(), self.keyword_loc())
    }
}

/// Represents the use of the `unless` keyword, either in the block form or the modifier form.
/// 
/// ```ruby
/// bar unless foo
/// ^^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// unless foo then bar end
/// ^^^^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct UnlessNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_unless_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_unless_node_t>
}

impl<'pr> UnlessNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::UnlessNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `predicate` param
    #[must_use]
    pub fn predicate(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).predicate };
        Node::new(self.parser, node)
    }

    /// Returns the `then_keyword_loc` param
    #[must_use]
    pub fn then_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).then_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `else_clause` param
    #[must_use]
    pub fn else_clause(&self) -> Option<ElseNode<'pr>> {
        let node: *mut pm_else_node_t = unsafe { (*self.pointer).else_clause };
        if node.is_null() {
            None
        } else {
            Some(ElseNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `end_keyword_loc` param
    #[must_use]
    pub fn end_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).end_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for UnlessNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnlessNode({:?}, {:?}, {:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.predicate(), self.then_keyword_loc(), self.statements(), self.else_clause(), self.end_keyword_loc())
    }
}

/// Represents the use of the `until` keyword, either in the block form or the modifier form.
/// 
/// ```ruby
/// bar until foo
/// ^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// until foo do bar end
/// ^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct UntilNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_until_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_until_node_t>
}

impl<'pr> UntilNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::UntilNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a loop after a begin statement, so the body is executed first before the condition
    #[must_use]
    pub fn is_begin_modifier(&self) -> bool {
        (self.flags() & PM_LOOP_FLAGS_BEGIN_MODIFIER) != 0
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `do_keyword_loc` param
    #[must_use]
    pub fn do_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).do_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `predicate` param
    #[must_use]
    pub fn predicate(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).predicate };
        Node::new(self.parser, node)
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for UntilNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UntilNode({:?}, {:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.do_keyword_loc(), self.closing_loc(), self.predicate(), self.statements())
    }
}

/// Represents the use of the `when` keyword within a case statement.
/// 
/// ```ruby
/// case true
/// when true
/// ^^^^^^^^^
/// end
/// ```
pub struct WhenNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_when_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_when_node_t>
}

impl<'pr> WhenNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::WhenNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `conditions` param
    #[must_use]
    pub fn conditions(&self) -> NodeList<'pr> {
        let pointer: *mut pm_node_list = unsafe { &raw mut (*self.pointer).conditions };
        NodeList { parser: self.parser, pointer: unsafe { NonNull::new_unchecked(pointer) }, marker: PhantomData }
    }

    /// Returns the `then_keyword_loc` param
    #[must_use]
    pub fn then_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).then_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for WhenNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WhenNode({:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.conditions(), self.then_keyword_loc(), self.statements())
    }
}

/// Represents the use of the `while` keyword, either in the block form or the modifier form.
/// 
/// ```ruby
/// bar while foo
/// ^^^^^^^^^^^^^
/// ```
/// 
/// ```ruby
/// while foo do bar end
/// ^^^^^^^^^^^^^^^^^^^^
/// ```
pub struct WhileNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_while_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_while_node_t>
}

impl<'pr> WhileNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::WhileNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// a loop after a begin statement, so the body is executed first before the condition
    #[must_use]
    pub fn is_begin_modifier(&self) -> bool {
        (self.flags() & PM_LOOP_FLAGS_BEGIN_MODIFIER) != 0
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `do_keyword_loc` param
    #[must_use]
    pub fn do_keyword_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).do_keyword_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `predicate` param
    #[must_use]
    pub fn predicate(&self) -> Node<'pr> {
        let node: *mut pm_node_t = unsafe { (*self.pointer).predicate };
        Node::new(self.parser, node)
    }

    /// Returns the `statements` param
    #[must_use]
    pub fn statements(&self) -> Option<StatementsNode<'pr>> {
        let node: *mut pm_statements_node_t = unsafe { (*self.pointer).statements };
        if node.is_null() {
            None
        } else {
            Some(StatementsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }
}

impl std::fmt::Debug for WhileNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WhileNode({:?}, {:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.do_keyword_loc(), self.closing_loc(), self.predicate(), self.statements())
    }
}

/// Represents an xstring literal with no interpolation.
/// 
/// ```ruby
/// `foo`
/// ^^^^^
/// ```
pub struct XStringNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_x_string_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_x_string_node_t>
}

impl<'pr> XStringNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::XStringNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// internal bytes forced the encoding to UTF-8
    #[must_use]
    pub fn is_forced_utf8_encoding(&self) -> bool {
        (self.flags() & PM_ENCODING_FLAGS_FORCED_UTF8_ENCODING) != 0
    }

    /// internal bytes forced the encoding to binary
    #[must_use]
    pub fn is_forced_binary_encoding(&self) -> bool {
        (self.flags() & PM_ENCODING_FLAGS_FORCED_BINARY_ENCODING) != 0
    }

    /// Returns the `opening_loc` param
    #[must_use]
    pub fn opening_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).opening_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `content_loc` param
    #[must_use]
    pub fn content_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).content_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `closing_loc` param
    #[must_use]
    pub fn closing_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).closing_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `unescaped` param
    #[must_use]
    pub fn unescaped(&self) -> &[u8] {
        unsafe {
            let source = (*self.pointer).unescaped.source;
            if source.is_null() {
                return &[];
            }
            let length = (*self.pointer).unescaped.length;
            std::slice::from_raw_parts(source, length)
        }
    }
}

impl std::fmt::Debug for XStringNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XStringNode({:?}, {:?}, {:?}, {:?})", self.opening_loc(), self.content_loc(), self.closing_loc(), self.unescaped())
    }
}

/// Represents the use of the `yield` keyword.
/// 
/// ```ruby
/// yield 1
/// ^^^^^^^
/// ```
pub struct YieldNode<'pr> {
    /// The pointer to the parser this node came from.
    parser: NonNull<pm_parser_t>,

    /// The raw pointer to the node allocated by prism.
    pointer: *mut pm_yield_node_t,

    /// The marker to indicate the lifetime of the pointer.
    marker: PhantomData<&'pr mut pm_yield_node_t>
}

impl<'pr> YieldNode<'pr> {
    /// Converts this node to a generic node.
    #[must_use]
    pub const fn as_node(&self) -> Node<'pr> {
        Node::YieldNode { parser: self.parser, pointer: self.pointer, marker: PhantomData }
    }

    /// Returns the location of this node.
    #[must_use]
    pub fn location(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).base.location };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the flags of this node.
    #[must_use]
    pub fn flags(&self) -> pm_node_flags_t {
        unsafe { (*self.pointer).base.flags }
    }

    /// Returns the `keyword_loc` param
    #[must_use]
    pub fn keyword_loc(&self) -> Location<'pr> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).keyword_loc };
        Location::new(self.parser, unsafe { &(*pointer) })
    }

    /// Returns the `lparen_loc` param
    #[must_use]
    pub fn lparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).lparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }

    /// Returns the `arguments` param
    #[must_use]
    pub fn arguments(&self) -> Option<ArgumentsNode<'pr>> {
        let node: *mut pm_arguments_node_t = unsafe { (*self.pointer).arguments };
        if node.is_null() {
            None
        } else {
            Some(ArgumentsNode { parser: self.parser, pointer: node, marker: PhantomData })
        }
    }

    /// Returns the `rparen_loc` param
    #[must_use]
    pub fn rparen_loc(&self) -> Option<Location<'pr>> {
        let pointer: *mut pm_location_t = unsafe { &raw mut (*self.pointer).rparen_loc };
        let start = unsafe { (*pointer).start };
        if start.is_null() {
            None
        } else {
            Some(Location::new(self.parser, unsafe { &(*pointer) }))
        }
    }
}

impl std::fmt::Debug for YieldNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "YieldNode({:?}, {:?}, {:?}, {:?})", self.keyword_loc(), self.lparen_loc(), self.arguments(), self.rparen_loc())
    }
}

/// A trait for visiting the AST.
pub trait Visit<'pr> {
   /// Called prior to visiting a node with potential child nodes.
   fn visit_branch_node_enter(&mut self, _node: Node<'pr>) {
   }

   /// Called after visiting a node with potential child nodes.
   fn visit_branch_node_leave(&mut self) {
   }

   /// Called prior to visiting a node that cannot have child nodes.
   fn visit_leaf_node_enter(&mut self, _node: Node<'pr>) {
   }

   /// Called after visiting a node that cannot have child nodes.
   fn visit_leaf_node_leave(&mut self) {
   }

   /// Visits a node.
   fn visit(&mut self, node: &Node<'pr>) {
       match node {
           Node::AliasGlobalVariableNode { parser, pointer, marker } => {
               let concrete = AliasGlobalVariableNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_alias_global_variable_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::AliasMethodNode { parser, pointer, marker } => {
               let concrete = AliasMethodNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_alias_method_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::AlternationPatternNode { parser, pointer, marker } => {
               let concrete = AlternationPatternNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_alternation_pattern_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::AndNode { parser, pointer, marker } => {
               let concrete = AndNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_and_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ArgumentsNode { parser, pointer, marker } => {
               let concrete = ArgumentsNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_arguments_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ArrayNode { parser, pointer, marker } => {
               let concrete = ArrayNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_array_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ArrayPatternNode { parser, pointer, marker } => {
               let concrete = ArrayPatternNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_array_pattern_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::AssocNode { parser, pointer, marker } => {
               let concrete = AssocNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_assoc_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::AssocSplatNode { parser, pointer, marker } => {
               let concrete = AssocSplatNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_assoc_splat_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::BackReferenceReadNode { parser, pointer, marker } => {
               let concrete = BackReferenceReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_back_reference_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::BeginNode { parser, pointer, marker } => {
               let concrete = BeginNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_begin_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::BlockArgumentNode { parser, pointer, marker } => {
               let concrete = BlockArgumentNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_block_argument_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::BlockLocalVariableNode { parser, pointer, marker } => {
               let concrete = BlockLocalVariableNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_block_local_variable_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::BlockNode { parser, pointer, marker } => {
               let concrete = BlockNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_block_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::BlockParameterNode { parser, pointer, marker } => {
               let concrete = BlockParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_block_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::BlockParametersNode { parser, pointer, marker } => {
               let concrete = BlockParametersNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_block_parameters_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::BreakNode { parser, pointer, marker } => {
               let concrete = BreakNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_break_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CallAndWriteNode { parser, pointer, marker } => {
               let concrete = CallAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_call_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CallNode { parser, pointer, marker } => {
               let concrete = CallNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_call_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CallOperatorWriteNode { parser, pointer, marker } => {
               let concrete = CallOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_call_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CallOrWriteNode { parser, pointer, marker } => {
               let concrete = CallOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_call_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CallTargetNode { parser, pointer, marker } => {
               let concrete = CallTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_call_target_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CapturePatternNode { parser, pointer, marker } => {
               let concrete = CapturePatternNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_capture_pattern_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CaseMatchNode { parser, pointer, marker } => {
               let concrete = CaseMatchNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_case_match_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::CaseNode { parser, pointer, marker } => {
               let concrete = CaseNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_case_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ClassNode { parser, pointer, marker } => {
               let concrete = ClassNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_class_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ClassVariableAndWriteNode { parser, pointer, marker } => {
               let concrete = ClassVariableAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_class_variable_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ClassVariableOperatorWriteNode { parser, pointer, marker } => {
               let concrete = ClassVariableOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_class_variable_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ClassVariableOrWriteNode { parser, pointer, marker } => {
               let concrete = ClassVariableOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_class_variable_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ClassVariableReadNode { parser, pointer, marker } => {
               let concrete = ClassVariableReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_class_variable_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ClassVariableTargetNode { parser, pointer, marker } => {
               let concrete = ClassVariableTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_class_variable_target_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ClassVariableWriteNode { parser, pointer, marker } => {
               let concrete = ClassVariableWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_class_variable_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantAndWriteNode { parser, pointer, marker } => {
               let concrete = ConstantAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantOperatorWriteNode { parser, pointer, marker } => {
               let concrete = ConstantOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantOrWriteNode { parser, pointer, marker } => {
               let concrete = ConstantOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantPathAndWriteNode { parser, pointer, marker } => {
               let concrete = ConstantPathAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_path_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantPathNode { parser, pointer, marker } => {
               let concrete = ConstantPathNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_path_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantPathOperatorWriteNode { parser, pointer, marker } => {
               let concrete = ConstantPathOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_path_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantPathOrWriteNode { parser, pointer, marker } => {
               let concrete = ConstantPathOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_path_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantPathTargetNode { parser, pointer, marker } => {
               let concrete = ConstantPathTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_path_target_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantPathWriteNode { parser, pointer, marker } => {
               let concrete = ConstantPathWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_path_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ConstantReadNode { parser, pointer, marker } => {
               let concrete = ConstantReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_constant_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ConstantTargetNode { parser, pointer, marker } => {
               let concrete = ConstantTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_constant_target_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ConstantWriteNode { parser, pointer, marker } => {
               let concrete = ConstantWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_constant_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::DefNode { parser, pointer, marker } => {
               let concrete = DefNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_def_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::DefinedNode { parser, pointer, marker } => {
               let concrete = DefinedNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_defined_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ElseNode { parser, pointer, marker } => {
               let concrete = ElseNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_else_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::EmbeddedStatementsNode { parser, pointer, marker } => {
               let concrete = EmbeddedStatementsNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_embedded_statements_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::EmbeddedVariableNode { parser, pointer, marker } => {
               let concrete = EmbeddedVariableNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_embedded_variable_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::EnsureNode { parser, pointer, marker } => {
               let concrete = EnsureNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_ensure_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::FalseNode { parser, pointer, marker } => {
               let concrete = FalseNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_false_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::FindPatternNode { parser, pointer, marker } => {
               let concrete = FindPatternNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_find_pattern_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::FlipFlopNode { parser, pointer, marker } => {
               let concrete = FlipFlopNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_flip_flop_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::FloatNode { parser, pointer, marker } => {
               let concrete = FloatNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_float_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ForNode { parser, pointer, marker } => {
               let concrete = ForNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_for_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ForwardingArgumentsNode { parser, pointer, marker } => {
               let concrete = ForwardingArgumentsNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_forwarding_arguments_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ForwardingParameterNode { parser, pointer, marker } => {
               let concrete = ForwardingParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_forwarding_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ForwardingSuperNode { parser, pointer, marker } => {
               let concrete = ForwardingSuperNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_forwarding_super_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::GlobalVariableAndWriteNode { parser, pointer, marker } => {
               let concrete = GlobalVariableAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_global_variable_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::GlobalVariableOperatorWriteNode { parser, pointer, marker } => {
               let concrete = GlobalVariableOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_global_variable_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::GlobalVariableOrWriteNode { parser, pointer, marker } => {
               let concrete = GlobalVariableOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_global_variable_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::GlobalVariableReadNode { parser, pointer, marker } => {
               let concrete = GlobalVariableReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_global_variable_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::GlobalVariableTargetNode { parser, pointer, marker } => {
               let concrete = GlobalVariableTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_global_variable_target_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::GlobalVariableWriteNode { parser, pointer, marker } => {
               let concrete = GlobalVariableWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_global_variable_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::HashNode { parser, pointer, marker } => {
               let concrete = HashNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_hash_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::HashPatternNode { parser, pointer, marker } => {
               let concrete = HashPatternNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_hash_pattern_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::IfNode { parser, pointer, marker } => {
               let concrete = IfNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_if_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ImaginaryNode { parser, pointer, marker } => {
               let concrete = ImaginaryNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_imaginary_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ImplicitNode { parser, pointer, marker } => {
               let concrete = ImplicitNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_implicit_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ImplicitRestNode { parser, pointer, marker } => {
               let concrete = ImplicitRestNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_implicit_rest_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::InNode { parser, pointer, marker } => {
               let concrete = InNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_in_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::IndexAndWriteNode { parser, pointer, marker } => {
               let concrete = IndexAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_index_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::IndexOperatorWriteNode { parser, pointer, marker } => {
               let concrete = IndexOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_index_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::IndexOrWriteNode { parser, pointer, marker } => {
               let concrete = IndexOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_index_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::IndexTargetNode { parser, pointer, marker } => {
               let concrete = IndexTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_index_target_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InstanceVariableAndWriteNode { parser, pointer, marker } => {
               let concrete = InstanceVariableAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_instance_variable_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InstanceVariableOperatorWriteNode { parser, pointer, marker } => {
               let concrete = InstanceVariableOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_instance_variable_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InstanceVariableOrWriteNode { parser, pointer, marker } => {
               let concrete = InstanceVariableOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_instance_variable_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InstanceVariableReadNode { parser, pointer, marker } => {
               let concrete = InstanceVariableReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_instance_variable_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::InstanceVariableTargetNode { parser, pointer, marker } => {
               let concrete = InstanceVariableTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_instance_variable_target_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::InstanceVariableWriteNode { parser, pointer, marker } => {
               let concrete = InstanceVariableWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_instance_variable_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::IntegerNode { parser, pointer, marker } => {
               let concrete = IntegerNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_integer_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::InterpolatedMatchLastLineNode { parser, pointer, marker } => {
               let concrete = InterpolatedMatchLastLineNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_interpolated_match_last_line_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InterpolatedRegularExpressionNode { parser, pointer, marker } => {
               let concrete = InterpolatedRegularExpressionNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_interpolated_regular_expression_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InterpolatedStringNode { parser, pointer, marker } => {
               let concrete = InterpolatedStringNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_interpolated_string_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InterpolatedSymbolNode { parser, pointer, marker } => {
               let concrete = InterpolatedSymbolNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_interpolated_symbol_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::InterpolatedXStringNode { parser, pointer, marker } => {
               let concrete = InterpolatedXStringNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_interpolated_x_string_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ItLocalVariableReadNode { parser, pointer, marker } => {
               let concrete = ItLocalVariableReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_it_local_variable_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ItParametersNode { parser, pointer, marker } => {
               let concrete = ItParametersNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_it_parameters_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::KeywordHashNode { parser, pointer, marker } => {
               let concrete = KeywordHashNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_keyword_hash_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::KeywordRestParameterNode { parser, pointer, marker } => {
               let concrete = KeywordRestParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_keyword_rest_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::LambdaNode { parser, pointer, marker } => {
               let concrete = LambdaNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_lambda_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::LocalVariableAndWriteNode { parser, pointer, marker } => {
               let concrete = LocalVariableAndWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_local_variable_and_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::LocalVariableOperatorWriteNode { parser, pointer, marker } => {
               let concrete = LocalVariableOperatorWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_local_variable_operator_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::LocalVariableOrWriteNode { parser, pointer, marker } => {
               let concrete = LocalVariableOrWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_local_variable_or_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::LocalVariableReadNode { parser, pointer, marker } => {
               let concrete = LocalVariableReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_local_variable_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::LocalVariableTargetNode { parser, pointer, marker } => {
               let concrete = LocalVariableTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_local_variable_target_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::LocalVariableWriteNode { parser, pointer, marker } => {
               let concrete = LocalVariableWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_local_variable_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::MatchLastLineNode { parser, pointer, marker } => {
               let concrete = MatchLastLineNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_match_last_line_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::MatchPredicateNode { parser, pointer, marker } => {
               let concrete = MatchPredicateNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_match_predicate_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::MatchRequiredNode { parser, pointer, marker } => {
               let concrete = MatchRequiredNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_match_required_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::MatchWriteNode { parser, pointer, marker } => {
               let concrete = MatchWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_match_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::MissingNode { parser, pointer, marker } => {
               let concrete = MissingNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_missing_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ModuleNode { parser, pointer, marker } => {
               let concrete = ModuleNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_module_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::MultiTargetNode { parser, pointer, marker } => {
               let concrete = MultiTargetNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_multi_target_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::MultiWriteNode { parser, pointer, marker } => {
               let concrete = MultiWriteNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_multi_write_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::NextNode { parser, pointer, marker } => {
               let concrete = NextNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_next_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::NilNode { parser, pointer, marker } => {
               let concrete = NilNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_nil_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::NoKeywordsParameterNode { parser, pointer, marker } => {
               let concrete = NoKeywordsParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_no_keywords_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::NumberedParametersNode { parser, pointer, marker } => {
               let concrete = NumberedParametersNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_numbered_parameters_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::NumberedReferenceReadNode { parser, pointer, marker } => {
               let concrete = NumberedReferenceReadNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_numbered_reference_read_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::OptionalKeywordParameterNode { parser, pointer, marker } => {
               let concrete = OptionalKeywordParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_optional_keyword_parameter_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::OptionalParameterNode { parser, pointer, marker } => {
               let concrete = OptionalParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_optional_parameter_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::OrNode { parser, pointer, marker } => {
               let concrete = OrNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_or_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ParametersNode { parser, pointer, marker } => {
               let concrete = ParametersNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_parameters_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ParenthesesNode { parser, pointer, marker } => {
               let concrete = ParenthesesNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_parentheses_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::PinnedExpressionNode { parser, pointer, marker } => {
               let concrete = PinnedExpressionNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_pinned_expression_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::PinnedVariableNode { parser, pointer, marker } => {
               let concrete = PinnedVariableNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_pinned_variable_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::PostExecutionNode { parser, pointer, marker } => {
               let concrete = PostExecutionNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_post_execution_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::PreExecutionNode { parser, pointer, marker } => {
               let concrete = PreExecutionNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_pre_execution_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::ProgramNode { parser, pointer, marker } => {
               let concrete = ProgramNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_program_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::RangeNode { parser, pointer, marker } => {
               let concrete = RangeNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_range_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::RationalNode { parser, pointer, marker } => {
               let concrete = RationalNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_rational_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::RedoNode { parser, pointer, marker } => {
               let concrete = RedoNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_redo_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::RegularExpressionNode { parser, pointer, marker } => {
               let concrete = RegularExpressionNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_regular_expression_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::RequiredKeywordParameterNode { parser, pointer, marker } => {
               let concrete = RequiredKeywordParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_required_keyword_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::RequiredParameterNode { parser, pointer, marker } => {
               let concrete = RequiredParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_required_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::RescueModifierNode { parser, pointer, marker } => {
               let concrete = RescueModifierNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_rescue_modifier_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::RescueNode { parser, pointer, marker } => {
               let concrete = RescueNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_rescue_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::RestParameterNode { parser, pointer, marker } => {
               let concrete = RestParameterNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_rest_parameter_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::RetryNode { parser, pointer, marker } => {
               let concrete = RetryNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_retry_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ReturnNode { parser, pointer, marker } => {
               let concrete = ReturnNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_return_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::SelfNode { parser, pointer, marker } => {
               let concrete = SelfNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_self_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::ShareableConstantNode { parser, pointer, marker } => {
               let concrete = ShareableConstantNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_shareable_constant_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::SingletonClassNode { parser, pointer, marker } => {
               let concrete = SingletonClassNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_singleton_class_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::SourceEncodingNode { parser, pointer, marker } => {
               let concrete = SourceEncodingNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_source_encoding_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::SourceFileNode { parser, pointer, marker } => {
               let concrete = SourceFileNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_source_file_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::SourceLineNode { parser, pointer, marker } => {
               let concrete = SourceLineNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_source_line_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::SplatNode { parser, pointer, marker } => {
               let concrete = SplatNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_splat_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::StatementsNode { parser, pointer, marker } => {
               let concrete = StatementsNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_statements_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::StringNode { parser, pointer, marker } => {
               let concrete = StringNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_string_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::SuperNode { parser, pointer, marker } => {
               let concrete = SuperNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_super_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::SymbolNode { parser, pointer, marker } => {
               let concrete = SymbolNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_symbol_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::TrueNode { parser, pointer, marker } => {
               let concrete = TrueNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_true_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::UndefNode { parser, pointer, marker } => {
               let concrete = UndefNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_undef_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::UnlessNode { parser, pointer, marker } => {
               let concrete = UnlessNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_unless_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::UntilNode { parser, pointer, marker } => {
               let concrete = UntilNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_until_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::WhenNode { parser, pointer, marker } => {
               let concrete = WhenNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_when_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::WhileNode { parser, pointer, marker } => {
               let concrete = WhileNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_while_node(&concrete);
               self.visit_branch_node_leave();
           }
           Node::XStringNode { parser, pointer, marker } => {
               let concrete = XStringNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_leaf_node_enter(concrete.as_node());
               self.visit_x_string_node(&concrete);
               self.visit_leaf_node_leave();
           }
           Node::YieldNode { parser, pointer, marker } => {
               let concrete = YieldNode { parser: *parser, pointer: *pointer, marker: *marker };
               self.visit_branch_node_enter(concrete.as_node());
               self.visit_yield_node(&concrete);
               self.visit_branch_node_leave();
           }
       }
   }

    /// Visits a `AliasGlobalVariableNode` node.
    fn visit_alias_global_variable_node(&mut self, node: &AliasGlobalVariableNode<'pr>) {
        visit_alias_global_variable_node(self, node);
    }

    /// Visits a `AliasMethodNode` node.
    fn visit_alias_method_node(&mut self, node: &AliasMethodNode<'pr>) {
        visit_alias_method_node(self, node);
    }

    /// Visits a `AlternationPatternNode` node.
    fn visit_alternation_pattern_node(&mut self, node: &AlternationPatternNode<'pr>) {
        visit_alternation_pattern_node(self, node);
    }

    /// Visits a `AndNode` node.
    fn visit_and_node(&mut self, node: &AndNode<'pr>) {
        visit_and_node(self, node);
    }

    /// Visits a `ArgumentsNode` node.
    fn visit_arguments_node(&mut self, node: &ArgumentsNode<'pr>) {
        visit_arguments_node(self, node);
    }

    /// Visits a `ArrayNode` node.
    fn visit_array_node(&mut self, node: &ArrayNode<'pr>) {
        visit_array_node(self, node);
    }

    /// Visits a `ArrayPatternNode` node.
    fn visit_array_pattern_node(&mut self, node: &ArrayPatternNode<'pr>) {
        visit_array_pattern_node(self, node);
    }

    /// Visits a `AssocNode` node.
    fn visit_assoc_node(&mut self, node: &AssocNode<'pr>) {
        visit_assoc_node(self, node);
    }

    /// Visits a `AssocSplatNode` node.
    fn visit_assoc_splat_node(&mut self, node: &AssocSplatNode<'pr>) {
        visit_assoc_splat_node(self, node);
    }

    /// Visits a `BackReferenceReadNode` node.
    fn visit_back_reference_read_node(&mut self, node: &BackReferenceReadNode<'pr>) {
        visit_back_reference_read_node(self, node);
    }

    /// Visits a `BeginNode` node.
    fn visit_begin_node(&mut self, node: &BeginNode<'pr>) {
        visit_begin_node(self, node);
    }

    /// Visits a `BlockArgumentNode` node.
    fn visit_block_argument_node(&mut self, node: &BlockArgumentNode<'pr>) {
        visit_block_argument_node(self, node);
    }

    /// Visits a `BlockLocalVariableNode` node.
    fn visit_block_local_variable_node(&mut self, node: &BlockLocalVariableNode<'pr>) {
        visit_block_local_variable_node(self, node);
    }

    /// Visits a `BlockNode` node.
    fn visit_block_node(&mut self, node: &BlockNode<'pr>) {
        visit_block_node(self, node);
    }

    /// Visits a `BlockParameterNode` node.
    fn visit_block_parameter_node(&mut self, node: &BlockParameterNode<'pr>) {
        visit_block_parameter_node(self, node);
    }

    /// Visits a `BlockParametersNode` node.
    fn visit_block_parameters_node(&mut self, node: &BlockParametersNode<'pr>) {
        visit_block_parameters_node(self, node);
    }

    /// Visits a `BreakNode` node.
    fn visit_break_node(&mut self, node: &BreakNode<'pr>) {
        visit_break_node(self, node);
    }

    /// Visits a `CallAndWriteNode` node.
    fn visit_call_and_write_node(&mut self, node: &CallAndWriteNode<'pr>) {
        visit_call_and_write_node(self, node);
    }

    /// Visits a `CallNode` node.
    fn visit_call_node(&mut self, node: &CallNode<'pr>) {
        visit_call_node(self, node);
    }

    /// Visits a `CallOperatorWriteNode` node.
    fn visit_call_operator_write_node(&mut self, node: &CallOperatorWriteNode<'pr>) {
        visit_call_operator_write_node(self, node);
    }

    /// Visits a `CallOrWriteNode` node.
    fn visit_call_or_write_node(&mut self, node: &CallOrWriteNode<'pr>) {
        visit_call_or_write_node(self, node);
    }

    /// Visits a `CallTargetNode` node.
    fn visit_call_target_node(&mut self, node: &CallTargetNode<'pr>) {
        visit_call_target_node(self, node);
    }

    /// Visits a `CapturePatternNode` node.
    fn visit_capture_pattern_node(&mut self, node: &CapturePatternNode<'pr>) {
        visit_capture_pattern_node(self, node);
    }

    /// Visits a `CaseMatchNode` node.
    fn visit_case_match_node(&mut self, node: &CaseMatchNode<'pr>) {
        visit_case_match_node(self, node);
    }

    /// Visits a `CaseNode` node.
    fn visit_case_node(&mut self, node: &CaseNode<'pr>) {
        visit_case_node(self, node);
    }

    /// Visits a `ClassNode` node.
    fn visit_class_node(&mut self, node: &ClassNode<'pr>) {
        visit_class_node(self, node);
    }

    /// Visits a `ClassVariableAndWriteNode` node.
    fn visit_class_variable_and_write_node(&mut self, node: &ClassVariableAndWriteNode<'pr>) {
        visit_class_variable_and_write_node(self, node);
    }

    /// Visits a `ClassVariableOperatorWriteNode` node.
    fn visit_class_variable_operator_write_node(&mut self, node: &ClassVariableOperatorWriteNode<'pr>) {
        visit_class_variable_operator_write_node(self, node);
    }

    /// Visits a `ClassVariableOrWriteNode` node.
    fn visit_class_variable_or_write_node(&mut self, node: &ClassVariableOrWriteNode<'pr>) {
        visit_class_variable_or_write_node(self, node);
    }

    /// Visits a `ClassVariableReadNode` node.
    fn visit_class_variable_read_node(&mut self, node: &ClassVariableReadNode<'pr>) {
        visit_class_variable_read_node(self, node);
    }

    /// Visits a `ClassVariableTargetNode` node.
    fn visit_class_variable_target_node(&mut self, node: &ClassVariableTargetNode<'pr>) {
        visit_class_variable_target_node(self, node);
    }

    /// Visits a `ClassVariableWriteNode` node.
    fn visit_class_variable_write_node(&mut self, node: &ClassVariableWriteNode<'pr>) {
        visit_class_variable_write_node(self, node);
    }

    /// Visits a `ConstantAndWriteNode` node.
    fn visit_constant_and_write_node(&mut self, node: &ConstantAndWriteNode<'pr>) {
        visit_constant_and_write_node(self, node);
    }

    /// Visits a `ConstantOperatorWriteNode` node.
    fn visit_constant_operator_write_node(&mut self, node: &ConstantOperatorWriteNode<'pr>) {
        visit_constant_operator_write_node(self, node);
    }

    /// Visits a `ConstantOrWriteNode` node.
    fn visit_constant_or_write_node(&mut self, node: &ConstantOrWriteNode<'pr>) {
        visit_constant_or_write_node(self, node);
    }

    /// Visits a `ConstantPathAndWriteNode` node.
    fn visit_constant_path_and_write_node(&mut self, node: &ConstantPathAndWriteNode<'pr>) {
        visit_constant_path_and_write_node(self, node);
    }

    /// Visits a `ConstantPathNode` node.
    fn visit_constant_path_node(&mut self, node: &ConstantPathNode<'pr>) {
        visit_constant_path_node(self, node);
    }

    /// Visits a `ConstantPathOperatorWriteNode` node.
    fn visit_constant_path_operator_write_node(&mut self, node: &ConstantPathOperatorWriteNode<'pr>) {
        visit_constant_path_operator_write_node(self, node);
    }

    /// Visits a `ConstantPathOrWriteNode` node.
    fn visit_constant_path_or_write_node(&mut self, node: &ConstantPathOrWriteNode<'pr>) {
        visit_constant_path_or_write_node(self, node);
    }

    /// Visits a `ConstantPathTargetNode` node.
    fn visit_constant_path_target_node(&mut self, node: &ConstantPathTargetNode<'pr>) {
        visit_constant_path_target_node(self, node);
    }

    /// Visits a `ConstantPathWriteNode` node.
    fn visit_constant_path_write_node(&mut self, node: &ConstantPathWriteNode<'pr>) {
        visit_constant_path_write_node(self, node);
    }

    /// Visits a `ConstantReadNode` node.
    fn visit_constant_read_node(&mut self, node: &ConstantReadNode<'pr>) {
        visit_constant_read_node(self, node);
    }

    /// Visits a `ConstantTargetNode` node.
    fn visit_constant_target_node(&mut self, node: &ConstantTargetNode<'pr>) {
        visit_constant_target_node(self, node);
    }

    /// Visits a `ConstantWriteNode` node.
    fn visit_constant_write_node(&mut self, node: &ConstantWriteNode<'pr>) {
        visit_constant_write_node(self, node);
    }

    /// Visits a `DefNode` node.
    fn visit_def_node(&mut self, node: &DefNode<'pr>) {
        visit_def_node(self, node);
    }

    /// Visits a `DefinedNode` node.
    fn visit_defined_node(&mut self, node: &DefinedNode<'pr>) {
        visit_defined_node(self, node);
    }

    /// Visits a `ElseNode` node.
    fn visit_else_node(&mut self, node: &ElseNode<'pr>) {
        visit_else_node(self, node);
    }

    /// Visits a `EmbeddedStatementsNode` node.
    fn visit_embedded_statements_node(&mut self, node: &EmbeddedStatementsNode<'pr>) {
        visit_embedded_statements_node(self, node);
    }

    /// Visits a `EmbeddedVariableNode` node.
    fn visit_embedded_variable_node(&mut self, node: &EmbeddedVariableNode<'pr>) {
        visit_embedded_variable_node(self, node);
    }

    /// Visits a `EnsureNode` node.
    fn visit_ensure_node(&mut self, node: &EnsureNode<'pr>) {
        visit_ensure_node(self, node);
    }

    /// Visits a `FalseNode` node.
    fn visit_false_node(&mut self, node: &FalseNode<'pr>) {
        visit_false_node(self, node);
    }

    /// Visits a `FindPatternNode` node.
    fn visit_find_pattern_node(&mut self, node: &FindPatternNode<'pr>) {
        visit_find_pattern_node(self, node);
    }

    /// Visits a `FlipFlopNode` node.
    fn visit_flip_flop_node(&mut self, node: &FlipFlopNode<'pr>) {
        visit_flip_flop_node(self, node);
    }

    /// Visits a `FloatNode` node.
    fn visit_float_node(&mut self, node: &FloatNode<'pr>) {
        visit_float_node(self, node);
    }

    /// Visits a `ForNode` node.
    fn visit_for_node(&mut self, node: &ForNode<'pr>) {
        visit_for_node(self, node);
    }

    /// Visits a `ForwardingArgumentsNode` node.
    fn visit_forwarding_arguments_node(&mut self, node: &ForwardingArgumentsNode<'pr>) {
        visit_forwarding_arguments_node(self, node);
    }

    /// Visits a `ForwardingParameterNode` node.
    fn visit_forwarding_parameter_node(&mut self, node: &ForwardingParameterNode<'pr>) {
        visit_forwarding_parameter_node(self, node);
    }

    /// Visits a `ForwardingSuperNode` node.
    fn visit_forwarding_super_node(&mut self, node: &ForwardingSuperNode<'pr>) {
        visit_forwarding_super_node(self, node);
    }

    /// Visits a `GlobalVariableAndWriteNode` node.
    fn visit_global_variable_and_write_node(&mut self, node: &GlobalVariableAndWriteNode<'pr>) {
        visit_global_variable_and_write_node(self, node);
    }

    /// Visits a `GlobalVariableOperatorWriteNode` node.
    fn visit_global_variable_operator_write_node(&mut self, node: &GlobalVariableOperatorWriteNode<'pr>) {
        visit_global_variable_operator_write_node(self, node);
    }

    /// Visits a `GlobalVariableOrWriteNode` node.
    fn visit_global_variable_or_write_node(&mut self, node: &GlobalVariableOrWriteNode<'pr>) {
        visit_global_variable_or_write_node(self, node);
    }

    /// Visits a `GlobalVariableReadNode` node.
    fn visit_global_variable_read_node(&mut self, node: &GlobalVariableReadNode<'pr>) {
        visit_global_variable_read_node(self, node);
    }

    /// Visits a `GlobalVariableTargetNode` node.
    fn visit_global_variable_target_node(&mut self, node: &GlobalVariableTargetNode<'pr>) {
        visit_global_variable_target_node(self, node);
    }

    /// Visits a `GlobalVariableWriteNode` node.
    fn visit_global_variable_write_node(&mut self, node: &GlobalVariableWriteNode<'pr>) {
        visit_global_variable_write_node(self, node);
    }

    /// Visits a `HashNode` node.
    fn visit_hash_node(&mut self, node: &HashNode<'pr>) {
        visit_hash_node(self, node);
    }

    /// Visits a `HashPatternNode` node.
    fn visit_hash_pattern_node(&mut self, node: &HashPatternNode<'pr>) {
        visit_hash_pattern_node(self, node);
    }

    /// Visits a `IfNode` node.
    fn visit_if_node(&mut self, node: &IfNode<'pr>) {
        visit_if_node(self, node);
    }

    /// Visits a `ImaginaryNode` node.
    fn visit_imaginary_node(&mut self, node: &ImaginaryNode<'pr>) {
        visit_imaginary_node(self, node);
    }

    /// Visits a `ImplicitNode` node.
    fn visit_implicit_node(&mut self, node: &ImplicitNode<'pr>) {
        visit_implicit_node(self, node);
    }

    /// Visits a `ImplicitRestNode` node.
    fn visit_implicit_rest_node(&mut self, node: &ImplicitRestNode<'pr>) {
        visit_implicit_rest_node(self, node);
    }

    /// Visits a `InNode` node.
    fn visit_in_node(&mut self, node: &InNode<'pr>) {
        visit_in_node(self, node);
    }

    /// Visits a `IndexAndWriteNode` node.
    fn visit_index_and_write_node(&mut self, node: &IndexAndWriteNode<'pr>) {
        visit_index_and_write_node(self, node);
    }

    /// Visits a `IndexOperatorWriteNode` node.
    fn visit_index_operator_write_node(&mut self, node: &IndexOperatorWriteNode<'pr>) {
        visit_index_operator_write_node(self, node);
    }

    /// Visits a `IndexOrWriteNode` node.
    fn visit_index_or_write_node(&mut self, node: &IndexOrWriteNode<'pr>) {
        visit_index_or_write_node(self, node);
    }

    /// Visits a `IndexTargetNode` node.
    fn visit_index_target_node(&mut self, node: &IndexTargetNode<'pr>) {
        visit_index_target_node(self, node);
    }

    /// Visits a `InstanceVariableAndWriteNode` node.
    fn visit_instance_variable_and_write_node(&mut self, node: &InstanceVariableAndWriteNode<'pr>) {
        visit_instance_variable_and_write_node(self, node);
    }

    /// Visits a `InstanceVariableOperatorWriteNode` node.
    fn visit_instance_variable_operator_write_node(&mut self, node: &InstanceVariableOperatorWriteNode<'pr>) {
        visit_instance_variable_operator_write_node(self, node);
    }

    /// Visits a `InstanceVariableOrWriteNode` node.
    fn visit_instance_variable_or_write_node(&mut self, node: &InstanceVariableOrWriteNode<'pr>) {
        visit_instance_variable_or_write_node(self, node);
    }

    /// Visits a `InstanceVariableReadNode` node.
    fn visit_instance_variable_read_node(&mut self, node: &InstanceVariableReadNode<'pr>) {
        visit_instance_variable_read_node(self, node);
    }

    /// Visits a `InstanceVariableTargetNode` node.
    fn visit_instance_variable_target_node(&mut self, node: &InstanceVariableTargetNode<'pr>) {
        visit_instance_variable_target_node(self, node);
    }

    /// Visits a `InstanceVariableWriteNode` node.
    fn visit_instance_variable_write_node(&mut self, node: &InstanceVariableWriteNode<'pr>) {
        visit_instance_variable_write_node(self, node);
    }

    /// Visits a `IntegerNode` node.
    fn visit_integer_node(&mut self, node: &IntegerNode<'pr>) {
        visit_integer_node(self, node);
    }

    /// Visits a `InterpolatedMatchLastLineNode` node.
    fn visit_interpolated_match_last_line_node(&mut self, node: &InterpolatedMatchLastLineNode<'pr>) {
        visit_interpolated_match_last_line_node(self, node);
    }

    /// Visits a `InterpolatedRegularExpressionNode` node.
    fn visit_interpolated_regular_expression_node(&mut self, node: &InterpolatedRegularExpressionNode<'pr>) {
        visit_interpolated_regular_expression_node(self, node);
    }

    /// Visits a `InterpolatedStringNode` node.
    fn visit_interpolated_string_node(&mut self, node: &InterpolatedStringNode<'pr>) {
        visit_interpolated_string_node(self, node);
    }

    /// Visits a `InterpolatedSymbolNode` node.
    fn visit_interpolated_symbol_node(&mut self, node: &InterpolatedSymbolNode<'pr>) {
        visit_interpolated_symbol_node(self, node);
    }

    /// Visits a `InterpolatedXStringNode` node.
    fn visit_interpolated_x_string_node(&mut self, node: &InterpolatedXStringNode<'pr>) {
        visit_interpolated_x_string_node(self, node);
    }

    /// Visits a `ItLocalVariableReadNode` node.
    fn visit_it_local_variable_read_node(&mut self, node: &ItLocalVariableReadNode<'pr>) {
        visit_it_local_variable_read_node(self, node);
    }

    /// Visits a `ItParametersNode` node.
    fn visit_it_parameters_node(&mut self, node: &ItParametersNode<'pr>) {
        visit_it_parameters_node(self, node);
    }

    /// Visits a `KeywordHashNode` node.
    fn visit_keyword_hash_node(&mut self, node: &KeywordHashNode<'pr>) {
        visit_keyword_hash_node(self, node);
    }

    /// Visits a `KeywordRestParameterNode` node.
    fn visit_keyword_rest_parameter_node(&mut self, node: &KeywordRestParameterNode<'pr>) {
        visit_keyword_rest_parameter_node(self, node);
    }

    /// Visits a `LambdaNode` node.
    fn visit_lambda_node(&mut self, node: &LambdaNode<'pr>) {
        visit_lambda_node(self, node);
    }

    /// Visits a `LocalVariableAndWriteNode` node.
    fn visit_local_variable_and_write_node(&mut self, node: &LocalVariableAndWriteNode<'pr>) {
        visit_local_variable_and_write_node(self, node);
    }

    /// Visits a `LocalVariableOperatorWriteNode` node.
    fn visit_local_variable_operator_write_node(&mut self, node: &LocalVariableOperatorWriteNode<'pr>) {
        visit_local_variable_operator_write_node(self, node);
    }

    /// Visits a `LocalVariableOrWriteNode` node.
    fn visit_local_variable_or_write_node(&mut self, node: &LocalVariableOrWriteNode<'pr>) {
        visit_local_variable_or_write_node(self, node);
    }

    /// Visits a `LocalVariableReadNode` node.
    fn visit_local_variable_read_node(&mut self, node: &LocalVariableReadNode<'pr>) {
        visit_local_variable_read_node(self, node);
    }

    /// Visits a `LocalVariableTargetNode` node.
    fn visit_local_variable_target_node(&mut self, node: &LocalVariableTargetNode<'pr>) {
        visit_local_variable_target_node(self, node);
    }

    /// Visits a `LocalVariableWriteNode` node.
    fn visit_local_variable_write_node(&mut self, node: &LocalVariableWriteNode<'pr>) {
        visit_local_variable_write_node(self, node);
    }

    /// Visits a `MatchLastLineNode` node.
    fn visit_match_last_line_node(&mut self, node: &MatchLastLineNode<'pr>) {
        visit_match_last_line_node(self, node);
    }

    /// Visits a `MatchPredicateNode` node.
    fn visit_match_predicate_node(&mut self, node: &MatchPredicateNode<'pr>) {
        visit_match_predicate_node(self, node);
    }

    /// Visits a `MatchRequiredNode` node.
    fn visit_match_required_node(&mut self, node: &MatchRequiredNode<'pr>) {
        visit_match_required_node(self, node);
    }

    /// Visits a `MatchWriteNode` node.
    fn visit_match_write_node(&mut self, node: &MatchWriteNode<'pr>) {
        visit_match_write_node(self, node);
    }

    /// Visits a `MissingNode` node.
    fn visit_missing_node(&mut self, node: &MissingNode<'pr>) {
        visit_missing_node(self, node);
    }

    /// Visits a `ModuleNode` node.
    fn visit_module_node(&mut self, node: &ModuleNode<'pr>) {
        visit_module_node(self, node);
    }

    /// Visits a `MultiTargetNode` node.
    fn visit_multi_target_node(&mut self, node: &MultiTargetNode<'pr>) {
        visit_multi_target_node(self, node);
    }

    /// Visits a `MultiWriteNode` node.
    fn visit_multi_write_node(&mut self, node: &MultiWriteNode<'pr>) {
        visit_multi_write_node(self, node);
    }

    /// Visits a `NextNode` node.
    fn visit_next_node(&mut self, node: &NextNode<'pr>) {
        visit_next_node(self, node);
    }

    /// Visits a `NilNode` node.
    fn visit_nil_node(&mut self, node: &NilNode<'pr>) {
        visit_nil_node(self, node);
    }

    /// Visits a `NoKeywordsParameterNode` node.
    fn visit_no_keywords_parameter_node(&mut self, node: &NoKeywordsParameterNode<'pr>) {
        visit_no_keywords_parameter_node(self, node);
    }

    /// Visits a `NumberedParametersNode` node.
    fn visit_numbered_parameters_node(&mut self, node: &NumberedParametersNode<'pr>) {
        visit_numbered_parameters_node(self, node);
    }

    /// Visits a `NumberedReferenceReadNode` node.
    fn visit_numbered_reference_read_node(&mut self, node: &NumberedReferenceReadNode<'pr>) {
        visit_numbered_reference_read_node(self, node);
    }

    /// Visits a `OptionalKeywordParameterNode` node.
    fn visit_optional_keyword_parameter_node(&mut self, node: &OptionalKeywordParameterNode<'pr>) {
        visit_optional_keyword_parameter_node(self, node);
    }

    /// Visits a `OptionalParameterNode` node.
    fn visit_optional_parameter_node(&mut self, node: &OptionalParameterNode<'pr>) {
        visit_optional_parameter_node(self, node);
    }

    /// Visits a `OrNode` node.
    fn visit_or_node(&mut self, node: &OrNode<'pr>) {
        visit_or_node(self, node);
    }

    /// Visits a `ParametersNode` node.
    fn visit_parameters_node(&mut self, node: &ParametersNode<'pr>) {
        visit_parameters_node(self, node);
    }

    /// Visits a `ParenthesesNode` node.
    fn visit_parentheses_node(&mut self, node: &ParenthesesNode<'pr>) {
        visit_parentheses_node(self, node);
    }

    /// Visits a `PinnedExpressionNode` node.
    fn visit_pinned_expression_node(&mut self, node: &PinnedExpressionNode<'pr>) {
        visit_pinned_expression_node(self, node);
    }

    /// Visits a `PinnedVariableNode` node.
    fn visit_pinned_variable_node(&mut self, node: &PinnedVariableNode<'pr>) {
        visit_pinned_variable_node(self, node);
    }

    /// Visits a `PostExecutionNode` node.
    fn visit_post_execution_node(&mut self, node: &PostExecutionNode<'pr>) {
        visit_post_execution_node(self, node);
    }

    /// Visits a `PreExecutionNode` node.
    fn visit_pre_execution_node(&mut self, node: &PreExecutionNode<'pr>) {
        visit_pre_execution_node(self, node);
    }

    /// Visits a `ProgramNode` node.
    fn visit_program_node(&mut self, node: &ProgramNode<'pr>) {
        visit_program_node(self, node);
    }

    /// Visits a `RangeNode` node.
    fn visit_range_node(&mut self, node: &RangeNode<'pr>) {
        visit_range_node(self, node);
    }

    /// Visits a `RationalNode` node.
    fn visit_rational_node(&mut self, node: &RationalNode<'pr>) {
        visit_rational_node(self, node);
    }

    /// Visits a `RedoNode` node.
    fn visit_redo_node(&mut self, node: &RedoNode<'pr>) {
        visit_redo_node(self, node);
    }

    /// Visits a `RegularExpressionNode` node.
    fn visit_regular_expression_node(&mut self, node: &RegularExpressionNode<'pr>) {
        visit_regular_expression_node(self, node);
    }

    /// Visits a `RequiredKeywordParameterNode` node.
    fn visit_required_keyword_parameter_node(&mut self, node: &RequiredKeywordParameterNode<'pr>) {
        visit_required_keyword_parameter_node(self, node);
    }

    /// Visits a `RequiredParameterNode` node.
    fn visit_required_parameter_node(&mut self, node: &RequiredParameterNode<'pr>) {
        visit_required_parameter_node(self, node);
    }

    /// Visits a `RescueModifierNode` node.
    fn visit_rescue_modifier_node(&mut self, node: &RescueModifierNode<'pr>) {
        visit_rescue_modifier_node(self, node);
    }

    /// Visits a `RescueNode` node.
    fn visit_rescue_node(&mut self, node: &RescueNode<'pr>) {
        visit_rescue_node(self, node);
    }

    /// Visits a `RestParameterNode` node.
    fn visit_rest_parameter_node(&mut self, node: &RestParameterNode<'pr>) {
        visit_rest_parameter_node(self, node);
    }

    /// Visits a `RetryNode` node.
    fn visit_retry_node(&mut self, node: &RetryNode<'pr>) {
        visit_retry_node(self, node);
    }

    /// Visits a `ReturnNode` node.
    fn visit_return_node(&mut self, node: &ReturnNode<'pr>) {
        visit_return_node(self, node);
    }

    /// Visits a `SelfNode` node.
    fn visit_self_node(&mut self, node: &SelfNode<'pr>) {
        visit_self_node(self, node);
    }

    /// Visits a `ShareableConstantNode` node.
    fn visit_shareable_constant_node(&mut self, node: &ShareableConstantNode<'pr>) {
        visit_shareable_constant_node(self, node);
    }

    /// Visits a `SingletonClassNode` node.
    fn visit_singleton_class_node(&mut self, node: &SingletonClassNode<'pr>) {
        visit_singleton_class_node(self, node);
    }

    /// Visits a `SourceEncodingNode` node.
    fn visit_source_encoding_node(&mut self, node: &SourceEncodingNode<'pr>) {
        visit_source_encoding_node(self, node);
    }

    /// Visits a `SourceFileNode` node.
    fn visit_source_file_node(&mut self, node: &SourceFileNode<'pr>) {
        visit_source_file_node(self, node);
    }

    /// Visits a `SourceLineNode` node.
    fn visit_source_line_node(&mut self, node: &SourceLineNode<'pr>) {
        visit_source_line_node(self, node);
    }

    /// Visits a `SplatNode` node.
    fn visit_splat_node(&mut self, node: &SplatNode<'pr>) {
        visit_splat_node(self, node);
    }

    /// Visits a `StatementsNode` node.
    fn visit_statements_node(&mut self, node: &StatementsNode<'pr>) {
        visit_statements_node(self, node);
    }

    /// Visits a `StringNode` node.
    fn visit_string_node(&mut self, node: &StringNode<'pr>) {
        visit_string_node(self, node);
    }

    /// Visits a `SuperNode` node.
    fn visit_super_node(&mut self, node: &SuperNode<'pr>) {
        visit_super_node(self, node);
    }

    /// Visits a `SymbolNode` node.
    fn visit_symbol_node(&mut self, node: &SymbolNode<'pr>) {
        visit_symbol_node(self, node);
    }

    /// Visits a `TrueNode` node.
    fn visit_true_node(&mut self, node: &TrueNode<'pr>) {
        visit_true_node(self, node);
    }

    /// Visits a `UndefNode` node.
    fn visit_undef_node(&mut self, node: &UndefNode<'pr>) {
        visit_undef_node(self, node);
    }

    /// Visits a `UnlessNode` node.
    fn visit_unless_node(&mut self, node: &UnlessNode<'pr>) {
        visit_unless_node(self, node);
    }

    /// Visits a `UntilNode` node.
    fn visit_until_node(&mut self, node: &UntilNode<'pr>) {
        visit_until_node(self, node);
    }

    /// Visits a `WhenNode` node.
    fn visit_when_node(&mut self, node: &WhenNode<'pr>) {
        visit_when_node(self, node);
    }

    /// Visits a `WhileNode` node.
    fn visit_while_node(&mut self, node: &WhileNode<'pr>) {
        visit_while_node(self, node);
    }

    /// Visits a `XStringNode` node.
    fn visit_x_string_node(&mut self, node: &XStringNode<'pr>) {
        visit_x_string_node(self, node);
    }

    /// Visits a `YieldNode` node.
    fn visit_yield_node(&mut self, node: &YieldNode<'pr>) {
        visit_yield_node(self, node);
    }
}

/// The default visitor implementation for a `AliasGlobalVariableNode` node.
pub fn visit_alias_global_variable_node<'pr, V>(visitor: &mut V, node: &AliasGlobalVariableNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.new_name());
    visitor.visit(&node.old_name());
}

/// The default visitor implementation for a `AliasMethodNode` node.
pub fn visit_alias_method_node<'pr, V>(visitor: &mut V, node: &AliasMethodNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.new_name());
    visitor.visit(&node.old_name());
}

/// The default visitor implementation for a `AlternationPatternNode` node.
pub fn visit_alternation_pattern_node<'pr, V>(visitor: &mut V, node: &AlternationPatternNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.left());
    visitor.visit(&node.right());
}

/// The default visitor implementation for a `AndNode` node.
pub fn visit_and_node<'pr, V>(visitor: &mut V, node: &AndNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.left());
    visitor.visit(&node.right());
}

/// The default visitor implementation for a `ArgumentsNode` node.
pub fn visit_arguments_node<'pr, V>(visitor: &mut V, node: &ArgumentsNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.arguments() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ArrayNode` node.
pub fn visit_array_node<'pr, V>(visitor: &mut V, node: &ArrayNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.elements() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ArrayPatternNode` node.
pub fn visit_array_pattern_node<'pr, V>(visitor: &mut V, node: &ArrayPatternNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.constant() {
        visitor.visit(&node);
    }
    for node in &node.requireds() {
        visitor.visit(&node);
    }
    if let Some(node) = node.rest() {
        visitor.visit(&node);
    }
    for node in &node.posts() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `AssocNode` node.
pub fn visit_assoc_node<'pr, V>(visitor: &mut V, node: &AssocNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.key());
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `AssocSplatNode` node.
pub fn visit_assoc_splat_node<'pr, V>(visitor: &mut V, node: &AssocSplatNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.value() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `BackReferenceReadNode` node.
pub const fn visit_back_reference_read_node<'pr, V>(_visitor: &mut V, _node: &BackReferenceReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `BeginNode` node.
pub fn visit_begin_node<'pr, V>(visitor: &mut V, node: &BeginNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
    if let Some(node) = node.rescue_clause() {
        visitor.visit_rescue_node(&node);
    }
    if let Some(node) = node.else_clause() {
        visitor.visit_else_node(&node);
    }
    if let Some(node) = node.ensure_clause() {
        visitor.visit_ensure_node(&node);
    }
}

/// The default visitor implementation for a `BlockArgumentNode` node.
pub fn visit_block_argument_node<'pr, V>(visitor: &mut V, node: &BlockArgumentNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.expression() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `BlockLocalVariableNode` node.
pub const fn visit_block_local_variable_node<'pr, V>(_visitor: &mut V, _node: &BlockLocalVariableNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `BlockNode` node.
pub fn visit_block_node<'pr, V>(visitor: &mut V, node: &BlockNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.parameters() {
        visitor.visit(&node);
    }
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `BlockParameterNode` node.
pub const fn visit_block_parameter_node<'pr, V>(_visitor: &mut V, _node: &BlockParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `BlockParametersNode` node.
pub fn visit_block_parameters_node<'pr, V>(visitor: &mut V, node: &BlockParametersNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.parameters() {
        visitor.visit_parameters_node(&node);
    }
    for node in &node.locals() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `BreakNode` node.
pub fn visit_break_node<'pr, V>(visitor: &mut V, node: &BreakNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
}

/// The default visitor implementation for a `CallAndWriteNode` node.
pub fn visit_call_and_write_node<'pr, V>(visitor: &mut V, node: &CallAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `CallNode` node.
pub fn visit_call_node<'pr, V>(visitor: &mut V, node: &CallNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `CallOperatorWriteNode` node.
pub fn visit_call_operator_write_node<'pr, V>(visitor: &mut V, node: &CallOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `CallOrWriteNode` node.
pub fn visit_call_or_write_node<'pr, V>(visitor: &mut V, node: &CallOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `CallTargetNode` node.
pub fn visit_call_target_node<'pr, V>(visitor: &mut V, node: &CallTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.receiver());
}

/// The default visitor implementation for a `CapturePatternNode` node.
pub fn visit_capture_pattern_node<'pr, V>(visitor: &mut V, node: &CapturePatternNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
    visitor.visit_local_variable_target_node(&node.target());
}

/// The default visitor implementation for a `CaseMatchNode` node.
pub fn visit_case_match_node<'pr, V>(visitor: &mut V, node: &CaseMatchNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.predicate() {
        visitor.visit(&node);
    }
    for node in &node.conditions() {
        visitor.visit(&node);
    }
    if let Some(node) = node.else_clause() {
        visitor.visit_else_node(&node);
    }
}

/// The default visitor implementation for a `CaseNode` node.
pub fn visit_case_node<'pr, V>(visitor: &mut V, node: &CaseNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.predicate() {
        visitor.visit(&node);
    }
    for node in &node.conditions() {
        visitor.visit(&node);
    }
    if let Some(node) = node.else_clause() {
        visitor.visit_else_node(&node);
    }
}

/// The default visitor implementation for a `ClassNode` node.
pub fn visit_class_node<'pr, V>(visitor: &mut V, node: &ClassNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.constant_path());
    if let Some(node) = node.superclass() {
        visitor.visit(&node);
    }
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ClassVariableAndWriteNode` node.
pub fn visit_class_variable_and_write_node<'pr, V>(visitor: &mut V, node: &ClassVariableAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ClassVariableOperatorWriteNode` node.
pub fn visit_class_variable_operator_write_node<'pr, V>(visitor: &mut V, node: &ClassVariableOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ClassVariableOrWriteNode` node.
pub fn visit_class_variable_or_write_node<'pr, V>(visitor: &mut V, node: &ClassVariableOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ClassVariableReadNode` node.
pub const fn visit_class_variable_read_node<'pr, V>(_visitor: &mut V, _node: &ClassVariableReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ClassVariableTargetNode` node.
pub const fn visit_class_variable_target_node<'pr, V>(_visitor: &mut V, _node: &ClassVariableTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ClassVariableWriteNode` node.
pub fn visit_class_variable_write_node<'pr, V>(visitor: &mut V, node: &ClassVariableWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantAndWriteNode` node.
pub fn visit_constant_and_write_node<'pr, V>(visitor: &mut V, node: &ConstantAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantOperatorWriteNode` node.
pub fn visit_constant_operator_write_node<'pr, V>(visitor: &mut V, node: &ConstantOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantOrWriteNode` node.
pub fn visit_constant_or_write_node<'pr, V>(visitor: &mut V, node: &ConstantOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantPathAndWriteNode` node.
pub fn visit_constant_path_and_write_node<'pr, V>(visitor: &mut V, node: &ConstantPathAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit_constant_path_node(&node.target());
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantPathNode` node.
pub fn visit_constant_path_node<'pr, V>(visitor: &mut V, node: &ConstantPathNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.parent() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ConstantPathOperatorWriteNode` node.
pub fn visit_constant_path_operator_write_node<'pr, V>(visitor: &mut V, node: &ConstantPathOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit_constant_path_node(&node.target());
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantPathOrWriteNode` node.
pub fn visit_constant_path_or_write_node<'pr, V>(visitor: &mut V, node: &ConstantPathOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit_constant_path_node(&node.target());
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantPathTargetNode` node.
pub fn visit_constant_path_target_node<'pr, V>(visitor: &mut V, node: &ConstantPathTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.parent() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ConstantPathWriteNode` node.
pub fn visit_constant_path_write_node<'pr, V>(visitor: &mut V, node: &ConstantPathWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit_constant_path_node(&node.target());
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ConstantReadNode` node.
pub const fn visit_constant_read_node<'pr, V>(_visitor: &mut V, _node: &ConstantReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ConstantTargetNode` node.
pub const fn visit_constant_target_node<'pr, V>(_visitor: &mut V, _node: &ConstantTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ConstantWriteNode` node.
pub fn visit_constant_write_node<'pr, V>(visitor: &mut V, node: &ConstantWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `DefNode` node.
pub fn visit_def_node<'pr, V>(visitor: &mut V, node: &DefNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    if let Some(node) = node.parameters() {
        visitor.visit_parameters_node(&node);
    }
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `DefinedNode` node.
pub fn visit_defined_node<'pr, V>(visitor: &mut V, node: &DefinedNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ElseNode` node.
pub fn visit_else_node<'pr, V>(visitor: &mut V, node: &ElseNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `EmbeddedStatementsNode` node.
pub fn visit_embedded_statements_node<'pr, V>(visitor: &mut V, node: &EmbeddedStatementsNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `EmbeddedVariableNode` node.
pub fn visit_embedded_variable_node<'pr, V>(visitor: &mut V, node: &EmbeddedVariableNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.variable());
}

/// The default visitor implementation for a `EnsureNode` node.
pub fn visit_ensure_node<'pr, V>(visitor: &mut V, node: &EnsureNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `FalseNode` node.
pub const fn visit_false_node<'pr, V>(_visitor: &mut V, _node: &FalseNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `FindPatternNode` node.
pub fn visit_find_pattern_node<'pr, V>(visitor: &mut V, node: &FindPatternNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.constant() {
        visitor.visit(&node);
    }
    visitor.visit_splat_node(&node.left());
    for node in &node.requireds() {
        visitor.visit(&node);
    }
    visitor.visit(&node.right());
}

/// The default visitor implementation for a `FlipFlopNode` node.
pub fn visit_flip_flop_node<'pr, V>(visitor: &mut V, node: &FlipFlopNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.left() {
        visitor.visit(&node);
    }
    if let Some(node) = node.right() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `FloatNode` node.
pub const fn visit_float_node<'pr, V>(_visitor: &mut V, _node: &FloatNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ForNode` node.
pub fn visit_for_node<'pr, V>(visitor: &mut V, node: &ForNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.index());
    visitor.visit(&node.collection());
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `ForwardingArgumentsNode` node.
pub const fn visit_forwarding_arguments_node<'pr, V>(_visitor: &mut V, _node: &ForwardingArgumentsNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ForwardingParameterNode` node.
pub const fn visit_forwarding_parameter_node<'pr, V>(_visitor: &mut V, _node: &ForwardingParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ForwardingSuperNode` node.
pub fn visit_forwarding_super_node<'pr, V>(visitor: &mut V, node: &ForwardingSuperNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.block() {
        visitor.visit_block_node(&node);
    }
}

/// The default visitor implementation for a `GlobalVariableAndWriteNode` node.
pub fn visit_global_variable_and_write_node<'pr, V>(visitor: &mut V, node: &GlobalVariableAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `GlobalVariableOperatorWriteNode` node.
pub fn visit_global_variable_operator_write_node<'pr, V>(visitor: &mut V, node: &GlobalVariableOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `GlobalVariableOrWriteNode` node.
pub fn visit_global_variable_or_write_node<'pr, V>(visitor: &mut V, node: &GlobalVariableOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `GlobalVariableReadNode` node.
pub const fn visit_global_variable_read_node<'pr, V>(_visitor: &mut V, _node: &GlobalVariableReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `GlobalVariableTargetNode` node.
pub const fn visit_global_variable_target_node<'pr, V>(_visitor: &mut V, _node: &GlobalVariableTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `GlobalVariableWriteNode` node.
pub fn visit_global_variable_write_node<'pr, V>(visitor: &mut V, node: &GlobalVariableWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `HashNode` node.
pub fn visit_hash_node<'pr, V>(visitor: &mut V, node: &HashNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.elements() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `HashPatternNode` node.
pub fn visit_hash_pattern_node<'pr, V>(visitor: &mut V, node: &HashPatternNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.constant() {
        visitor.visit(&node);
    }
    for node in &node.elements() {
        visitor.visit(&node);
    }
    if let Some(node) = node.rest() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `IfNode` node.
pub fn visit_if_node<'pr, V>(visitor: &mut V, node: &IfNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.predicate());
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
    if let Some(node) = node.subsequent() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ImaginaryNode` node.
pub fn visit_imaginary_node<'pr, V>(visitor: &mut V, node: &ImaginaryNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.numeric());
}

/// The default visitor implementation for a `ImplicitNode` node.
pub fn visit_implicit_node<'pr, V>(visitor: &mut V, node: &ImplicitNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `ImplicitRestNode` node.
pub const fn visit_implicit_rest_node<'pr, V>(_visitor: &mut V, _node: &ImplicitRestNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `InNode` node.
pub fn visit_in_node<'pr, V>(visitor: &mut V, node: &InNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.pattern());
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `IndexAndWriteNode` node.
pub fn visit_index_and_write_node<'pr, V>(visitor: &mut V, node: &IndexAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit_block_argument_node(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `IndexOperatorWriteNode` node.
pub fn visit_index_operator_write_node<'pr, V>(visitor: &mut V, node: &IndexOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit_block_argument_node(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `IndexOrWriteNode` node.
pub fn visit_index_or_write_node<'pr, V>(visitor: &mut V, node: &IndexOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.receiver() {
        visitor.visit(&node);
    }
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit_block_argument_node(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `IndexTargetNode` node.
pub fn visit_index_target_node<'pr, V>(visitor: &mut V, node: &IndexTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.receiver());
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit_block_argument_node(&node);
    }
}

/// The default visitor implementation for a `InstanceVariableAndWriteNode` node.
pub fn visit_instance_variable_and_write_node<'pr, V>(visitor: &mut V, node: &InstanceVariableAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `InstanceVariableOperatorWriteNode` node.
pub fn visit_instance_variable_operator_write_node<'pr, V>(visitor: &mut V, node: &InstanceVariableOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `InstanceVariableOrWriteNode` node.
pub fn visit_instance_variable_or_write_node<'pr, V>(visitor: &mut V, node: &InstanceVariableOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `InstanceVariableReadNode` node.
pub const fn visit_instance_variable_read_node<'pr, V>(_visitor: &mut V, _node: &InstanceVariableReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `InstanceVariableTargetNode` node.
pub const fn visit_instance_variable_target_node<'pr, V>(_visitor: &mut V, _node: &InstanceVariableTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `InstanceVariableWriteNode` node.
pub fn visit_instance_variable_write_node<'pr, V>(visitor: &mut V, node: &InstanceVariableWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `IntegerNode` node.
pub const fn visit_integer_node<'pr, V>(_visitor: &mut V, _node: &IntegerNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `InterpolatedMatchLastLineNode` node.
pub fn visit_interpolated_match_last_line_node<'pr, V>(visitor: &mut V, node: &InterpolatedMatchLastLineNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.parts() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `InterpolatedRegularExpressionNode` node.
pub fn visit_interpolated_regular_expression_node<'pr, V>(visitor: &mut V, node: &InterpolatedRegularExpressionNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.parts() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `InterpolatedStringNode` node.
pub fn visit_interpolated_string_node<'pr, V>(visitor: &mut V, node: &InterpolatedStringNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.parts() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `InterpolatedSymbolNode` node.
pub fn visit_interpolated_symbol_node<'pr, V>(visitor: &mut V, node: &InterpolatedSymbolNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.parts() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `InterpolatedXStringNode` node.
pub fn visit_interpolated_x_string_node<'pr, V>(visitor: &mut V, node: &InterpolatedXStringNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.parts() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `ItLocalVariableReadNode` node.
pub const fn visit_it_local_variable_read_node<'pr, V>(_visitor: &mut V, _node: &ItLocalVariableReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ItParametersNode` node.
pub const fn visit_it_parameters_node<'pr, V>(_visitor: &mut V, _node: &ItParametersNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `KeywordHashNode` node.
pub fn visit_keyword_hash_node<'pr, V>(visitor: &mut V, node: &KeywordHashNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.elements() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `KeywordRestParameterNode` node.
pub const fn visit_keyword_rest_parameter_node<'pr, V>(_visitor: &mut V, _node: &KeywordRestParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `LambdaNode` node.
pub fn visit_lambda_node<'pr, V>(visitor: &mut V, node: &LambdaNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.parameters() {
        visitor.visit(&node);
    }
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `LocalVariableAndWriteNode` node.
pub fn visit_local_variable_and_write_node<'pr, V>(visitor: &mut V, node: &LocalVariableAndWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `LocalVariableOperatorWriteNode` node.
pub fn visit_local_variable_operator_write_node<'pr, V>(visitor: &mut V, node: &LocalVariableOperatorWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `LocalVariableOrWriteNode` node.
pub fn visit_local_variable_or_write_node<'pr, V>(visitor: &mut V, node: &LocalVariableOrWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `LocalVariableReadNode` node.
pub const fn visit_local_variable_read_node<'pr, V>(_visitor: &mut V, _node: &LocalVariableReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `LocalVariableTargetNode` node.
pub const fn visit_local_variable_target_node<'pr, V>(_visitor: &mut V, _node: &LocalVariableTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `LocalVariableWriteNode` node.
pub fn visit_local_variable_write_node<'pr, V>(visitor: &mut V, node: &LocalVariableWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `MatchLastLineNode` node.
pub const fn visit_match_last_line_node<'pr, V>(_visitor: &mut V, _node: &MatchLastLineNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `MatchPredicateNode` node.
pub fn visit_match_predicate_node<'pr, V>(visitor: &mut V, node: &MatchPredicateNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
    visitor.visit(&node.pattern());
}

/// The default visitor implementation for a `MatchRequiredNode` node.
pub fn visit_match_required_node<'pr, V>(visitor: &mut V, node: &MatchRequiredNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
    visitor.visit(&node.pattern());
}

/// The default visitor implementation for a `MatchWriteNode` node.
pub fn visit_match_write_node<'pr, V>(visitor: &mut V, node: &MatchWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit_call_node(&node.call());
    for node in &node.targets() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `MissingNode` node.
pub const fn visit_missing_node<'pr, V>(_visitor: &mut V, _node: &MissingNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ModuleNode` node.
pub fn visit_module_node<'pr, V>(visitor: &mut V, node: &ModuleNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.constant_path());
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `MultiTargetNode` node.
pub fn visit_multi_target_node<'pr, V>(visitor: &mut V, node: &MultiTargetNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.lefts() {
        visitor.visit(&node);
    }
    if let Some(node) = node.rest() {
        visitor.visit(&node);
    }
    for node in &node.rights() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `MultiWriteNode` node.
pub fn visit_multi_write_node<'pr, V>(visitor: &mut V, node: &MultiWriteNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.lefts() {
        visitor.visit(&node);
    }
    if let Some(node) = node.rest() {
        visitor.visit(&node);
    }
    for node in &node.rights() {
        visitor.visit(&node);
    }
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `NextNode` node.
pub fn visit_next_node<'pr, V>(visitor: &mut V, node: &NextNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
}

/// The default visitor implementation for a `NilNode` node.
pub const fn visit_nil_node<'pr, V>(_visitor: &mut V, _node: &NilNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `NoKeywordsParameterNode` node.
pub const fn visit_no_keywords_parameter_node<'pr, V>(_visitor: &mut V, _node: &NoKeywordsParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `NumberedParametersNode` node.
pub const fn visit_numbered_parameters_node<'pr, V>(_visitor: &mut V, _node: &NumberedParametersNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `NumberedReferenceReadNode` node.
pub const fn visit_numbered_reference_read_node<'pr, V>(_visitor: &mut V, _node: &NumberedReferenceReadNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `OptionalKeywordParameterNode` node.
pub fn visit_optional_keyword_parameter_node<'pr, V>(visitor: &mut V, node: &OptionalKeywordParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `OptionalParameterNode` node.
pub fn visit_optional_parameter_node<'pr, V>(visitor: &mut V, node: &OptionalParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.value());
}

/// The default visitor implementation for a `OrNode` node.
pub fn visit_or_node<'pr, V>(visitor: &mut V, node: &OrNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.left());
    visitor.visit(&node.right());
}

/// The default visitor implementation for a `ParametersNode` node.
pub fn visit_parameters_node<'pr, V>(visitor: &mut V, node: &ParametersNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.requireds() {
        visitor.visit(&node);
    }
    for node in &node.optionals() {
        visitor.visit(&node);
    }
    if let Some(node) = node.rest() {
        visitor.visit(&node);
    }
    for node in &node.posts() {
        visitor.visit(&node);
    }
    for node in &node.keywords() {
        visitor.visit(&node);
    }
    if let Some(node) = node.keyword_rest() {
        visitor.visit(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit_block_parameter_node(&node);
    }
}

/// The default visitor implementation for a `ParenthesesNode` node.
pub fn visit_parentheses_node<'pr, V>(visitor: &mut V, node: &ParenthesesNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `PinnedExpressionNode` node.
pub fn visit_pinned_expression_node<'pr, V>(visitor: &mut V, node: &PinnedExpressionNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.expression());
}

/// The default visitor implementation for a `PinnedVariableNode` node.
pub fn visit_pinned_variable_node<'pr, V>(visitor: &mut V, node: &PinnedVariableNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.variable());
}

/// The default visitor implementation for a `PostExecutionNode` node.
pub fn visit_post_execution_node<'pr, V>(visitor: &mut V, node: &PostExecutionNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `PreExecutionNode` node.
pub fn visit_pre_execution_node<'pr, V>(visitor: &mut V, node: &PreExecutionNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `ProgramNode` node.
pub fn visit_program_node<'pr, V>(visitor: &mut V, node: &ProgramNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit_statements_node(&node.statements());
}

/// The default visitor implementation for a `RangeNode` node.
pub fn visit_range_node<'pr, V>(visitor: &mut V, node: &RangeNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.left() {
        visitor.visit(&node);
    }
    if let Some(node) = node.right() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `RationalNode` node.
pub const fn visit_rational_node<'pr, V>(_visitor: &mut V, _node: &RationalNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `RedoNode` node.
pub const fn visit_redo_node<'pr, V>(_visitor: &mut V, _node: &RedoNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `RegularExpressionNode` node.
pub const fn visit_regular_expression_node<'pr, V>(_visitor: &mut V, _node: &RegularExpressionNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `RequiredKeywordParameterNode` node.
pub const fn visit_required_keyword_parameter_node<'pr, V>(_visitor: &mut V, _node: &RequiredKeywordParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `RequiredParameterNode` node.
pub const fn visit_required_parameter_node<'pr, V>(_visitor: &mut V, _node: &RequiredParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `RescueModifierNode` node.
pub fn visit_rescue_modifier_node<'pr, V>(visitor: &mut V, node: &RescueModifierNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.expression());
    visitor.visit(&node.rescue_expression());
}

/// The default visitor implementation for a `RescueNode` node.
pub fn visit_rescue_node<'pr, V>(visitor: &mut V, node: &RescueNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.exceptions() {
        visitor.visit(&node);
    }
    if let Some(node) = node.reference() {
        visitor.visit(&node);
    }
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
    if let Some(node) = node.subsequent() {
        visitor.visit_rescue_node(&node);
    }
}

/// The default visitor implementation for a `RestParameterNode` node.
pub const fn visit_rest_parameter_node<'pr, V>(_visitor: &mut V, _node: &RestParameterNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `RetryNode` node.
pub const fn visit_retry_node<'pr, V>(_visitor: &mut V, _node: &RetryNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ReturnNode` node.
pub fn visit_return_node<'pr, V>(visitor: &mut V, node: &ReturnNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
}

/// The default visitor implementation for a `SelfNode` node.
pub const fn visit_self_node<'pr, V>(_visitor: &mut V, _node: &SelfNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `ShareableConstantNode` node.
pub fn visit_shareable_constant_node<'pr, V>(visitor: &mut V, node: &ShareableConstantNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.write());
}

/// The default visitor implementation for a `SingletonClassNode` node.
pub fn visit_singleton_class_node<'pr, V>(visitor: &mut V, node: &SingletonClassNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.expression());
    if let Some(node) = node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `SourceEncodingNode` node.
pub const fn visit_source_encoding_node<'pr, V>(_visitor: &mut V, _node: &SourceEncodingNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `SourceFileNode` node.
pub const fn visit_source_file_node<'pr, V>(_visitor: &mut V, _node: &SourceFileNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `SourceLineNode` node.
pub const fn visit_source_line_node<'pr, V>(_visitor: &mut V, _node: &SourceLineNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `SplatNode` node.
pub fn visit_splat_node<'pr, V>(visitor: &mut V, node: &SplatNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.expression() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `StatementsNode` node.
pub fn visit_statements_node<'pr, V>(visitor: &mut V, node: &StatementsNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.body() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `StringNode` node.
pub const fn visit_string_node<'pr, V>(_visitor: &mut V, _node: &StringNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `SuperNode` node.
pub fn visit_super_node<'pr, V>(visitor: &mut V, node: &SuperNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
    if let Some(node) = node.block() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `SymbolNode` node.
pub const fn visit_symbol_node<'pr, V>(_visitor: &mut V, _node: &SymbolNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `TrueNode` node.
pub const fn visit_true_node<'pr, V>(_visitor: &mut V, _node: &TrueNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `UndefNode` node.
pub fn visit_undef_node<'pr, V>(visitor: &mut V, node: &UndefNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.names() {
        visitor.visit(&node);
    }
}

/// The default visitor implementation for a `UnlessNode` node.
pub fn visit_unless_node<'pr, V>(visitor: &mut V, node: &UnlessNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.predicate());
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
    if let Some(node) = node.else_clause() {
        visitor.visit_else_node(&node);
    }
}

/// The default visitor implementation for a `UntilNode` node.
pub fn visit_until_node<'pr, V>(visitor: &mut V, node: &UntilNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.predicate());
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `WhenNode` node.
pub fn visit_when_node<'pr, V>(visitor: &mut V, node: &WhenNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    for node in &node.conditions() {
        visitor.visit(&node);
    }
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `WhileNode` node.
pub fn visit_while_node<'pr, V>(visitor: &mut V, node: &WhileNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    visitor.visit(&node.predicate());
    if let Some(node) = node.statements() {
        visitor.visit_statements_node(&node);
    }
}

/// The default visitor implementation for a `XStringNode` node.
pub const fn visit_x_string_node<'pr, V>(_visitor: &mut V, _node: &XStringNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{}

/// The default visitor implementation for a `YieldNode` node.
pub fn visit_yield_node<'pr, V>(visitor: &mut V, node: &YieldNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    if let Some(node) = node.arguments() {
        visitor.visit_arguments_node(&node);
    }
}
