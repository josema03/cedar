/*
 * Copyright 2022-2023 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Utility functions and types for JSON interface
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS, JsonSchema)]
#[serde(untagged)]
#[serde(
    expecting = "policies as a concatenated string or multiple policies as a hashmap where the policy Id is the key with no duplicate IDs"
)]
#[ts(export_to = "../cedar-policy-bindings/")]
#[ts(export)]
#[schemars(deny_unknown_fields)]
/// Struct defining the two possible ways to pass a set of policies to `json_is_authorized` and `json_validate`
pub enum PolicySpecification {
    /// provides multiple policies as a concatenated string
    Concatenated(String),
    /// provides multiple policies as a hashmap where the policyId is the key
    #[serde(with = "::serde_with::rust::maps_duplicate_key_is_error")]
    #[schemars(with = "HashMap<String, String>")]
    Map(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "success")]
/// Result of a call to a JSON interface
pub enum InterfaceResult {
    /// The call succeeded
    #[serde(rename = "true")]
    Success {
        /// JSON containing the result of the call
        result: String,
    },
    #[serde(rename = "false")]
    /// The call failed
    Failure {
        /// Whether the failure is "internal".
        ///
        /// An "internal failure" is returned when there is a fault in the
        /// Cedar Rust code, or when there is a problem with the request in
        /// the parts which the Java library is responsible for (e.g. an
        /// unsupported operation).
        ///
        /// By contrast, a "bad request" is returned when there is an issue in the
        /// part of the request supplied by the ultimate user of the library, e.g. a
        /// syntax error in a policy.
        #[serde(rename = "isInternal")]
        is_internal: bool,
        /// String description of the error(s) that led to the failure
        errors: Vec<String>,
    },
}

impl InterfaceResult {
    /// A successful result
    pub fn succeed<T: Serialize>(value: T) -> Self {
        serde_json::to_string(&value).map_or_else(
            |e| Self::fail_internally(format!("error serializing result: {e:}")),
            |result| Self::Success { result },
        )
    }

    /// An "internal failure" result; see docs on [`InterfaceResult::Failure`]
    pub fn fail_internally(message: String) -> Self {
        Self::Failure {
            is_internal: true,
            errors: vec![message],
        }
    }

    /// A failure result that isn't internal; see docs on
    /// `InterfaceResult::Failure`
    pub fn fail_bad_request(errors: Vec<String>) -> Self {
        Self::Failure {
            is_internal: false,
            errors,
        }
    }
}

#[cfg(test)]
pub(crate) fn assert_is_failure(result: &InterfaceResult, internal: bool, err: &str) {
    use cool_asserts::assert_matches;
    use itertools::Itertools;

    assert_matches!(result, InterfaceResult::Failure { is_internal, errors } => {
        assert!(
            errors.iter().exactly_one().unwrap().contains(err),
            "Expected to see error containing `{err}`, but saw {errors:?}");
        assert_eq!(is_internal, &internal, "Unexpected value for `is_internal`");
    });
}

/// Utility Macro used to export JSON schemas
#[macro_export]
macro_rules! generate_json_schema_file {
    ($($x:ident),*) => {

        $(
            paste::paste! {
                #[test]
                fn [<generate_json_schema_for_ $x:lower>]() {
                    use std::io::prelude::*;

                    let _dir = std::fs::create_dir_all("../cedar-policy-json-schemas");

                    let mut file = std::fs::File::create(format!("../cedar-policy-json-schemas/{}.json", stringify!($x))).unwrap();
                    let json_data = serde_json::to_string_pretty(&schemars::schema_for!($x)).unwrap();
                    let _result = file.write_all(json_data.as_bytes());
                }
            }
        )*

        // let schema_vec = vec![$((stringify!($x), schemars::schema_for!($x)),)*];

        // for (name, schema) in schema_vec {
        //     let mut file = std::fs::File::create(format!("{}.json", name)).unwrap();
        //     let json_data = serde_json::to_string_pretty(&schema).unwrap();
        //     let _result = file.write_all(json_data.as_bytes());
        // }
    };
}
