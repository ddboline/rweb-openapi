//! Schema specification for [OpenAPI 3.0.0](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.0.md)

use crate::{
    v3_0::components::{Components, ObjectOrReference},
    Str,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

/// top level document
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Spec {
    /// This string MUST be the [semantic version number](https://semver.org/spec/v2.0.0.html)
    /// of the
    /// [OpenAPI Specification version](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#versions)
    /// that the OpenAPI document uses. The `openapi` field SHOULD be used by tooling
    /// specifications and clients to interpret the OpenAPI document. This is not related to
    /// the API
    /// [`info.version`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#infoVersion)
    /// string.
    pub openapi: Str,
    /// Provides metadata about the API. The metadata MAY be used by tooling as required.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server.
    /// If the `servers` property is not provided, or is an empty array, the default value would
    /// be a
    /// [Server Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverObject)
    /// with a
    /// [url](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverUrl)
    /// value of `/`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,

    /// Holds the relative paths to the individual endpoints and their operations. The path is
    /// appended to the URL from the
    /// [`Server Object`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverObject)
    /// in order to construct the full URL. The Paths MAY be empty, due to
    /// [ACL constraints](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#securityFiltering).
    pub paths: IndexMap<Str, PathItem>,

    /// An element to hold various schemas for the specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    /// A declaration of which security mechanisms can be used across the API.
    /// The list of  values includes alternative security requirement objects that can be used.
    /// Only one of the security requirement objects need to be satisfied to authorize a request.
    /// Individual operations can override this definition.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,

    /// A list of tags used by the specification with additional metadata.
    ///The order of the tags can be used to reflect on their order by the parsing tools.
    /// Not all tags that are used by the
    /// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject)
    /// must be declared. The tags that are not declared MAY be organized randomly or
    /// based on the tools' logic. Each tag name in the list MUST be unique.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,

    /// Additional external documentation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// General information about the API.
///
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#infoObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
// #[serde(rename_all = "lowercase")]
pub struct Info {
    /// The title of the application.
    pub title: Str,
    /// A short description of the application. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    /// A URL to the Terms of Service for the API. MUST be in the format of a URL.
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<Url>,
    /// The version of the OpenAPI document (which is distinct from the [OpenAPI Specification
    /// version](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oasVersion)
    /// or the API implementation version).
    pub version: Str,
    /// The contact information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

/// Contact information for the exposed API.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#contactObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Contact {
    #[serde(skip_serializing_if = "str::is_empty")]
    pub name: Str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    // TODO: Make sure the email is a valid email
    #[serde(skip_serializing_if = "str::is_empty")]
    pub email: Str,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions
}

/// License information for the exposed API.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#licenseObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct License {
    /// The license name used for the API.
    pub name: Str,
    /// A URL to the license used for the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// An object representing a Server.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Server {
    /// A URL to the target host. This URL supports Server Variables and MAY be relative, to
    /// indicate that the host location is relative to the location where the OpenAPI document
    /// is being served. Variable substitutions will be made when a variable is named
    /// in {brackets}.
    pub url: Str,
    /// An optional string describing the host designated by the URL. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    /// A map between a variable name and its value. The value is used for substitution in
    /// the server's URL template.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub variables: IndexMap<Str, ServerVariable>,
}

/// An object representing a Server Variable for server URL template substitution.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#serverVariableObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ServerVariable {
    /// The default value to use for substitution, and to send, if an alternate value is not
    /// supplied. Unlike the Schema Object's default, this value MUST be provided by the consumer.
    pub default: Str,
    /// An enumeration of string values to be used if the substitution options are from a limited
    /// set.
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub substitutions_enum: Vec<Str>,
    /// An optional description for the server variable. [CommonMark] syntax MAY be used for rich
    /// text representation.
    ///
    /// [CommonMark]: https://spec.commonmark.org/
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
}

