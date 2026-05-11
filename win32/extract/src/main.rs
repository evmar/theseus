use std::collections::{HashMap, HashSet};

use windows_metadata::MethodCallAttributes;

fn convert_typename(
    name: &windows_metadata::TypeName,
    ext: Option<windows_metadata::TypeName>,
) -> String {
    if *name == windows_metadata::TypeName::BOOL {
        "bool".into()
    } else if *name == windows_metadata::TypeName::PSTR {
        "u32 /* STR */".into()
    } else if *name == windows_metadata::TypeName::PWSTR {
        "u32 /* WSTR */".into()
    } else {
        if let Some(ext) = ext {
            if ext == windows_metadata::TypeName::Struct {
                format!("{}", name.name())
            } else if ext == windows_metadata::TypeName::Enum {
                format!("u32 /* {} */", name.name())
            } else if ext == windows_metadata::TypeName::Delegate {
                // e.g. WNDPROC
                format!("u32 /* {} */", name.name())
            } else {
                todo!("{} ({})", name.name(), ext.name())
            }
        } else {
            format!("{}", name.name())
        }
    }
}

fn convert_type(ty: &windows_metadata::Type) -> String {
    use windows_metadata::Type::*;
    match ty {
        Void => format!("()"), // found in e.g void* params
        Bool => todo!(),
        Char => todo!(),
        I8 => format!("i8"),
        U8 => format!("u8"),
        I16 => todo!(),
        U16 => format!("u16"),
        I32 => format!("i32"),
        U32 => format!("u32"),
        I64 => todo!(),
        U64 => todo!(),
        F32 => todo!(),
        F64 => todo!(),
        ISize => format!("i32"),
        USize => format!("u32"),
        String => todo!(),
        Object => todo!(),
        Name(name) => convert_typename(name, None),
        Const(name) => convert_typename(name, None),
        GenericParam(_generic_param) => todo!(),
        TypeDef(type_def, vec) => {
            assert!(vec.is_empty()); // ?
            convert_typename(&type_def.type_name(), type_def.extends())
        }
        MutPtr(ty, x) => {
            if *x != 1 {
                // it appears this is for like lplpFoo types, possibly
                todo!("{ty:?} {x:?}");
            }
            return convert_type(ty);
        }
        ConstPtr(_, _) => todo!(),
        Win32Array(ty, len) => format!("[{}; {}]", convert_type(ty), len),
        WinrtArray(_) => todo!(),
        WinrtArrayRef(_) => todo!(),
        ConstRef(_) => todo!(),
        PrimitiveOrEnum(_, _) => todo!(),
    }
}

fn convert_method(method: &windows_metadata::MethodDef) -> String {
    use std::fmt::Write;
    let mut buf = String::new();

    let sig = method.signature(&[]);
    //writeln!(&mut buf, "{:?}", sig).unwrap();

    if sig.call_flags.contains(MethodCallAttributes::VARARG) {
        writeln!(&mut buf, "// XXX: cdecl").unwrap();
    }
    writeln!(&mut buf, "#[win32_derive::dllexport]").unwrap();
    writeln!(&mut buf, "pub fn {name}(", name = method.name()).unwrap();
    writeln!(&mut buf, "    ctx: &mut Context,").unwrap();
    if sig.call_flags.contains(MethodCallAttributes::HASTHIS) {
        writeln!(&mut buf, "    this: u32,").unwrap();
    }

    // First param is possibly this pointer(?)
    let mut params = method.params().collect::<Vec<_>>();
    let types = &sig.params;

    if params.len() == types.len() + 1 && params[0].name() == "" {
        // ??
        params.remove(0);
    }

    if params.len() != types.len() {
        todo!(
            "params.len() {} != types.len() {}",
            params.len(),
            types.len()
        );
    }

    for (param, ty) in params.iter().zip(types) {
        writeln!(
            &mut buf,
            "    _{name}: {ty},",
            name = param.name(),
            ty = convert_type(ty)
        )
        .unwrap();
    }

    match sig.return_type {
        windows_metadata::Type::Void => writeln!(&mut buf, ") {{ todo!() }}").unwrap(),
        ty => writeln!(&mut buf, ") -> {} {{ todo!() }}", convert_type(&ty)).unwrap(),
    }

    buf
}

fn convert_typedef(type_def: &windows_metadata::TypeDef) {
    println!("struct {name} {{", name = type_def.name());
    for f in type_def.fields() {
        println!(
            "    {name}: {ty},",
            name = f.name(),
            ty = convert_type(&f.ty(None))
        );
    }
    println!("}}");
}

fn main() {
    let bytes = Vec::from(include_bytes!("../Windows.Win32.winmd"));

    let file = windows_metadata::File::new(bytes).expect("Invalid metadata");
    let reader = windows_metadata::Reader::new(vec![file]);

    let mut modules = HashMap::new();
    let search: HashSet<String> = HashSet::from_iter(std::env::args().skip(1));

    for item in reader.items() {
        match item {
            windows_metadata::Item::Type(type_def) => {
                if !search.contains(type_def.name()) {
                    continue;
                }
                convert_typedef(&type_def);
                println!();
            }
            windows_metadata::Item::Const(_field) => {
                //println!("f {n}", n = field.name());
            }
            windows_metadata::Item::Fn(method_def, _) => {
                if !search.contains(method_def.name()) {
                    continue;
                }
                let module = method_def.module_name();
                let body = convert_method(&method_def);
                modules.entry(module).or_insert(vec![]).push(body);
            }
        }
    }

    for (module, bodies) in &modules {
        println!("// {module}");
        for body in bodies {
            println!("{body}");
        }
    }
}
