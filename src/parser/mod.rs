//! ```bnf
//! @entry := <program>
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
//! <trait_declaration> := "pub"? "trait" <IDENTIFIER> "{" ( ( <function_signature> ";" ) | <function_declaration> )* "}"
//! <import_declaration> := "import" <IDENTIFIER> "from" <STRING_LITERAL> ";"
//! 
//! <type_literal> := "int" | "float" | "char" | "bool" | <IDENTIFY>  "<" <type_literal>* ">"
//! <function_signature> := "fn" <IDENTIFIER> "(" ( <IDENTIFIER> ":" <type_literal> )* ")" ( ":" <type_literal> )?
//! <identifier> := <IDENTIFIER> ( ( "." | "::" ) <identifier> )*
//! <statement> := 
//!     <expression> ";"
//!   | ( "let" | "const" ) <IDENTIFIER> ( ":" <type_literal> )? ( "=" <expression> )? ";"
//!   | <identifier> "=" <expression> ";"
//!   | <identifier> "(" <expression>* ")" ";"
//! <expression> := 
//!     <unary_ope> <expression>
//!   | <expression> <binary_ope> <expression>
//! <unary_ope> := "-"
//! <binary_ope> := "+" | "-" | "/" | "*"
//! ```

pub mod parser;

// 識別子（変数名、関数名など）
type Identifier = String;

// 型リテラル
#[derive(Debug, PartialEq)]
pub enum TypeLiteral {
    Int,
    Float,
    Char,
    Bool,
    Custom(Identifier, Vec<TypeLiteral>),
}

// 演算子
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Divide,
    Multiply,
}

// 式
#[derive(Debug, PartialEq)]
pub enum Expression {
    // 整数、浮動小数点数、文字、文字列、真偽値などのリテラル
    Literal(Literal),
    // 識別子（変数参照など）
    Identifier(Identifier),
    // 単項演算
    UnaryOp(UnaryOp, Box<Expression>),
    // 二項演算
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
    // 関数呼び出し
    FunctionCall(Identifier, Vec<Expression>),
}

// リテラル
#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Char(char),
    Bool(bool),
    String(String),
}

// 文
#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    // 変数宣言: let/const name: Type = expr;
    VariableDeclaration {
        is_const: bool,
        name: Identifier,
        type_hint: Option<TypeLiteral>,
        initializer: Option<Expression>,
    },
    // 代入: name = expr;
    Assignment {
        target: Identifier,
        value: Expression,
    },
    // 関数呼び出し文: name(args);
    FunctionCall(Identifier, Vec<Expression>),
}

// 関数シグネチャ
#[derive(Debug, PartialEq)]
pub struct FunctionSignature {
    pub name: Identifier,
    pub params: Vec<(Identifier, TypeLiteral)>,
    pub return_type: Option<TypeLiteral>,
}

// 宣言
#[derive(Debug, PartialEq)]
pub enum Declaration {
    StaticVariable {
        is_pub: bool,
        name: Identifier,
        type_hint: TypeLiteral,
        initializer: Expression,
    },
    Namespace {
        is_pub: bool,
        name: Identifier,
        body: Program,
    },
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    Trait(TraitDeclaration),
    Import {
        module: Identifier,
        from: String,
    },
}

// 関数宣言
#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub is_pub: bool,
    pub signature: FunctionSignature,
    pub body: Vec<Statement>,
}

// フィールド宣言（クラス内）
#[derive(Debug, PartialEq)]
pub struct FieldDeclaration {
    pub is_pub: bool,
    pub name: Identifier,
    pub type_hint: TypeLiteral,
}

// クラス宣言
#[derive(Debug, PartialEq)]
pub struct ClassDeclaration {
    pub is_pub: bool,
    pub name: Identifier,
    // 関数またはフィールドの宣言のリスト
    pub members: Vec<ClassMember>,
}

// クラスのメンバ
#[derive(Debug, PartialEq)]
pub enum ClassMember {
    Function(FunctionDeclaration),
    Field(FieldDeclaration),
}

// トレイト宣言
#[derive(Debug, PartialEq)]
pub struct TraitDeclaration {
    pub is_pub: bool,
    pub name: Identifier,
    // 関数シグネチャまたは関数宣言のリスト
    pub methods: Vec<TraitMethod>,
}

// トレイトのメソッド
#[derive(Debug, PartialEq)]
pub enum TraitMethod {
    Signature(FunctionSignature),
    Definition(FunctionDeclaration),
}

// プログラム全体
#[derive(Debug, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}