/// Describes the operations available on a single path.
///
/// A Path Item MAY be empty, due to [ACL
/// constraints](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#securityFiltering).
/// The path itself is still exposed to the documentation viewer but they will not know which
/// operations and parameters are available.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathItem {
    /// Allows for an external definition of this path item. The referenced structure MUST be
    /// in the format of a
    /// [Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#pathItemObject).
    /// If there are conflicts between the referenced definition and this Path Item's definition,
    /// the behavior is undefined.
    #[serde(skip_serializing_if = "str::is_empty", rename = "$ref")]
    pub reference: Str,

    /// An optional, string summary, intended to apply to all operations in this path.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub summary: Str,
    /// An optional, string description, intended to apply to all operations in this path.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,

    /// A definition of a GET operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    /// A definition of a PUT operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    /// A definition of a POST operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    /// A definition of a DELETE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    /// A definition of a OPTIONS operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    /// A definition of a HEAD operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    /// A definition of a PATCH operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    /// A definition of a TRACE operation on this path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<Operation>,

    /// An alternative `server` array to service all operations in this path.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,

    /// A list of parameters that are applicable for all the operations described under this
    /// path. These parameters can be overridden at the operation level, but cannot be removed
    /// there. The list MUST NOT include duplicated parameters. A unique parameter is defined by
    /// a combination of a
    /// [name](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterName)
    /// and
    /// [location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn).
    /// The list can use the
    /// [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#referenceObject)
    /// to link to parameters that are defined at the
    /// [OpenAPI Object's components/parameters](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsParameters).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ObjectOrReference<Parameter>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// Describes a single API operation on a path.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
// #[serde(rename_all = "lowercase")]
pub struct Operation {
    /// A list of tags for API documentation control. Tags can be used for logical grouping of
    /// operations by resources or any other qualifier.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Str>,
    /// A short summary of what the operation does.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub summary: Str,
    /// A verbose explanation of the operation behavior.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    /// Additional external documentation for this operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalDocs")]
    pub external_docs: Option<ExternalDoc>,
    /// Unique string used to identify the operation. The id MUST be unique among all operations
    /// described in the API. Tools and libraries MAY use the operationId to uniquely identify an
    /// operation, therefore, it is RECOMMENDED to follow common programming naming conventions.
    #[serde(skip_serializing_if = "str::is_empty", rename = "operationId")]
    pub operation_id: Str,

    /// A list of parameters that are applicable for this operation. If a parameter is already
    /// defined at the
    /// [Path Item](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#pathItemParameters),
    /// the new definition will override it but can never remove it. The list MUST NOT
    /// include duplicated parameters. A unique parameter is defined by a combination of a
    /// [name](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterName)
    /// and
    /// [location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn).
    /// The list can use the
    /// [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#referenceObject)
    /// to link to parameters that are defined at the
    /// [OpenAPI Object's components/parameters](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsParameters).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ObjectOrReference<Parameter>>,

    /// The request body applicable for this operation. The requestBody is only supported in HTTP methods where the HTTP 1.1 specification RFC7231 has explicitly defined semantics for request bodies. In other cases where the HTTP spec is vague, requestBody SHALL be ignored by consumers.
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBody")]
    pub request_body: Option<ObjectOrReference<RequestBody>>,

    /// The list of possible responses as they are returned from executing this operation.
    ///
    /// A container for the expected responses of an operation. The container maps a HTTP
    /// response code to the expected response.
    ///
    /// The documentation is not necessarily expected to cover all possible HTTP response codes
    /// because they may not be known in advance. However, documentation is expected to cover
    /// a successful operation response and any known errors.
    ///
    /// The `default` MAY be used as a default response object for all HTTP codes that are not
    /// covered individually by the specification.
    ///
    /// The `Responses Object` MUST contain at least one response code, and it SHOULD be the
    /// response for a successful operation call.
    ///
    /// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#responsesObject>.
    pub responses: IndexMap<Str, Response>,

