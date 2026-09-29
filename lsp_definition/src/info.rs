pub type Identifier = String;
pub type Class = String;
pub type ParametersCount = usize;

/// What the cursor points at: a symbol and the way it is used there.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemanticInfo {
    pub symbol: Symbol,
    pub usage: Usage,
}

impl SemanticInfo {
    pub fn new(symbol: Symbol, usage: Usage) -> Self {
        Self { symbol, usage }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Function(Identifier),
    Class(Identifier),
    /// `new` with no class name typed yet.
    AnyClass,
    /// Method or property; `class` is `None` when the object type is unknown.
    Member {
        name: Identifier,
        class: Option<Class>,
    },
}

impl Symbol {
    pub fn name(&self) -> Option<&str> {
        match self {
            Symbol::Function(name) | Symbol::Class(name) | Symbol::Member { name, .. } => {
                Some(name)
            }
            Symbol::AnyClass => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Usage {
    Declaration,
    /// `f(a, b)`, `obj.m(a, b)`
    Call(ParametersCount),
    /// A variable holding a call result: `var x = f(); x`
    CallResult(ParametersCount),
    /// `new A(a, b)`
    New(ParametersCount),
    /// `super(a, b)`; the symbol is the parent class.
    Super(ParametersCount),
    /// `class B extends A`
    Extends,
    /// `this`, `super` or a variable holding an instance.
    Instance,
    /// `obj.prop` read or write.
    Access,
}

impl Usage {
    pub fn arguments(self) -> Option<ParametersCount> {
        match self {
            Usage::Call(n) | Usage::CallResult(n) | Usage::New(n) | Usage::Super(n) => Some(n),
            Usage::Declaration | Usage::Extends | Usage::Instance | Usage::Access => None,
        }
    }
}
