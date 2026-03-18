// A port of the c-api-hello-world example:
// https://github.com/WebAssembly/binaryen/blob/899e44e2c3dd40795c843a91cfc326e11d64cab0/test/example/c-api-hello-world.c

use binaryen::{
    expression::operation::Operation,
    function::Function,
    module::Module,
    type_::{Type, Types},
};

#[test]
fn c_api_hello_world() {
    let mut module = Module::new();

    // Create a function type for  i32 (i32, i32)
    let ii = vec![Type::i32(), Type::i32()];
    let mut types = Types::default();
    let params = types.create(ii);
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
    let _adder = Function::add(&mut module, "adder", params, results, &add);

    // Print it out
    module.print();
}