    /// A map of possible out-of band callbacks related to the parent operation. The key is
    /// a unique identifier for the Callback Object. Each value in the map is a
    /// [Callback Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#callbackObject)
    /// that describes a request that may be initiated by the API provider and the
    /// expected responses. The key value used to identify the callback object is
    /// an expression, evaluated at runtime, that identifies a URL to use for the
    /// callback operation.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub callbacks: IndexMap<Str, Callback>,

    /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage
    /// of the declared operation. Default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// A declaration of which security mechanisms can be used for this operation. The list of
    /// values includes alternative security requirement objects that can be used. Only one
    /// of the security requirement objects need to be satisfied to authorize a request.
    /// This definition overrides any declared top-level
    /// [`security`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oasSecurity).
    /// To remove a top-level security declaration, an empty array can be used.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<SecurityRequirement>,

    /// An alternative `server` array to service this operation. If an alternative `server`
    /// object is specified at the Path Item Object or Root level, it will be overridden by
    /// this value.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<Server>,
}

/// Describes a single operation parameter.
/// A unique parameter is defined by a combination of a
/// [name](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterName)
/// and [location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn).
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// The name of the parameter. Parameter names are case sensitive.
    pub name: Str,

    /// The location of the parameter. Possible values are `"query"`, `"header"`, `"path"` or `"cookie"`.
    #[serde(rename = "in")]
    pub location: Location,

    /// A brief description of the parameter. This could contain examples of use. 
    /// [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,

    /// Determines whether this parameter is mandatory.
    /// If the [parameter location](#parameterIn) is `"path"`, this property is **REQUIRED** and its value MUST be `true`.
    /// Otherwise, the property MAY be included and its default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage.
    /// Default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    // /// Sets the ability to pass empty-valued parameters.
    // /// This is valid only for `query` parameters and allows sending a parameter with an empty value.
    // /// Default value is `false`.
    // /// If [`style`](#parameterStyle) is used, and if behavior is `n/a` (cannot be serialized), the value of `allowEmptyValue` SHALL be ignored.
    // /// Use of this property is NOT RECOMMENDED, as it is likely to be removed in a later revision.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub allowEmptyValue: Option<bool>,

    /// Describes how the parameter value will be serialized depending on the type of the parameter
    /// value. Default values (based on value of in): for `query` - `form`; for `path` - `simple`; for
    /// `header` - `simple`; for cookie - `form`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ParameterStyle>,

    /// When this is true, parameter values of type `array` or `object` generate separate parameters for each value of the array or key-value pair of the map. 
    /// For other types of parameters this property has no effect. 
    /// When [`style`] is `form`, the default value is `true`.
    /// For all other styles, the default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// Determines whether the parameter value SHOULD allow reserved characters, as defined by [RFC3986](https://tools.ietf.org/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=` to be included without percent-encoding.
    /// This property only applies to parameters with an `in` value of `query`.
    /// The default value is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,

    /// The schema defining the type used for the parameter or a map containing the representations for the parameter.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub representation: Option<ParameterRepresentation>,

    /// Example(s) of the parameter's potential value
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub example: Option<ParameterExamples>,
}

/// The schema defining the type used for the parameter or a map containing the representations for the parameter.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParameterRepresentation {
    Simple {
        /// The schema defining the type used for the parameter.
        schema: ComponentOrInlineSchema,
    },
    Content {
        /// A map containing the representations for the parameter. The key is the media type and the value describes it. The map MUST only contain one entry.
        content: IndexMap<Str, MediaType>,
    }
}

/// Example(s) of the parameter's potential value
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParameterExamples {
    One {
        /// Example of the parameter's potential value.
        /// The example SHOULD match the specified schema and encoding properties if present.
        /// The `example` field is mutually exclusive of the `examples` field. Furthermore, if referencing a `schema` that contains an example, the `example` value SHALL _override_ the example provided by the schema.
        /// To represent examples of media types that cannot naturally be represented in JSON or YAML, a string value can contain the example with escaping where necessary.
        example: Option<serde_json::Value>,
    },
    Multiple {
        /// Examples of the parameter's potential value.
        /// Each example SHOULD contain a value in the correct format as specified in the parameter encoding.
        /// The `examples` field is mutually exclusive of the `example` field.
        /// Furthermore, if referencing a `schema` that contains an example, the `examples` value SHALL _override_ the example provided by the schema.
        examples: IndexMap<Str, ObjectOrReference<Example>>,
    },
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Type {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "file")]
    File,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Location {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "formData")]
    FormData,
}

