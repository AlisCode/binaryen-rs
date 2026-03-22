use std::ffi::CString;

use binaryen_sys::bindings::{BinaryenAddActiveElementSegment, BinaryenAddTable, BinaryenTableRef};

use crate::{expression::Expression, module::Module, type_::Type};

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

    pub fn add_active_element_segment(
        &mut self,
        table: &str,
        name: &str,
        func_names: Vec<&str>,
        offset: &Expression,
    ) {
        let module = self.as_inner_mut();
        let table = CString::new(table).expect("No 0-byte allowed in table name");
        let name = CString::new(name).expect("No 0-byte allowed in element segment name");
        let c_func_names: Vec<_> = func_names
            .into_iter()
            .map(|func| CString::new(func).expect("No 0-byte allowed in function name"))
            .collect();
        let mut func_name_ptrs: Vec<_> = c_func_names.iter().map(|func| func.as_ptr()).collect();
        let offset = offset.as_inner();

        unsafe {
            BinaryenAddActiveElementSegment(
                module,
                table.as_ptr(),
                name.as_ptr(),
                func_name_ptrs.as_mut_ptr(),
                func_name_ptrs.len() as u32,
                offset,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::Binaryen, expression::literal::Literal, module::Module, type_::Type};

    #[test]
    fn should_add_table() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();

        let _table = module.add_table("table", 1, 1, Type::funcref());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }

    #[test]
    fn should_add_active_element_segment() {
        Binaryen::set_colors_enabled(false);
        let mut module = Module::new();

        let _table = module.add_table("table", 1, 1, Type::funcref());

        let callee_body = module.expr_builder().const_(Literal::i32(0));
        let _callee = module.add_function(
            "callee",
            Type::none(),
            Type::i32(),
            vec![],
            Some(&callee_body),
        );

        let offset = module.expr_builder().const_(Literal::i32(0));
        module.add_active_element_segment("table", "elem", vec!["callee"], &offset);

        assert!(module.validate());

        let text = module.allocate_and_write_text();
        insta::assert_snapshot!(text);
    }
}
