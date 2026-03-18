// A port of the c-api-hello-world example:
// https://github.com/WebAssembly/binaryen/blob/899e44e2c3dd40795c843a91cfc326e11d64cab0/test/example/c-api-hello-world.c

use binaryen::{expression::operation::Operation, module::Module, type_::Type};

#[test]
fn c_api_hello_world() {
    let module = Module::new();

    // Create a function type for  i32 (i32, i32)
    let params = Type::create(vec![Type::i32(), Type::i32()]);
    let results = Type::i32();

    // Get the 0 and 1 arguments, and add them
    let builder = module.expr_builder();
    let x = builder.local_get(0, Type::i32());
    let y = builder.local_get(1, Type::i32());
    let add = builder.binary(Operation::i32_add(), &x, &y);

    // Create the add function
    // Note: no additional local variables
    // Note: no basic blocks here, we are an AST. The function body is just an
    // expression node.
    let _adder = module.add_function("adder", params, results, vec![], &add);

    // Print it out
    module.print();
}
