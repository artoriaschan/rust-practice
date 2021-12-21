// 借助proc_macro包提供的编译器接口在代码中读取和操作Rust代码
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// #[derive(HelloMacro)]时，hello_macro_derive函数就会被自动调用。
#[proc_macro_derive(HelloMacro)]
pub fn hello_marco_derive(input: TokenStream) -> TokenStream {
    // 将Rust代码转换为我们能够进行处理的语法树
    let ast = syn::parse(input).unwrap();
    // 构造对应的trait实现
    impl_hello_marco(&ast)
}

fn impl_hello_marco(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // quote!宏允许我们定义那些希望返回的Rust代码。
    let gen = quote! {
        // 它会将我们输入的#name替换为变量name中的值。
        impl HelloMacro for #name {
            fn hello_macro(){
                // 这里使用的stringify!宏是内置在Rust中的，它接收一个Rust表达式，比如1 + 2，并在编译时将这个表达式转换成字符串字面量，比如"1 + 2"。
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
