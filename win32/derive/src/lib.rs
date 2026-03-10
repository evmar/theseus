use proc_macro::TokenStream;
use quote::*;
use syn::*;

#[proc_macro_attribute]
pub fn dllexport(_attr: TokenStream, mut tokens: TokenStream) -> TokenStream {
    let input = tokens.clone();

    let func: ItemFn = syn::parse_macro_input!(input);
    let name = func.sig.ident;
    let mut args = Vec::new();
    let return_addr = format_ident!("return_addr");
    let u32_ty = syn::parse::<syn::Type>(quote!(u32).into()).unwrap();
    args.push((&return_addr, &u32_ty));
    for arg in func.sig.inputs.iter() {
        let FnArg::Typed(arg) = arg else {
            unimplemented!()
        };
        let Pat::Ident(name) = arg.pat.as_ref() else {
            unimplemented!()
        };
        args.push((&name.ident, &*arg.ty));
    }

    // generate
    //   let foo = read(esp + 0);
    //   let bar = read(esp + 4);
    //   ...
    let fetch_args = {
        let mut v = vec![];
        for (i, (arg, ty)) in args.iter().enumerate() {
            let offset = i as u32 * 4;
            v.push(quote! {
                let #arg = <#ty>::from_abi(MACHINE.memory.read::<u32>(MACHINE.regs.esp + #offset));
            });
        }
        quote!(#(#v)*)
    };

    // generate
    //    println!("00401000 FooBar(baz=3 blah=4)");
    let trace = {
        let fmt_string = {
            let named_args = args
                .iter()
                .map(|(arg, _)| format!("{arg}={{{arg}:x?}}"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{return_addr:08x}} {name}({named_args})")
        };
        quote![println!(#fmt_string);]
    };

    let wrapper_name = format_ident!("stdcall_{}", name);
    let stack_popped = args.len() as u32;
    let mut call_args = args.iter().map(|(arg, _)| arg);
    call_args.next(); // skip return_addr

    let wrapper: TokenStream = quote! {
        pub fn #wrapper_name() -> Cont { unsafe {
            use crate::{ABIReturn, FromABIParam};
            #fetch_args
            #trace
            let ret: ABIReturn = #name(#(#call_args),*).into();
            MACHINE.regs.eax = ret.to_abi_return();
            MACHINE.regs.esp += #stack_popped * 4;
            runtime::indirect(return_addr)
        } }
    }
    .into();
    //eprintln!("wrapper {}", wrapper);

    tokens.extend(wrapper);
    tokens
}
