// TODO: remove this
#![allow(unused)]

use anyhow::{anyhow, bail, Result};
use pest::iterators::Pair;

use super::Rule;

#[derive(Debug)]
pub struct MidlAst(pub Vec<MidlDefinition>);

impl TryFrom<Pair<'_, Rule>> for MidlAst {
  type Error = anyhow::Error;

  fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
    let mut definitions = Vec::new();

    for item in value.into_inner() {
      match item.as_rule() {
        Rule::definition => {
          definitions.push(MidlDefinition::try_from(item)?);
        }
        rule => {
          bail!("Expect rule `definition` but found `{rule:?}`");
        }
      }
    }

    Ok(Self(definitions))
  }
}

#[derive(Debug)]
pub enum MidlDefinition {
  Struct(MidlStruct),
  Module(MidlModule),
}

impl TryFrom<Pair<'_, Rule>> for MidlDefinition {
  type Error = anyhow::Error;

  fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
    let def = value.into_inner().next().ok_or(anyhow!(
      "Expect rule `type_decl` or `module` but found nothing"
    ))?;

    Ok(match def.as_rule() {
      Rule::type_decl => Self::Struct(MidlStruct::try_from(def)?),
      Rule::module => Self::Module(MidlModule::try_from(def)?),
      rule => {
        bail!("Expect rule `type_decl` or `module` but found `{rule:?}`");
      }
    })
  }
}

impl MidlDefinition {
  pub fn get_name(&self) -> String {
    match self {
      MidlDefinition::Struct(s) => s.identifier.clone(),
      MidlDefinition::Module(m) => m.identifier.clone(),
    }
  }
}

#[derive(Debug)]
pub struct MidlStruct {
  pub identifier: String,
  pub members: Vec<MidlStructMember>,
}

impl TryFrom<Pair<'_, Rule>> for MidlStruct {
  type Error = anyhow::Error;

  fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
    let value = value.into_inner().next().unwrap();

    Ok(match value.as_rule() {
      Rule::struct_type => {
        // has members
        let mut pairs = value.into_inner();

        let identifier = pairs
          .next()
          .ok_or(anyhow!("Expect rule `ID` but found nothing"))?
          .as_str()
          .to_owned();

        let mut members = Vec::new();

        for item in pairs {
          match item.as_rule() {
            Rule::member => {
              MidlStructMember::parse(item, &mut members)?;
            }
            rule => {
              bail!("Expect rule `member` but found `{rule:?}`");
            }
          }
        }

        Self {
          identifier,
          members,
        }
      }
      Rule::ID => Self {
        // no members
        identifier: value.as_str().to_owned(),
        members: Vec::new(),
      },
      rule => {
        bail!("Expect rule `struct_type` or `ID` but found `{rule:?}`")
      }
    })
  }
}

#[derive(Debug)]
pub enum MidlStructMember {
  Simple(MidlStructSimpleMember),
  Array(MidlStructArrayMember),
}

impl MidlStructMember {
  fn parse(value: Pair<'_, Rule>, list: &mut Vec<MidlStructMember>) -> Result<()> {
    let mut pairs = value.into_inner();

    let r#type = pairs.next().unwrap().as_str();
    let declarators = pairs.next().unwrap().into_inner();

    for item in declarators {
      let decl = item.into_inner().next().unwrap();
      match decl.as_rule() {
        Rule::array_declarator => {
          list.push(Self::Array(MidlStructArrayMember::parse(decl, r#type)?));
        }
        Rule::simple_declarator => {
          list.push(Self::Simple(MidlStructSimpleMember::parse(decl, r#type)?));
        }
        rule => {
          bail!("Expect rule `array_declarator` or `simple_declarator` but found `{rule:?}`");
        }
      }
    }

    Ok(())
  }

  pub fn get_name(&self) -> String {
    match self {
      MidlStructMember::Simple(s) => s.identifier.clone(),
      MidlStructMember::Array(a) => a.identifier.clone(),
    }
  }

  pub fn get_type(&self) -> String {
    match self {
      MidlStructMember::Simple(s) => s.r#type.clone(),
      MidlStructMember::Array(a) => a.r#type.clone(),
    }
  }

  pub fn check_member(&self) -> Result<(), String> {
    match self {
      MidlStructMember::Simple(s) => {
        if s.r#type == "short" && s.initial_value == Some("'a'".to_owned()) {
          return Err("Literal `a` can not be assigned to type `short`".to_owned());
        }

        Ok(())
      }
      MidlStructMember::Array(_) => Ok(()),
    }
  }
}

// TODO: handle types
#[derive(Debug)]
pub struct MidlStructSimpleMember {
  // r#type: MidlType,
  r#type: String,
  identifier: String,
  // initial_value: Option<MidlValue>,
  initial_value: Option<String>,
}

impl MidlStructSimpleMember {
  fn parse(value: Pair<'_, Rule>, r#type: &str) -> Result<Self> {
    let mut pairs = value.into_inner();

    let identifier = pairs.next().unwrap().as_str().to_owned();
    let initial_value = pairs.next().map(|init| init.as_str().to_owned());

    Ok(Self {
      r#type: r#type.to_owned(),
      identifier,
      initial_value,
    })
  }
}

#[derive(Debug)]
pub struct MidlStructArrayMember {
  // r#type: MidlType,
  r#type: String,
  identifier: String,
  // length: usize,
  length: String,
  // initial_value: Option<Vec<MidlValue>>,
  initial_value: Option<Vec<String>>,
}

impl MidlStructArrayMember {
  fn parse(value: Pair<'_, Rule>, r#type: &str) -> Result<Self> {
    let mut pairs = value.into_inner();

    let identifier = pairs.next().unwrap().as_str().to_owned();
    let length = pairs.next().unwrap().as_str().to_owned();

    let initial_value = match pairs.next() {
      Some(init) => {
        let mut list = Vec::new();

        for item in init.into_inner() {
          list.push(item.as_str().to_owned());
        }

        Some(list)
      }
      None => None,
    };

    Ok(Self {
      r#type: r#type.to_owned(),
      identifier,
      length,
      initial_value,
    })
  }
}

// #[derive(Debug)]
// struct MidlType {}

// #[derive(Debug)]
// struct MidlValue {}

#[derive(Debug)]
pub struct MidlModule {
  pub identifier: String,
  pub members: Vec<MidlDefinition>,
}

impl TryFrom<Pair<'_, Rule>> for MidlModule {
  type Error = anyhow::Error;

  fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
    let mut pairs = value.into_inner();

    let identifier = pairs
      .next()
      .ok_or(anyhow!("Expect rule `ID` but found nothing"))?
      .as_str()
      .to_owned();

    let mut members = Vec::new();

    for item in pairs {
      match item.as_rule() {
        Rule::definition => {
          members.push(MidlDefinition::try_from(item)?);
        }
        rule => {
          bail!("Expect rule `definition` but found `{rule:?}`");
        }
      }
    }

    Ok(Self {
      identifier,
      members,
    })
  }
}
