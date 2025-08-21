//! ```bnf
//! @entry := <program>
//! 
//! # top level
//! <program> := <declaration>*
//! <declaration> := 
//!     <static_variable_declaration>
//!   | <namespace_declaration>
//!   | <function_declaration>
//!   | <class_declaration>
//!   | <trait_declaration>
//!   | <import_declaration>
//! <static_variable_declaration> := "pub"? "static" <IDENTIFIER> ":" <type_literal> "=" <expression> ";"
//! <namespace_declaration> := "pub"? "namespace" <IDENTIFIER> "{" <program> "}"
//! <function_declaration> := "pub"? <function_signature> "{" <statement>* "}"
//! <class_declaration> := "pub"? "class" <IDENTIFIER> "{" ( <function_declaration> | <field_declaration> )* "}"
//! <field_declaration> := "pub"? <IDENTIFIER> ":" <type_literal> ";"
//! <trait_declaration> := "pub"? "trait" <IDENTIFIER> "{" ( ( <function_signature> ) | <function_declaration> )* "}"
//! <import_declaration> := "import" <IDENTIFIER> | ( "{" <IDENTIFIER>* "}" ) "from" <STRING_LITERAL> ";"
//! 
//! # statement level
//! <statement> := 
//!     <expression> ";"
//!   | ( "let" | "const" ) <pattern> ( ":" <type_literal> )? ( "=" <expression> )? ";"
//!   | <accesser> "=" <expression> ";"
//!   | <accesser> "(" ( <expression> "," )* <expression>? ")" ";"
//!   | <for_statement>
//!   | <while_statement>
//! <for_statement> := "for" <pattern> "in" <expression> "{" <statement>* "}"
//! <while_statement> := "while" <expression> "{" <statement>* "}"
//! 
//! # expression level
//! <expression> := 
//!     <unary_result>
//!   | <binary_result>
//!   | <if_expression>
//!   | <match_expression>
//!   | <block_expression>
//!   | <literal>
//! <unary_result> := <unary_ope> <expression>
//! <binary_result> := <expression> <binary_ope> <expression>
//! <unary_ope> := "-"
//! <binary_ope> := "+" | "-" | "/" | "*"
//! <if_expression> := "if" <expression> <statement>* ( "else" <expression> )?
//! <match_expression> := "match" <expression> "{" ( <pattern> "=>" <expression> "," )* "}"
//! <block_expression> := "{" <statement>* "}"
//! 
//! # atomic level
//! <type_literal> := "int" | "float" | "char" | "bool" | <IDENTIFY> ( "<" <type_literal>* ">" )? | "(" ( <type_literal> "," )* <type_literal>? ")"
//! <function_signature> := "fn" <IDENTIFIER> "(" ( <pattern> ":" <type_literal> "," )* ( <pattern> ":" <type_literal> )? ")" ( ":" <type_literal> )?
//! <accesser> := <IDENTIFIER> ( ( "." | "::" ) <accesser> )*
//! <literal> := <STRING_LITERAL> | <INT_LITERAL> | <FLOAT_LITERAL> | <BOOL_LITERAL> | <struct_literal> | 
//! 
//! # pattern
//! <pattern> := 
//!     <literal-pattern>
//!   | <identifier-pattern>
//!   | <wildcard-pattern>
//!   | <tuple-pattern>
//!   | <struct-pattern>
//!   | <enum-pattern>
//!   | <reference-pattern>
//!   | <slice-pattern>
//!   | <range-pattern>
//!   | <or-pattern>
//! 
//! <literal-pattern> := <literal>
//! <identifier-pattern> := <IDENTIFIER>
//! <wildcard-pattern> := "_"
//! <tuple-pattern> := "(" ( <pattern> "," )* <pattern>? ")"
//! <struct-pattern> := <accesser>
//! <enum-pattern> := 
//! <reference-pattern> := 
//! <slice-pattern> := 
//! <range-pattern> := 
//! <or-pattern> := 
//! ```

pub mod parser;
pub mod error;

type Identifier = String;

/// <program> := <declaration>*
pub struct Program {
    declarations: Vec<Declaration>,
}


/// <declaration> := 
///     <static_variable_declaration>
///   | <namespace_declaration>
///   | <function_declaration>
///   | <class_declaration>
///   | <trait_declaration>
///   | <import_declaration>
pub enum Declaration {
    StaticVariable(StaticVariableDeclaration),
    Namespace(NamespaceDeclaration),
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    Trait(TraitDeclaration),
    Import(ImportDeclaration),
}

/// <static_variable_declaration> := "pub"? "static" <IDENTIFIER> ":" <type_literal> "=" <expression> ";"
struct StaticVariableDeclaration {
    is_pub: bool,
    name: Identifier,
    type_annotation: Option<TypeLiteral>,
    value: Expression,
}

/// <namespace_declaration> := "pub"? "namespace" <IDENTIFIER> "{" <program> "}"
struct NamespaceDeclaration {
    name: Identifier,
    inner: Program,
}

/// <function_declaration> := "pub"? <function_signature> "{" <statement>* "}"
struct FunctionDeclaration {
    is_pub: bool,
    name: Identifier,
    signature: FunctionSignature,
    inner: Vec<Statement>
}

/// <class_declaration> := "pub"? "class" <IDENTIFIER> "{" ( <function_declaration> | <field_declaration> )* "}"
struct ClassDeclaration {
    is_pub: bool,
    fields: Vec<FieldDeclaration>,
    methods: Vec<FunctionDeclaration>,
}

/// <field_declaration> := "pub"? <IDENTIFIER> ":" <type_literal> ";"
pub struct FieldDeclaration {
    is_pub: bool,
    name: Identifier,
    type_annotation: TypeLiteral,
}

/// <trait_declaration> := "pub"? "trait" <IDENTIFIER> "{" ( ( <function_signature> ) | <function_declaration> )* "}"
struct TraitDeclaration {
    is_pub: bool,
    signatures: Vec<FunctionSignature>,
    functions: Vec<FunctionDeclaration>,
}

/// <import_declaration> := "import" <IDENTIFIER> "from" <STRING_LITERAL> ";"
struct ImportDeclaration {
    name: Identifier,
    path: String,
}

/// <type_literal> := "int" | "float" | "char" | "bool" | <IDENTIFY> ( "<" <type_literal>* ">" )? | "(" ( <type_literal> "," )* <type_literal>? ")"
pub enum TypeLiteral {
    IntType,
    FloatType,
    CharType,
    BoolType,
    CustomType {
        name: Identifier,
        type_args: Vec<TypeLiteral>,
    },
    TupleType {
        inner: Vec<TypeLiteral>,
    }
}

pub struct Expression;

/// <function_signature> := "fn" <IDENTIFIER> "(" ( <pattern> ":" <type_literal> "," )* ( <pattern> ":" <type_literal> )? ")" ( ":" <type_literal> )?
pub struct FunctionSignature;
pub struct Statement;