/// Just for convenience.
impl Default for Location {
    fn default() -> Self {
        Location::Query
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ParameterStyle {
    Matrix,
    Label,
    Form,
    Simple,
    SpaceDelimited,
    PipeDelimited,
    DeepObject,
}

mod component_ser_as_ref {
    use super::*;
    use serde::*;

    const PATH_REF_PREFIX: &str = "#/components/schemas/"; 

    pub fn serialize<S: Serializer>(component: &Str, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&(PATH_REF_PREFIX.to_string() + component))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deser: D) -> Result<Str, D::Error> {
        let s = String::deserialize(deser)?;
        if let Some(s) = s.strip_prefix(PATH_REF_PREFIX) {
            Ok(Str::Owned(s.to_string()))
        } else {
            Err(de::Error::custom("not a component schema reference path"))
        }
    }
}

/// Either a reference to a component schema
/// or an \[inline\] schema itself.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ComponentOrInlineSchema {
    Component {
        /// Name of the component schema.
        /// 
        /// Serialized as [JSON reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03)
        /// path to the definition within the specification document
        #[serde(rename = "$ref", serialize_with = "component_ser_as_ref::serialize", deserialize_with = "component_ser_as_ref::deserialize")]
        name: Str
    },
    Inline(Schema),
    // Add "ExtRef" variant if support for externally referenced schemas (neither inline nor components) is needed
}
impl ComponentOrInlineSchema {
    /// Unwrap inlined
    pub fn unwrap(&self) -> Option<&Schema> {
        match self {
            Self::Inline(s) => Some(s),
            Self::Component{..} => None,
        }
    }
}

/// The Schema Object allows the definition of input and output data types.
/// These types can be objects, but also primitives and arrays.
/// This object is an extended subset of the
/// [JSON Schema Specification Wright Draft 00](http://json-schema.org/).
/// For more information about the properties, see
/// [JSON Schema Core](https://tools.ietf.org/html/draft-wright-json-schema-00) and
/// [JSON Schema Validation](https://tools.ietf.org/html/draft-wright-json-schema-validation-00).
/// Unless stated otherwise, the property definitions follow the JSON Schema.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    // This is from 3.0.1; Differs for 3.1
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    // The following properties are taken directly from the JSON Schema definition and follow the same specifications:
    // - multipleOf
    // - maximum
    // - exclusiveMaximum
    // - minimum
    // - exclusiveMinimum
    // - maxLength
    // - minLength
    // - pattern (This string SHOULD be a valid regular expression, according to the ECMA 262 regular expression dialect)
    // - maxItems
    // - minItems
    // - uniqueItems
    // - maxProperties
    // - minProperties
    // - required
    // - enum
    //
    // The following properties are taken from the JSON Schema definition but their definitions were adjusted to the OpenAPI Specification.
    // - type - Value MUST be a string. Multiple types via an array are not supported.
    // - allOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - oneOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - anyOf - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - not - Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - items - Value MUST be an object and not an array. Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema. `items` MUST be present if the `type` is `array`.
    // - properties - Property definitions MUST be a [Schema Object](#schemaObject) and not a standard JSON Schema (inline or referenced).
    // - additionalProperties - Value can be boolean or object. Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard JSON Schema.
    // - description - [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    // - format - See [Data Type Formats](#dataTypeFormat) for further details. While relying on JSON Schema's defined formats, the OAS offers a few additional predefined formats.
    // - default - The default value represents what would be assumed by the consumer of the input as the value of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the defined type for the Schema Object defined at the same level. For example, if `type` is `string`, then `default` can be `"foo"` but cannot be `1`.

    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,

    #[serde(skip_serializing_if = "str::is_empty")]
    pub format: Str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<ComponentOrInlineSchema>>,

    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<Str, ComponentOrInlineSchema>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,

    /// Value can be boolean or object. Inline or referenced schema MUST be of a
    /// [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#schemaObject)
    /// and not a standard JSON Schema.
    ///
    /// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#properties>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<ComponentOrInlineSchema>>,

    /// A free-form property to include an example of an instance for this schema.
    /// To represent examples that cannot be naturally represented in JSON or YAML,
    /// a string value can be used to contain the example with escaping where necessary.
    /// NOTE: According to [spec], _Primitive data types in the OAS are based on the
    ///       types supported by the JSON Schema Specification Wright Draft 00._
    ///       This suggest using
    ///       [`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html). [spec][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#data-types]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::value::Value>,

    #[serde(skip_serializing_if = "str::is_empty")]
    pub title: Str,

    /// The default value represents what would be assumed by the consumer of the input as the value
    /// of the schema if one is not provided. Unlike JSON Schema, the value MUST conform to the
    /// defined type for the Schema Object defined at the same level. For example, if type is
    /// `string`, then `default` can be `"foo"` but cannot be `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<ComponentOrInlineSchema>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<ComponentOrInlineSchema>,

    /// Inline or referenced schema MUST be of a [Schema Object](#schemaObject) and not a standard
    /// JSON Schema.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<ComponentOrInlineSchema>,


    // JSON Schema Validation
    // TODO: fetch up descriptions from https://json-schema.org/draft/2020-12/json-schema-validation.html

    // Any
    
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<Type>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_values: Vec<Str>,
    #[serde(rename = "const", skip_serializing_if = "Option::is_none")]
    pub const_value: Option<serde_json::Value>,

    // Numbers

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<serde_json::Value>,

    // Strings

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub pattern: Str,

    // Arrays

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,

    // Objects

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<Str>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub dependent_required: IndexMap<Str, Vec<Str>>,
}

