use reporter::Located;
use strum_macros::EnumIter;

pub type BinaryOp = Located<BinaryOpKind>;

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone, EnumIter)]
pub enum BinaryOpKind {
    Add,
    Multiply,
    Divide,
    Subtract,
    Pow,
    IntDivide,
    Modulo,
    ShiftLeft,
    ShiftRight,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    BoolOr,
    BoolAnd,
    BitOr,
    BitAnd,
    BitXor,
}

impl BinaryOpKind {
    pub fn is_comparator(self) -> bool {
        matches!(
            self,
            BinaryOpKind::Equal
                | BinaryOpKind::NotEqual
                | BinaryOpKind::LessEqual
                | BinaryOpKind::Less
                | BinaryOpKind::Greater
                | BinaryOpKind::GreaterEqual
        )
    }

    /// `==` and `!=`
    pub fn is_equality(self) -> bool {
        matches!(self, BinaryOpKind::Equal | BinaryOpKind::NotEqual)
    }

    /// `+`, `-`, `*`, `/` and `%`
    pub fn is_arithmetic(self) -> bool {
        matches!(
            self,
            BinaryOpKind::Add
                | BinaryOpKind::Subtract
                | BinaryOpKind::Multiply
                | BinaryOpKind::Divide
                | BinaryOpKind::Modulo
        )
    }

    pub fn is_bitwise(self) -> bool {
        matches!(self, BinaryOpKind::BitAnd | BinaryOpKind::BitOr | BinaryOpKind::BitXor)
    }

    pub fn is_bitshift(self) -> bool {
        matches!(self, BinaryOpKind::ShiftLeft | BinaryOpKind::ShiftRight)
    }

    pub fn is_valid_for_field_type(self) -> bool {
        matches!(
            self,
            BinaryOpKind::Add
                | BinaryOpKind::Subtract
                | BinaryOpKind::Multiply
                | BinaryOpKind::Divide
                | BinaryOpKind::Equal
                | BinaryOpKind::NotEqual
        )
    }
}
