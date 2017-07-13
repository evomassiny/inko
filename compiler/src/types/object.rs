use mutability::Mutability;
use rc_cell::RcCell;
use symbol::RcSymbol;
use symbol_table::SymbolTable;
use types::Type;

#[derive(Debug)]
pub struct Object {
    /// The name of the object, if any.
    pub name: Option<String>,

    /// The attributes defined on this object.
    pub attributes: SymbolTable,

    /// The methods defined on this object.
    pub methods: SymbolTable,

    /// The traits this object implements.
    pub implemented_traits: Vec<RcSymbol>,

    /// The type arguments this object may have.
    pub type_arguments: SymbolTable,

    /// The prototype of this object.
    pub prototype: Option<RcCell<Object>>,
}

impl Object {
    pub fn new() -> RcCell<Object> {
        RcCell::new(Object {
            name: None,
            attributes: SymbolTable::new(),
            methods: SymbolTable::new(),
            implemented_traits: Vec::new(),
            type_arguments: SymbolTable::new(),
            prototype: None,
        })
    }

    pub fn with_name(name: &str) -> RcCell<Object> {
        RcCell::new(Object {
            name: Some(name.to_string()),
            attributes: SymbolTable::new(),
            methods: SymbolTable::new(),
            implemented_traits: Vec::new(),
            type_arguments: SymbolTable::new(),
            prototype: None,
        })
    }

    pub fn define_immutable_attribute<T: ToString>(
        &mut self,
        name: T,
        kind: Type,
    ) {
        self.attributes.define(name, kind, Mutability::Immutable);
    }
}