/// Describes a single response from an API Operation, including design-time, static `links`
/// to operations based on the response.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#responseObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    /// A short description of the response.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    pub description: Str,

    /// Maps a header name to its definition.
    /// [RFC7230](https://tools.ietf.org/html/rfc7230#page-22) states header names are case
    /// insensitive. If a response header is defined with the name `"Content-Type"`, it SHALL
    /// be ignored.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<Str, ObjectOrReference<Header>>,

    /// A map containing descriptions of potential response payloads. The key is a media type
    /// or [media type range](https://tools.ietf.org/html/rfc7231#appendix-D) and the value
    /// describes it. For responses that match multiple keys, only the most specific key is
    /// applicable. e.g. text/plain overrides text/*
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub content: IndexMap<Str, MediaType>,

    /// A map of operations links that can be followed from the response. The key of the map
    /// is a short name for the link, following the naming constraints of the names for
    /// [Component Objects](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#componentsObject).
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub links: IndexMap<Str, ObjectOrReference<Link>>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// The Header Object follows the structure of the
/// [Parameter Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterObject)
/// with the following changes:
/// 1. `name` MUST NOT be specified, it is given in the corresponding `headers` map.
/// 1. `in` MUST NOT be specified, it is implicitly in `header`.
/// 1. All traits that are affected by the location MUST be applicable to a location of
///    `header` (for example, [`style`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterStyle)).
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#headerObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Header {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ComponentOrInlineSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    /// string, number, boolean, integer, array, file ( only for formData )
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub param_type: Option<Type>,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub format: Str,
    /// A brief description of the parameter. This could contain examples
    /// of use.  GitHub Flavored Markdown is allowed.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    // collectionFormat: ???
    // default: ???
    // maximum ?
    // exclusiveMaximum ??
    // minimum ??
    // exclusiveMinimum ??
    // maxLength ??
    // minLength ??
    // pattern ??
    // maxItems ??
    // minItems ??
    // enum ??
    // multipleOf ??
    // allowEmptyValue ( for query / body params )
}

