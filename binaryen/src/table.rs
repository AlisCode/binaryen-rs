use std::ffi::CString;

use binaryen_sys::bindings::{BinaryenAddTable, BinaryenTableRef};

use crate::{module::Module, type_::Type};

#[allow(dead_code)]
pub struct Table(BinaryenTableRef);

impl Table {
    fn add(module: &mut Module, name: &str, initial: u32, maximum: u32, table_type: Type) -> Self {
        let module = module.as_inner_mut();
        let name = CString::new(name).expect("No 0-byte allowed in table name");
        let table_type = table_type.into_inner();

        let table =
            unsafe { BinaryenAddTable(module, name.as_ptr(), initial, maximum, table_type) };
        Table(table)
    }
}

impl Module {
    pub fn add_table(&mut self, name: &str, initial: u32, maximum: u32, table_type: Type) -> Table {
        Table::add(self, name, initial, maximum, table_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::{module::Module, type_::Type};

    #[test]
    fn should_add_table() {
        let mut module = Module::new();

        let _table = module.add_table("table", 1, 1, Type::funcref());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
