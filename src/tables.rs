#![allow(dead_code)]

use crate::codes::*;
use crate::database::*;
use crate::error::*;
use crate::flags::*;
use std::io::Result;

#[derive(PartialEq)]
pub enum Category {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
    Contract,
}

// TOOD: should just be the table trait
pub(crate) trait TableRange<'a> {
    // TODO: maybe use Rust's range parameter syntax here to combine these into one function
    fn range(db: &'a Database, first: u32, last: u32) -> Self;
    fn rest(db: &'a Database, first: u32) -> Self;
}

macro_rules! table {
    ($snake:ident, $camel:ident) => {
        #[derive(Copy, Clone)]
        pub struct $camel<'a> {
            pub(crate) db: &'a Database,
            pub(crate) first: u32,
            pub(crate) last: u32,
        }
        impl<'a> Iterator for $camel<'a> {
            type Item = $camel<'a>;
            fn next(&mut self) -> Option<$camel<'a>> {
                if self.first >= self.last {
                    return None;
                }
                let result = Some(*self);
                self.first += 1;
                result
            }
        }
        impl<'a> TableRange<'a> for $camel<'a> {
            fn range(db: &'a Database, first: u32, last: u32) -> $camel<'a> {
                $camel { db, first, last }
            }
            fn rest(db: &'a Database, first: u32) -> $camel<'a> {
                $camel { db, first, last: db.$snake.rows() }
            }
        }
        impl<'a> $camel<'a> {
            pub(crate) fn new(db: &'a Database, index: u32) -> $camel<'a> {
                $camel { db, first: index, last: index + 1 }
            }
            fn len(&self) -> u32 {
                self.last - self.first
            }
            fn u32(&self, column: u32) -> Result<u32> {
                self.db.u32(&self.db.$snake, self.first, column)
            }
            fn str(&self, column: u32) -> Result<&'a str> {
                self.db.str(&self.db.$snake, self.first, column)
            }
            fn list<T: TableRange<'a>>(&self, column: u32) -> Result<T> {
                let first = self.u32(column)? - 1;

                if self.first + 1 < self.db.$snake.rows() {
                    Ok(T::range(self.db, first, self.db.u32(&self.db.$snake, self.first + 1, column)? - 1))
                } else {
                    Ok(T::rest(self.db, first))
                }
            }
        }
    };
}

table!(type_ref, TypeRef);
impl<'a> TypeRef<'a> {
    pub fn name(&self) -> Result<&'a str> {
        self.str(1)
    }
    pub fn namespace(&self) -> Result<&'a str> {
        self.str(2)
    }
}

table!(generic_param_constraint, GenericParamConstraint);
table!(type_spec, TypeSpec);

table!(type_def, TypeDef);
impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> Result<TypeAttributes> {
        Ok(TypeAttributes(self.u32(0)?))
    }
    pub fn name(&self) -> Result<&'a str> {
        self.str(1)
    }
    pub fn namespace(&self) -> Result<&'a str> {
        self.str(2)
    }
    pub fn extends(&self) -> Result<TypeDefOrRef> {
        Ok(TypeDefOrRef::decode(&self.db, self.u32(3)?))
    }
    pub fn methods(&self) -> Result<MethodDef> {
        self.list::<MethodDef>(5)
    }

    pub fn attributes(&self) -> Result<CustomAttribute<'a>> {
        let parent = HasCustomAttribute::TypeDef(*self);
        let (first, last) = self.db.equal_range(&self.db.custom_attribute, 0, self.db.custom_attribute.rows(), 0, parent.encode())?;
        Ok(CustomAttribute::range(self.db, first, last))
    }

    pub fn category(&self) -> Result<Category> {
        if self.flags()?.interface() {
            return Ok(Category::Interface);
        }
        match self.extends()?.name()? {
            "Enum" => Ok(Category::Enum),
            "ValueType" => {
                // TODO: check when it has ApiContractAttribute and then return Category::Contract
                Ok(Category::Struct)
            }
            "MulticastDelegate" => Ok(Category::Delegate),
            "Attribute" => Ok(Category::Attribute),
            _ => Ok(Category::Class),
        }
    }
}

table!(custom_attribute, CustomAttribute);
impl<'a> CustomAttribute<'a> {
    pub fn parent(&self) -> Result<HasCustomAttribute> {
        Ok(HasCustomAttribute::decode(&self.db, self.u32(0)?))
    }
    pub fn class(&self) -> Result<CustomAttributeType> {
        Ok(CustomAttributeType::decode(&self.db, self.u32(1)?))
    }
    // value() -> Result<CustomAttributeSig>

    pub fn has_name(&self, namespace: &str, name: &str) -> Result<bool> {
        Ok(match self.class()? {
            CustomAttributeType::MethodDef(row) => {
                 let parent = row.parent()?;
                 name == parent.name()? && namespace == parent.namespace()?
            },
            CustomAttributeType::MemberRef(row) => match row.class()? {
                MemberRefParent::TypeDef(row) => name == row.name()? && namespace == row.namespace()?,
                MemberRefParent::TypeRef(row) => name == row.name()? && namespace == row.namespace()?,
                _ => false,
            },
            _ => false,
        })
    }
}

table!(method_def, MethodDef);
impl<'a> MethodDef<'a> {
    pub fn name(&self) -> Result<&'a str> {
        self.str(3)
    }
    pub fn parent(&self) -> Result<TypeDef> {
        let last = self.db.type_def.rows();
        let first = self.db.upper_bound(&self.db.type_def, 0, last, 6, self.first)?;
        Ok(TypeDef::range(self.db, first, last))
    }
}

table!(member_ref, MemberRef);
impl<'a> MemberRef<'a> {
    pub fn class(&self) -> Result<MemberRefParent> {
        Ok(MemberRefParent::decode(&self.db, self.u32(0)?))
    }
    pub fn name(&self) -> Result<&'a str> {
        self.str(1)
    }
    // pub fun signature(&self) {}
}

table!(module, Module);
table!(param, Param);
table!(interface_impl, InterfaceImpl);
table!(constant, Constant);
table!(field, Field);
table!(field_marshal, FieldMarshal);
table!(decl_security, DeclSecurity);
table!(class_layout, ClassLayout);
table!(field_layout, FieldLayout);
table!(standalone_sig, StandaloneSig);
table!(event_map, EventMap);
table!(event, Event);
table!(property_map, PropertyMap);
table!(property, Property);
table!(method_semantics, MethodSemantics);
table!(method_impl, MethodImpl);
table!(module_ref, ModuleRef);
table!(impl_map, ImplMap);
table!(field_rva, FieldRva);
table!(assembly, Assembly);
table!(assembly_processor, AssemblyProcessor);
table!(assembly_os, AssemblyOs);
table!(assembly_ref, AssemblyRef);
table!(assembly_ref_processor, AssemblyRefProcessor);
table!(assembly_ref_os, AssemblyRefOs);
table!(file, File);
table!(exported_type, ExportedType);
table!(manifest_resource, ManifestResource);
table!(nested_class, NestedClass);
table!(generic_param, GenericParam);
table!(method_spec, MethodSpec);