/// Describes a single request body.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#requestBodyObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct RequestBody {
    /// A brief description of the request body. This could contain examples of use.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,

    /// The content of the request body. The key is a media type or
    /// [media type range](https://tools.ietf.org/html/rfc7231#appendix-D) and the
    /// value describes it. For requests that match multiple keys, only the most specific key
    /// is applicable. e.g. text/plain overrides text/*
    pub content: IndexMap<Str, MediaType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// The Link object represents a possible design-time link for a response.
///
/// The presence of a link does not guarantee the caller's ability to successfully invoke it,
/// rather it provides a known relationship and traversal mechanism between responses and
/// other operations.
///
/// Unlike _dynamic_ links (i.e. links provided *in* the response payload), the OAS linking
/// mechanism does not require link information in the runtime response.
///
/// For computing links, and providing instructions to execute them, a
/// [runtime expression](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#runtimeExpression)
/// is used for accessing values in an operation and using them as parameters while invoking
/// the linked operation.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#linkObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Link {
    #[serde(flatten)]
    operation: LinkOperation,

    /// A map representing parameters to pass to an operation as specified with `operationId`
    /// or identified via `operationRef`. The key is the parameter name to be used, whereas
    /// the value can be a constant or an expression to be evaluated and passed to the
    /// linked operation. The parameter name can be qualified using the
    /// [parameter location](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterIn)
    /// `[{in}.]{name}` for operations that use the same parameter name in different
    /// locations (e.g. path.id).
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    parameters: IndexMap<Str, RuntimeExpressionOrValue>,

    /// A literal value or
    /// [{expression}](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#runtimeExpression)
    /// to use as a request body when calling the target operation.
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    request_body: Option<RuntimeExpressionOrValue>,

    /// A description of the link. [CommonMark syntax](http://spec.commonmark.org/) MAY be
    /// used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    description: Str,

    /// A server object to be used by the target operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<Server>,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtension
}

/// Runtime expression or literal value. Used for Link `parameters` and `request_body`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RuntimeExpressionOrValue {
    RuntimeExpression(Str),
    LiteralValue(serde_json::Value),
}

/// The name of an _existing_ resolvable OAS operation, or a relative or absolute reference to an OAS operation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum LinkOperation {
    Id {
        /// The name of an _existing_, resolvable OAS operation, as defined with a unique
        /// `operationId`. This field is mutually exclusive of the `operationRef` field.
        #[serde(rename = "operationId")]
        operation_id: Str,
    },
    Ref {
        /// A relative or absolute reference to an OAS operation. This field is mutually exclusive
        /// of the `operationId` field, and MUST point to an
        /// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject).
        /// Relative `operationRef` values MAY be used to locate an existing
        /// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject)
        /// in the OpenAPI definition.
        #[serde(rename = "operationRef")]
        operation_ref: Str,
    },
}

/// Each Media Type Object provides schema and examples for the media type identified by its key.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#media-type-object>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct MediaType {
    /// The schema defining the type used for the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ComponentOrInlineSchema>,

    /// Example of the media type.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub examples: Option<MediaTypeExample>,

    /// A map between a property name and its encoding information. The key, being the
    /// property name, MUST exist in the schema as a property. The encoding object SHALL
    /// only apply to `requestBody` objects when the media type is `multipart`
    /// or `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub encoding: IndexMap<Str, Encoding>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum MediaTypeExample {
    /// Example of the media type. The example object SHOULD be in the correct format as
    /// specified by the media type. The `example` field is mutually exclusive of the
    /// `examples` field. Furthermore, if referencing a `schema` which contains an example,
    /// the `example` value SHALL override the example provided by the schema.
    Example { example: serde_json::Value },
    /// Examples of the media type. Each example object SHOULD match the media type and
    /// specified schema if present. The `examples` field is mutually exclusive of
    /// the `example` field. Furthermore, if referencing a `schema` which contains an
    /// example, the `examples` value SHALL override the example provided by the schema.
    Examples {
        examples: IndexMap<Str, ObjectOrReference<Example>>,
    },
}

