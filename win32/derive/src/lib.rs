use proc_macro::TokenStream;
use quote::*;
use syn::*;

#[proc_macro_attribute]
pub fn dllexport(_attr: TokenStream, mut tokens: TokenStream) -> TokenStream {
    let input = tokens.clone();

    let func: ItemFn = syn::parse_macro_input!(input);
    let name = func.sig.ident;
    let mut args = Vec::new();
    for arg in func.sig.inputs.iter() {
        let FnArg::Typed(arg) = arg else {
            unimplemented!()
        };
        let Pat::Ident(name) = arg.pat.as_ref() else {
            unimplemented!()
        };
        args.push(&name.ident);
    }

    /*
    *
    pub fn stdcall_GetStdHandle() -> Cont {
        unsafe {
            let stack: *mut u32 = MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32;
            let ret = *stack.add(0);
            MACHINE.regs.eax = GetStdHandle(*stack.add(1));
            MACHINE.regs.esp += 2 * 4;
            (MACHINE.indirect)(ret)
        }
    }
    */
    let wrapper_name = format_ident!("stdcall_{}", name);
    let arg_count = args.len() as u32;
    let args = args
        .iter()
        .enumerate()
        .map(|(i, _)| quote! { *stack.add(#i) });
    let wrapper: TokenStream = quote! {
        fn #wrapper_name() -> Cont { unsafe {
            let stack: *mut u32 = MACHINE.memory.add(MACHINE.regs.esp as usize) as *mut u32;
            let ret = *stack.add(0);
            MACHINE.regs.eax = #name(#(#args),*);
            MACHINE.regs.esp += #arg_count * 4;
            (MACHINE.indirect)(ret)
        } }
    }
    .into();
    //eprintln!("wrapper {}", wrapper);

    tokens.extend(wrapper);
    tokens
}
