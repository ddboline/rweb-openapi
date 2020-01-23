use crate::{
    v3_0::{
        Callback, Example, Header, Link, Parameter, RequestBody, Response, Schema, SecurityScheme,
    },
    Str,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ObjectOrReference<T> {
    Object(T),
    Ref {
        #[serde(rename = "$ref")]
        ref_path: Str,
    },
}

/// Holds a set of reusable objects for different aspects of the OAS.
///
/// All objects defined within the components object will have no effect on the API unless
/// they are explicitly referenced from properties outside the components object.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Components {
    /// An object to hold reusable Schema Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub schemas: BTreeMap<Str, ObjectOrReference<Schema>>,

    /// An object to hold reusable Response Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub responses: BTreeMap<Str, ObjectOrReference<Response>>,

    /// An object to hold reusable Parameter Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<Str, ObjectOrReference<Parameter>>,

    /// An object to hold reusable Example
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub examples: BTreeMap<Str, ObjectOrReference<Example>>,

    /// An object to hold reusable Request Body Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty", rename = "requestBodies")]
    pub request_bodies: BTreeMap<Str, ObjectOrReference<RequestBody>>,

    /// An object to hold reusable Header Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<Str, ObjectOrReference<Header>>,

    /// An object to hold reusable Security Scheme Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty", rename = "securitySchemes")]
    pub security_schemes: BTreeMap<Str, ObjectOrReference<SecurityScheme>>,

    /// An object to hold reusable Link Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<Str, ObjectOrReference<Link>>,

    /// An object to hold reusable Callback Objects.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub callbacks: BTreeMap<Str, ObjectOrReference<Callback>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}