/// A single encoding definition applied to a single schema property.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Encoding {
    /// The Content-Type for encoding a specific property. Default value depends on the
    /// property type: for `string` with `format` being `binary` – `application/octet-stream`;
    /// for other primitive types – `text/plain`; for `object` - `application/json`;
    /// for `array` – the default is defined based on the inner type. The value can be a
    /// specific media type (e.g. `application/json`), a wildcard media type
    /// (e.g. `image/*`), or a comma-separated list of the two types.
    #[serde(skip_serializing_if = "str::is_empty", rename = "contentType")]
    pub content_type: Str,

    /// A map allowing additional information to be provided as headers, for example
    /// `Content-Disposition`.  `Content-Type` is described separately and SHALL be
    /// ignored in this section. This property SHALL be ignored if the request body
    /// media type is not a `multipart`.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<Str, ObjectOrReference<Header>>,

    /// Describes how a specific property value will be serialized depending on its type.
    /// See [Parameter Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterObject)
    /// for details on the
    /// [`style`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#parameterStyle)
    /// property. The behavior follows the same values as `query` parameters, including
    /// default values. This property SHALL be ignored if the request body media type
    /// is not `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub style: Str,

    /// When this is true, property values of type `array` or `object` generate
    /// separate parameters for each value of the array, or key-value-pair of the map.
    /// For other types of properties this property has no effect. When
    /// [`style`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#encodingStyle)
    /// is `form`, the default value is `true`. For all other styles, the default value
    /// is `false`. This property SHALL be ignored if the request body media type is
    /// not `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,

    /// Determines whether the parameter value SHOULD allow reserved characters, as defined
    /// by [RFC3986](https://tools.ietf.org/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=`
    /// to be included without percent-encoding. The default value is `false`. This
    /// property SHALL be ignored if the request body media type is
    /// not `application/x-www-form-urlencoded`.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
}

/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#exampleObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub summary: Str,

    /// Long description for the example.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    /// Embedded literal example or a URL that points to the literal example.
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub value: Option<ExampleValue>,

    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// Embedded literal example or a URL that points to the literal example.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ExampleValue {
    Embedded {
        /// Embedded literal example. The `value` field and `externalValue` field are mutually
        /// exclusive. To represent examples of media types that cannot naturally represented
        /// in JSON or YAML, use a string value to contain the example, escaping where necessary.
        value: serde_json::Value
    },
    External {
        /// A URL that points to the literal example. This provides the capability to reference
        /// examples that cannot easily be included in JSON or YAML documents. The `value` field
        /// and `externalValue` field are mutually exclusive.
        #[serde(rename = "externalValue")]
        external_value: Str
    },
}

/// Defines a security scheme that can be used by the operations. Supported schemes are
/// HTTP authentication, an API key (either as a header or as a query parameter),
///OAuth2's common flows (implicit, password, application and access code) as defined
/// in [RFC6749](https://tools.ietf.org/html/rfc6749), and
/// [OpenID Connect Discovery](https://tools.ietf.org/html/draft-ietf-oauth-discovery-06).
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#securitySchemeObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: Str,
        #[serde(rename = "in")]
        location: Str,
    },
    #[serde(rename = "http")]
    Http {
        scheme: Str,
        #[serde(rename = "bearerFormat")]
        bearer_format: Str,
    },
    #[serde(rename = "oauth2")]
    OAuth2 { flows: Flows },
    #[serde(rename = "openIdConnect")]
    OpenIdConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: Str,
    },
}

/// Allows configuration of the supported OAuth Flows.
/// See [link]
/// [link][https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oauth-flows-object]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Flows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<ImplicitFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<PasswordFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<ClientCredentialsFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<AuthorizationCodeFlow>,
}

/// Configuration details for a implicit OAuth Flow
/// See [link]
/// [link](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oauth-flow-object)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitFlow {
    pub authorization_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,
    pub scopes: IndexMap<Str, Str>,
}

