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
                let #arg = <#ty>::from_abi(m.memory.read::<u32>(m.regs.esp + #offset));
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
                .skip(1) // skip return_addr
                .map(|(arg, _)| format!("{arg}={{{arg}:x?}}"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{return_addr:08x}} {name}({named_args})")
        };
        quote![log::info!(#fmt_string);]
    };

    let wrapper_name = format_ident!("{}_stdcall", name);
    let stack_popped = args.len() as u32;
    let mut call_args = args.iter().map(|(arg, _)| arg);
    call_args.next(); // skip return_addr

    let wrapper: TokenStream = quote! {
        pub fn #wrapper_name(m: &mut crate::Machine) -> runtime::Cont { unsafe {
            use crate::{ABIReturn, FromABIParam};
            use runtime::*;
            #fetch_args
            #trace
            let ret: ABIReturn = #name(#(#call_args),*).into();
            m.regs.eax = ret.to_abi_return();
            m.regs.esp += #stack_popped * 4;
            runtime::indirect(return_addr)
        } }
    }
    .into();
    //eprintln!("wrapper {}", wrapper);

    tokens.extend(wrapper);
    tokens
}

#[proc_macro_derive(ABIEnum)]
pub fn abi_enum(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let enum_: syn::ItemEnum = syn::parse_macro_input!(item);

    let name = &enum_.ident;
    // If one of the values is negative, match using i32 instead of u32.
    let has_negative = enum_.variants.iter().any(|variant| {
        let num = &variant.discriminant.as_ref().unwrap().1;
        match num {
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                ..
            }) => true,
            _ => false,
        }
    });

    let matches = enum_.variants.iter().map(|variant| {
        let num = &variant.discriminant.as_ref().unwrap().1;
        let sym = &variant.ident;
        quote! {
            #num => #name::#sym,
        }
    });

    let get_value = if has_negative {
        quote!(let value = value as i32;)
    } else {
        quote!()
    };

    quote! {
        impl crate::FromABIParam for #name {
            #[allow(unused)]
            fn from_abi(value: u32) -> Self {
                #get_value
                match value {
                    #(#matches)*
                    _ => todo!()
                }
            }
        }
    }
    .into()
}
