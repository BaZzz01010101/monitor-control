use std::collections::BTreeMap;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    pub model: Option<String>,
    pub mccs_version: Option<String>,
    pub commands: Vec<u8>,
    vcp: BTreeMap<u8, Vec<u32>>,
}

impl Capabilities {
    pub fn parse(input: &str) -> Result<Self, CapabilityParseError> {
        let mut parser = Parser::new(input);
        let root = parser.parse()?;

        let model = first_atom(group_after_key(&root, "model"));
        let mccs_version = first_atom(group_after_key(&root, "mccs_ver"));
        let commands = group_after_key(&root, "cmds")
            .map(parse_hex_atoms_u8)
            .transpose()?
            .unwrap_or_default();
        let vcp = group_after_key(&root, "vcp")
            .map(parse_vcp_codes)
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            model,
            mccs_version,
            commands,
            vcp,
        })
    }

    pub fn supports_vcp(&self, code: u8) -> bool {
        self.vcp.contains_key(&code)
    }

    pub fn vcp_values(&self, code: u8) -> Option<&[u32]> {
        self.vcp.get(&code).map(Vec::as_slice)
    }

    pub fn vcp_codes(&self) -> impl Iterator<Item = u8> + '_ {
        self.vcp.keys().copied()
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CapabilityParseError {
    #[error("unbalanced capabilities string")]
    Unbalanced,
    #[error("unexpected token after root capabilities group")]
    TrailingTokens,
    #[error("expected root capabilities group")]
    ExpectedRootGroup,
    #[error("invalid hex token `{0}`")]
    InvalidHex(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Atom(String),
    Group(Vec<Item>),
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn parse(&mut self) -> Result<Vec<Item>, CapabilityParseError> {
        self.skip_ws();
        if self.next_char() != Some('(') {
            return Err(CapabilityParseError::ExpectedRootGroup);
        }
        self.pos += 1;
        let root = self.parse_group()?;
        self.skip_ws();
        if self.pos != self.input.len() {
            return Err(CapabilityParseError::TrailingTokens);
        }
        Ok(root)
    }

    fn parse_group(&mut self) -> Result<Vec<Item>, CapabilityParseError> {
        let mut items = Vec::new();

        loop {
            self.skip_ws();
            match self.next_char() {
                Some('(') => {
                    self.pos += 1;
                    items.push(Item::Group(self.parse_group()?));
                }
                Some(')') => {
                    self.pos += 1;
                    return Ok(items);
                }
                Some(_) => items.push(Item::Atom(self.parse_atom())),
                None => return Err(CapabilityParseError::Unbalanced),
            }
        }
    }

    fn parse_atom(&mut self) -> String {
        let start = self.pos;
        while let Some(ch) = self.next_char() {
            if ch.is_whitespace() || ch == '(' || ch == ')' {
                break;
            }
            self.pos += ch.len_utf8();
        }
        self.input[start..self.pos].to_string()
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.next_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.pos += ch.len_utf8();
        }
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }
}

fn group_after_key<'a>(items: &'a [Item], key: &str) -> Option<&'a [Item]> {
    let mut index = 0;
    while index + 1 < items.len() {
        if matches!(&items[index], Item::Atom(atom) if atom == key) {
            if let Item::Group(group) = &items[index + 1] {
                return Some(group);
            }
        }
        index += 1;
    }
    None
}

fn first_atom(items: Option<&[Item]>) -> Option<String> {
    items.and_then(|items| {
        items.iter().find_map(|item| match item {
            Item::Atom(atom) => Some(atom.clone()),
            Item::Group(_) => None,
        })
    })
}

fn parse_hex_atoms_u8(items: &[Item]) -> Result<Vec<u8>, CapabilityParseError> {
    items
        .iter()
        .filter_map(|item| match item {
            Item::Atom(atom) => Some(parse_hex_u32(atom).and_then(|value| {
                u8::try_from(value).map_err(|_| CapabilityParseError::InvalidHex(atom.clone()))
            })),
            Item::Group(_) => None,
        })
        .collect()
}

fn parse_vcp_codes(items: &[Item]) -> Result<BTreeMap<u8, Vec<u32>>, CapabilityParseError> {
    let mut codes = BTreeMap::new();
    let mut index = 0;

    while index < items.len() {
        let Item::Atom(code_atom) = &items[index] else {
            index += 1;
            continue;
        };

        let code = parse_hex_u32(code_atom).and_then(|value| {
            u8::try_from(value).map_err(|_| CapabilityParseError::InvalidHex(code_atom.clone()))
        })?;
        let values = if let Some(Item::Group(value_group)) = items.get(index + 1) {
            index += 1;
            value_group
                .iter()
                .filter_map(|item| match item {
                    Item::Atom(atom) => Some(parse_hex_u32(atom)),
                    Item::Group(_) => None,
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };

        codes.insert(code, values);
        index += 1;
    }

    Ok(codes)
}

fn parse_hex_u32(atom: &str) -> Result<u32, CapabilityParseError> {
    u32::from_str_radix(atom, 16).map_err(|_| CapabilityParseError::InvalidHex(atom.to_string()))
}