/// Configuration details for a password OAuth Flow
/// See [link]
/// [link](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oauth-flow-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PasswordFlow {
    token_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,
    pub scopes: IndexMap<Str, Str>,
}

/// Configuration details for a client credentials OAuth Flow
/// See [link]
/// [link](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oauth-flow-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsFlow {
    token_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,
    pub scopes: IndexMap<Str, Str>,
}

/// Configuration details for a authorization code OAuth Flow
/// See [link]
/// [link](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#oauth-flow-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeFlow {
    pub authorization_url: Url,
    token_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<Url>,
    pub scopes: IndexMap<Str, Str>,
}

// TODO: Implement
/// A map of possible out-of band callbacks related to the parent operation. Each value in
/// the map is a Path Item Object that describes a set of requests that may be initiated by
/// the API provider and the expected responses. The key value used to identify the callback
/// object is an expression, evaluated at runtime, that identifies a URL to use for the
/// callback operation.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#callbackObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Callback(
    /// A Path Item Object used to define a callback request and expected responses.
    serde_json::Value, // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
);

/// # [Security Requirement Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.1.0.md#securityRequirementObject)
/// Lists the required security schemes to execute this operation.
/// The name used for each property MUST correspond to a security scheme declared in the [Security Schemes](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.1.0.md#componentsSecuritySchemes) under the [Components Object](#componentsObject).
///
/// Security Requirement Objects that contain multiple schemes require that all schemes MUST be satisfied for a request to be authorized.
/// This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.
///
/// When a list of Security Requirement Objects is defined on the [OpenAPI Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.1.0.md#oasObject) or [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.1.0.md#operationObject), only one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
pub type SecurityRequirement = IndexMap<Str, Vec<Str>>;

/// Adds metadata to a single tag that is used by the
/// [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#operationObject).
/// It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#tagObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Tag {
    /// The name of the tag.
    pub name: Str,

    /// A short description for the tag.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    // /// Additional external documentation for this tag.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub external_docs: Option<Vec<ExternalDoc>>,

    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

/// Allows referencing an external resource for extended documentation.
///
/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#externalDocumentationObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDoc {
    /// The URL for the target documentation.
    pub url: Url,

    /// A short description of the target documentation.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub description: Str,
    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_scheme_oauth_deser() {
        const IMPLICIT_OAUTH2_SAMPLE: &str = r#"{
          "type": "oauth2",
          "flows": {
            "implicit": {
              "authorizationUrl": "https://example.com/api/oauth/dialog",
              "scopes": {
                "write:pets": "modify pets in your account",
                "read:pets": "read your pets"
              }
            },
            "authorizationCode": {
              "authorizationUrl": "https://example.com/api/oauth/dialog",
              "tokenUrl": "https://example.com/api/oauth/token",
              "scopes": {
                "write:pets": "modify pets in your account",
                "read:pets": "read your pets"
              }
            }
          }
        }"#;
        let obj: SecurityScheme = serde_json::from_str(&IMPLICIT_OAUTH2_SAMPLE).unwrap();
        match obj {
            SecurityScheme::OAuth2 { flows } => {
                assert!(flows.implicit.is_some());
                let implicit = flows.implicit.unwrap();
                assert_eq!(
                    implicit.authorization_url,
                    Url::parse("https://example.com/api/oauth/dialog").unwrap()
                );
                assert!(implicit.scopes.contains_key("write:pets"));
                assert!(implicit.scopes.contains_key("read:pets"));

                assert!(flows.authorization_code.is_some());
                let auth_code = flows.authorization_code.unwrap();
                assert_eq!(
                    auth_code.authorization_url,
                    Url::parse("https://example.com/api/oauth/dialog").unwrap()
                );
                assert_eq!(
                    auth_code.token_url,
                    Url::parse("https://example.com/api/oauth/token").unwrap()
                );
                assert!(implicit.scopes.contains_key("write:pets"));
                assert!(implicit.scopes.contains_key("read:pets"));
            }
            _ => assert!(false, "wrong security scheme type"),
        }
    }
}
