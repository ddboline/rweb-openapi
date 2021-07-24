use crate::{
    v3_0::{
        Callback, Example, Header, Link, Parameter, RequestBody, Response, Schema, SecurityScheme,
    },
    Str,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub schemas: IndexMap<Str, ObjectOrReference<Schema>>,

    /// An object to hold reusable Response Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub responses: IndexMap<Str, ObjectOrReference<Response>>,

    /// An object to hold reusable Parameter Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<Str, ObjectOrReference<Parameter>>,

    /// An object to hold reusable Example
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub examples: IndexMap<Str, ObjectOrReference<Example>>,

    /// An object to hold reusable Request Body Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty", rename = "requestBodies")]
    pub request_bodies: IndexMap<Str, ObjectOrReference<RequestBody>>,

    /// An object to hold reusable Header Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<Str, ObjectOrReference<Header>>,

    /// An object to hold reusable Security Scheme Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty", rename = "securitySchemes")]
    pub security_schemes: IndexMap<Str, ObjectOrReference<SecurityScheme>>,

    /// An object to hold reusable Link Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub links: IndexMap<Str, ObjectOrReference<Link>>,

    /// An object to hold reusable Callback Objects.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub callbacks: IndexMap<Str, ObjectOrReference<Callback>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}
