// #![allow(unused_variables)]
// #![allow(dead_code)]

mod codes;
mod database;
mod error;
mod flags;
pub mod reader;
mod tables;
use database::*;
use reader::*;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> std::io::Result<()> {
    let reader = Reader::from_os()?;

    let db = Database::new(r"c:\windows\system32\WinMetadata\Windows.Foundation.winmd")?;

    let types = db.type_def();

    for row in types.iter()
    {
        let t = TypeDef::new(row);
        println!("{}.{}", t.namespace()?, t.name()?);

    }

    // if let Some(t) = reader.find("Windows.Foundation", "IAsyncAction") {
    //     println!(" {}.{}", t.namespace()?, t.name()?);
    //     let mut iter = t.methods()?;
    //     println!("        {}", iter.name()?);
    //     iter.next();
    //     println!("        {}", iter.name()?);
    //     iter.next();
    //     println!("        {}", iter.name()?);
    //     iter.next();

    //     // for m in t.methods() {
    //     //     println!("        {}", m.name()?);
    //     // }
    // }

    // for name in reader.namespaces() {
    //     if name != "Windows.Foundation.Collections" && name != "Windows.Foundation" {
    //         continue;
    //     }
    //     println!("\nnamespace {}", name);
    //     if let Some(types) = reader.types(name) {
    //         for t in types.interfaces() {
    //             println!("    interface {}", t.name()?);
    //             for m in t.methods() {
    //                 println!("        {}", m.name()?);
    //             }
    //         }
    //         for t in types.classes() {
    //             println!("    class {}", t.name()?);
    //         }
    //         for t in types.enums() {
    //             println!("    enum {}", t.name()?);
    //         }
    //         for t in types.structs() {
    //             println!("    struct {}", t.name()?);
    //         }
    //         for t in types.delegates() {
    //             println!("    delegate {}", t.name()?);
    //         }
    //     }
    // }

    Ok(())
}
