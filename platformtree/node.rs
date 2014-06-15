// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use syntax::codemap::Span;
use std::collections::hashmap::HashMap;
use std::gc::Gc;

#[deriving(Show)]
pub enum AttributeValue {
  UIntValue(uint),
  StrValue(String),
  RefValue(String),
}

#[deriving(Show)]
pub struct Attribute {
  pub value: AttributeValue,
  pub key_span: Span,
  pub value_span: Span,
}

impl Attribute {
  pub fn new(value: AttributeValue, key_span: Span, value_span: Span)
      -> Attribute {
    Attribute {
      value: value,
      key_span: key_span,
      value_span: value_span,
    }
  }
}

#[deriving(Show)]
pub struct Node {
  pub name: Option<String>,
  pub name_span: Span,

  pub path: String,
  pub path_span: Span,

  pub attributes: HashMap<String, Attribute>,

  pub subnodes: Vec<Gc<Node>>,
}

impl Node {
  pub fn new(name: Option<String>, name_span: Span, path: String,
      path_span: Span) -> Node {
    Node {
      name: name,
      name_span: name_span,
      path: path,
      path_span: path_span,
      attributes: HashMap::new(),
      subnodes: Vec::new(),
    }
  }

  pub fn get_string_attr<'a>(&'a self, key: &str) -> Option<&'a String> {
    self.attributes.find(&key.to_str()).and_then(|av| match av.value {
      StrValue(ref s) => Some(s),
      _ => None,
    })
  }

  pub fn get_int_attr(&self, key: &str) -> Option<uint> {
    self.attributes.find(&key.to_str()).and_then(|av| match av.value {
      UIntValue(ref u) => Some(*u),
      _ => None,
    })
  }

  pub fn get_ref_attr<'a>(&'a self, key: &str) -> Option<&'a String> {
    self.attributes.find(&key.to_str()).and_then(|av| match av.value {
      RefValue(ref s) => Some(s),
      _ => None,
    })
  }
}

#[deriving(Show)]
pub struct PlatformTree {
  pub nodes: Vec<Gc<Node>>,
}

impl PlatformTree {
  pub fn new(nodes: Vec<Gc<Node>>) -> PlatformTree {
    PlatformTree {nodes: nodes}
  }
}
