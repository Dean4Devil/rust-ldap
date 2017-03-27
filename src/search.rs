use std::io;
use std::collections::HashMap;

use asnom::structure::StructureTag;
use asnom::structures::{Tag, Sequence, Integer, OctetString, Boolean};
use asnom::common::TagClass::*;

use rfc4515::parse;

use futures::{Future, stream, Stream};
use tokio_service::Service;

use ldap::Ldap;
use service::{LdapMessage, LdapMessageStream};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scope {
    BaseObject   = 0,
    SingleLevel  = 1,
    WholeSubtree = 2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DerefAliases {
    Never             = 0,
    InSearch          = 1,
    FindingBaseObject = 2,
    Always            = 3,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SearchEntry {
    Reference(Vec<String>),
    Object {
        object_name: String,
        attributes: HashMap<String, Vec<String>>,
    },
}

impl SearchEntry {
    pub fn construct(tag: Tag) -> SearchEntry {
        match tag {
            Tag::StructureTag(t) => {
                match t.id {
                    // Search Result Entry
                    // Search Result Done (if the result set is empty)
                    4|5 => {
                        let mut tags = t.expect_constructed().unwrap();
                        let attributes = tags.pop().unwrap();
                        let object_name = tags.pop().unwrap();
                        let object_name = String::from_utf8(object_name.expect_primitive().unwrap()).unwrap();

                        let a = construct_attributes(attributes.expect_constructed().unwrap_or(vec![])).unwrap();

                        SearchEntry::Object {
                            object_name: object_name,
                            attributes: a,
                        }
                    },
                    // Search Result Reference
                    19 => {
                        // TODO actually handle this case
                        SearchEntry::Reference(vec![])
                    },
                    _ => panic!("Search received a non-search tag!"),
                }
            }
            _ => unimplemented!()
        }
    }
}

fn construct_attributes(tags: Vec<StructureTag>) -> Option<HashMap<String, Vec<String>>> {
    let mut map = HashMap::new();
    for tag in tags.into_iter() {
        let mut inner = tag.expect_constructed().unwrap();

        let values = inner.pop().unwrap();
        let valuev = values.expect_constructed().unwrap()
                           .into_iter()
                           .map(|t| t.expect_primitive().unwrap())
                           .map(|v| String::from_utf8(v).unwrap())
                           .collect();
        let key = inner.pop().unwrap();
        let keystr = String::from_utf8(key.expect_primitive().unwrap()).unwrap();

        map.insert(keystr, valuev);
    }

    Some(map)
}

impl Ldap {
    pub fn search(&self,
                    base: String,
                    scope: Scope,
                    deref: DerefAliases,
                    typesonly: bool,
                    filter: String,
                    attrs: Vec<String>) ->
        Box<Future<Item = Vec<SearchEntry>, Error = io::Error>> {
        let req = Tag::Sequence(Sequence {
            id: 3,
            class: Application,
            inner: vec![
                   Tag::OctetString(OctetString {
                       inner: base.into_bytes(),
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: scope as i64,
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: deref as i64,
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: 0,
                       .. Default::default()
                   }),
                   Tag::Integer(Integer {
                       inner: 0,
                       .. Default::default()
                   }),
                   Tag::Boolean(Boolean {
                       inner: typesonly,
                       .. Default::default()
                   }),
                   parse(&filter).unwrap(),
                   Tag::Sequence(Sequence {
                       inner: attrs.into_iter().map(|s|
                            Tag::OctetString(OctetString { inner: s.into_bytes(), ..Default::default() })).collect(),
                       .. Default::default()
                   })
            ],
        });

        let fut = self.call(req).and_then(|res| {
            let ostr = match res {
                LdapMessage::Stream(first, body) => {
                    let fstr = stream::once(Ok(first));
                    fstr.chain(body)
                },
                LdapMessage::Once(first) => {
                    let fstr = stream::once(Ok(first));
                    fstr.chain(LdapMessageStream::empty())
                },
            };
            ostr.map(|x| SearchEntry::construct(x))
                .collect()
                .and_then(|x| Ok(x))
        });

        Box::new(fut)
    }
}

