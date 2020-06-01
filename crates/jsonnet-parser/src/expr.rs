use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldName {
	/// {fixed: 2}
	Fixed(String),
	/// {["dyn"+"amic"]: 3}
	Dyn(LocExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
	/// :
	Normal,
	/// ::
	Hidden,
	/// :::
	Unhide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssertStmt(pub LocExpr, pub Option<LocExpr>);

#[derive(Debug, Clone, PartialEq)]
pub struct FieldMember {
	pub name: FieldName,
	pub plus: bool,
	pub params: Option<ParamsDesc>,
	pub visibility: Visibility,
	pub value: LocExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Member {
	Field(FieldMember),
	BindStmt(BindSpec),
	AssertStmt(AssertStmt),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOpType {
	Plus,
	Minus,
	BitNot,
	Not,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOpType {
	Mul,
	Div,
	Mod,

	Add,
	Sub,

	Lhs,
	Rhs,

	Lt,
	Gt,
	Lte,
	Gte,

	In,

	Eq,
	Ne,

	BitAnd,
	BitOr,
	BitXor,

	And,
	Or,
}

/// name, default value
#[derive(Debug, Clone, PartialEq)]
pub struct Param(pub String, pub Option<LocExpr>);
/// Defined function parameters
#[derive(Debug, Clone, PartialEq)]
pub struct ParamsDesc(pub Vec<Param>);
impl ParamsDesc {
	pub fn with_defaults(&self) -> Vec<Param> {
		self.0.iter().filter(|e| e.1.is_some()).cloned().collect()
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arg(pub Option<String>, pub LocExpr);
#[derive(Debug, Clone, PartialEq)]
pub struct ArgsDesc(pub Vec<Arg>);

#[derive(Debug, Clone, PartialEq)]
pub struct BindSpec {
	pub name: String,
	pub params: Option<ParamsDesc>,
	pub value: LocExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfSpecData(pub LocExpr);
#[derive(Debug, Clone, PartialEq)]
pub struct ForSpecData(pub String, pub LocExpr);

#[derive(Debug, Clone, PartialEq)]
pub enum CompSpec {
	IfSpec(IfSpecData),
	ForSpec(ForSpecData),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjBody {
	MemberList(Vec<Member>),
	ObjComp {
		pre_locals: Vec<BindSpec>,
		key: LocExpr,
		value: LocExpr,
		post_locals: Vec<BindSpec>,
		first: ForSpecData,
		rest: Vec<CompSpec>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
	This,
	Super,
	Dollar,
	Null,
	True,
	False,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliceDesc {
	pub start: Option<LocExpr>,
	pub end: Option<LocExpr>,
	pub step: Option<LocExpr>,
}

/// Syntax base
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Literal(LiteralType),

	/// String value: "hello"
	Str(String),
	/// Number: 1, 2.0, 2e+20
	Num(f64),
	/// Variable name: test
	Var(String),

	/// Array of expressions: [1, 2, "Hello"]
	Arr(Vec<LocExpr>),
	/// Array comprehension:
	/// ```jsonnet
	///  ingredients: [
	///    { kind: kind, qty: 4 / 3 }
	///    for kind in [
	///      'Honey Syrup',
	///      'Lemon Juice',
	///      'Farmers Gin',
	///    ]
	///  ],
	/// ```
	ArrComp(LocExpr, ForSpecData, Vec<CompSpec>),

	/// Object: {a: 2}
	Obj(ObjBody),
	/// Object extension: var1 {b: 2}
	ObjExtend(LocExpr, ObjBody),

	/// (obj)
	Parened(LocExpr),

	/// Params in function definition
	/// hello, world, test = 2
	Params(ParamsDesc),
	/// Args in function call
	/// 2 + 2, 3, named = 6
	Args(ArgsDesc),

	/// -2
	UnaryOp(UnaryOpType, LocExpr),
	/// 2 - 2
	BinaryOp(LocExpr, BinaryOpType, LocExpr),
	/// assert 2 == 2 : "Math is broken"
	AssertExpr(AssertStmt, LocExpr),
	/// local a = 2; { b: a }
	LocalExpr(Vec<BindSpec>, LocExpr),

	/// a = 3
	Bind(BindSpec),
	/// import "hello"
	Import(String),
	/// importStr "file.txt"
	ImportStr(String),
	/// error "I'm broken"
	Error(LocExpr),
	/// a(b, c)
	Apply(LocExpr, ArgsDesc),
	///
	Select(LocExpr, String),
	/// a[b]
	Index(LocExpr, LocExpr),
	/// a[1::2]
	Slice(LocExpr, SliceDesc),
	/// function(x) x
	Function(ParamsDesc, LocExpr),
	/// if true == false then 1 else 2
	IfElse {
		cond: IfSpecData,
		cond_then: LocExpr,
		cond_else: Option<LocExpr>,
	},
	/// if 2 = 3
	IfSpec(IfSpecData),
	/// for elem in array
	ForSpec(ForSpecData),
}

/// file, begin offset, end offset
#[derive(Clone, PartialEq)]
pub struct ExprLocation(pub String, pub usize, pub usize);
impl Debug for ExprLocation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{:?}-{:?}", self.0, self.1, self.2)
	}
}

/// Holds AST expression and its location in source file+
#[derive(Clone, PartialEq)]
pub struct LocExpr(pub Rc<Expr>, pub Option<Rc<ExprLocation>>);
impl Debug for LocExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?} from {:?}", self.0, self.1)
	}
}

/// Creates LocExpr from Expr and ExprLocation components
#[macro_export]
macro_rules! loc_expr {
	($expr:expr, $need_loc:expr, ($name:expr, $start:expr, $end:expr)) => {
		LocExpr(
			std::rc::Rc::new($expr),
			if $need_loc {
				Some(std::rc::Rc::new(ExprLocation(
					$name.to_owned(),
					$start,
					$end,
				)))
			} else {
				None
				},
			)
	};
}

/// Creates LocExpr without location info
#[macro_export]
macro_rules! loc_expr_todo {
	($expr:expr) => {
		LocExpr(Rc::new($expr), None)
	};
}
