use std::collections::HashSet;

use crate::parser::{MidlAst, MidlDefinition};

fn name_pass(ast: &MidlAst) -> Result<(), String> {
  for member in &ast.0 {
    let mut used = HashSet::new();

    match member {
      MidlDefinition::Struct(cur) => {
        for item in &cur.members {
          let name = item.get_name();
          if used.contains(&name) {
            return Err(format!(
              "Duplicate definition of `{name}` in struct `{}`",
              &cur.identifier
            ));
          }
          used.insert(name);
        }
      }
      MidlDefinition::Module(cur) => {
        for item in &cur.members {
          let name = item.get_name();
          if used.contains(&name) {
            return Err(format!(
              "Duplicate definition of `{name}` in module `{}`",
              &cur.identifier
            ));
          }
          used.insert(name);
        }
      }
    }
  }

  Ok(())
}

fn definition_pass(ast: &MidlAst) -> Result<(), String> {
  const BUILTIN_TYPES: [&str; 13] = [
    "boolean",
    "char",
    "string",
    "long double",
    "double",
    "float",
    "long long",
    "int64",
    "long",
    "int32",
    "short",
    "int16",
    "int8",
  ];

  let mut used = HashSet::new();

  for member in &ast.0 {
    match member {
      MidlDefinition::Struct(cur) => {
        for item in &cur.members {
          let typ = item.get_type();

          if BUILTIN_TYPES.contains(&typ.as_ref()) {
            continue;
          }

          if used.contains(&typ) {
            continue;
          }

          return Err(format!(
            "No definition of `{typ}` in struct `{}`",
            &cur.identifier
          ));
        }

        used.insert(cur.identifier.clone());
      }
      _ => {}
    }
  }

  Ok(())
}

fn literal_pass(ast: &MidlAst) -> Result<(), String> {
  for member in &ast.0 {
    match member {
      MidlDefinition::Struct(cur) => {
        for item in &cur.members {
          item.check_member()?;
        }
      }
      MidlDefinition::Module(_) => {}
    }
  }

  Ok(())
}

pub fn analyze(ast: &MidlAst) -> Result<(), String> {
  name_pass(ast)?;
  definition_pass(ast)?;
  literal_pass(ast)?;
  Ok(())
}
