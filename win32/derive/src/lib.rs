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

    let trace = {
        let fmt_string = {
            let named_args = args
                .iter()
                .map(|arg| format!("{arg}={{:x}}"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{return_addr:08x}} {name}({named_args})")
        };
        let stack_args = args.iter().enumerate().map(|(i, _)| {
            let i = (i as u32 + 1) * 4;
            quote! { MACHINE.memory.read::<u32>(MACHINE.regs.esp + #i) }
        });
        quote![println!(#fmt_string, #(#stack_args),*);]
    };

    let wrapper_name = format_ident!("stdcall_{}", name);
    let stack_popped = args.len() as u32 + 1;
    let stack_args = args.iter().enumerate().map(|(i, _)| {
        let i = (i as u32 + 1) * 4;
        quote! { MACHINE.memory.read(MACHINE.regs.esp + #i ) }
    });

    let wrapper: TokenStream = quote! {
        pub fn #wrapper_name() -> Cont { unsafe {
            let return_addr: u32 =  MACHINE.memory.read(MACHINE.regs.esp);
            #trace
            let ret: ABIReturn = #name(#(#stack_args),*).into();
            MACHINE.regs.eax = ret.0;
            MACHINE.regs.esp += #stack_popped * 4;
            (MACHINE.indirect)(return_addr)
        } }
    }
    .into();
    //eprintln!("wrapper {}", wrapper);

    tokens.extend(wrapper);
    tokens
}
