use macro_tools::syn;

pub mod as_mut;
pub mod as_ref;
pub mod deref;
pub mod deref_mut;
pub mod from;
pub mod index;
pub mod index_mut;
pub mod inner_from;
pub mod new;
pub mod not;
pub mod phantom;
pub mod variadic_from;
pub mod ops;

#[ path = "from/field_attributes.rs" ]
pub mod field_attributes;
#[ path = "from/item_attributes.rs" ]
pub mod item_attributes;

enum FieldAccess 
{
    Named ( syn::Ident ),
    Unnamed ( syn::Index ),
}