use crate::parser::{MidlAst, MidlDefinition, MidlModule, MidlStruct};

pub fn codegen(ast: &MidlAst) -> String {
  let mut res = String::new();

  for member in &ast.0 {
    match member {
      MidlDefinition::Struct(s) => res.push_str(&codegen_struct(s)),
      MidlDefinition::Module(m) => res.push_str(&codegen_module(m)),
    }
  }

  res.push('\n');
  res
}

fn codegen_struct(stru: &MidlStruct) -> String {
  let mut res = format!("struct {} {{\n", &stru.identifier);

  for member in &stru.members {
    res.push_str(&format!("  {} {};\n", member.get_type(), member.get_name()));
  }

  res.push_str("};\n");
  res
}

fn codegen_module(modu: &MidlModule) -> String {
  let mut res = format!("namespace {} {{\n", &modu.identifier);

  for member in &modu.members {
    match member {
      MidlDefinition::Struct(s) => res.push_str(&codegen_struct(s)),
      MidlDefinition::Module(m) => res.push_str(&codegen_module(m)),
    }
  }

  res.push_str("}\n");
  res
}
