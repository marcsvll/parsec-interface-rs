// Copyright (c) 2019-2020, Arm Limited, All Rights Reserved
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//          http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! # Protobuf converter
//!
//! This module exposes the `ProtobufConverter` struct that implements the `Convert` trait.
mod convert_psa_algorithm;
mod convert_ping;
mod convert_psa_generate_key;
mod convert_psa_key_attributes;
mod convert_psa_import_key;
mod convert_psa_export_public_key;
mod convert_psa_destroy_key;
mod convert_psa_sign_hash;
mod convert_psa_verify_hash;
mod convert_list_providers;
mod convert_list_opcodes;

#[rustfmt::skip]
#[allow(unused_qualifications, missing_copy_implementations, clippy::pedantic, clippy::module_inception)]
mod generated_ops {
    // Include the Rust generated file in its own module.
    macro_rules! include_protobuf_as_module {
        ($name:ident) => {
            pub mod $name {
                // The generated Rust file is in OUT_DIR, named $name.rs
                include!(concat!(env!("OUT_DIR"), "/", stringify!($name), ".rs"));
            }
        };
    }

    include_protobuf_as_module!(psa_sign_hash);
    include_protobuf_as_module!(psa_verify_hash);
    include_protobuf_as_module!(psa_generate_key);
    include_protobuf_as_module!(psa_destroy_key);
    include_protobuf_as_module!(psa_export_public_key);
    include_protobuf_as_module!(psa_import_key);
    include_protobuf_as_module!(list_opcodes);
    include_protobuf_as_module!(list_providers);
    include_protobuf_as_module!(ping);
    include_protobuf_as_module!(psa_key_attributes);
    include_protobuf_as_module!(psa_algorithm);
}

use crate::operations::{Convert, NativeOperation, NativeResult};
use crate::requests::{
    request::RequestBody, response::ResponseBody, BodyType, Opcode, ResponseStatus, Result,
};
use generated_ops::list_opcodes as list_opcodes_proto;
use generated_ops::list_providers as list_providers_proto;
use generated_ops::ping as ping_proto;
use generated_ops::psa_destroy_key as psa_destroy_key_proto;
use generated_ops::psa_export_public_key as psa_export_public_key_proto;
use generated_ops::psa_generate_key as psa_generate_key_proto;
use generated_ops::psa_import_key as psa_import_key_proto;
use generated_ops::psa_sign_hash as psa_sign_hash_proto;
use generated_ops::psa_verify_hash as psa_verify_hash_proto;
use prost::Message;
use std::convert::TryInto;

macro_rules! wire_to_native {
    ($body:expr, $proto_type:ty) => {{
        let mut proto: $proto_type = Default::default();
        if proto.merge($body).is_err() {
            return Err(ResponseStatus::DeserializingBodyFailed);
        }
        proto.try_into()?
    }};
}

macro_rules! native_to_wire {
    ($native_msg:expr, $proto_type:ty) => {{
        let proto: $proto_type = $native_msg.try_into()?;
        let mut bytes = Vec::new();
        if proto.encode(&mut bytes).is_err() {
            return Err(ResponseStatus::SerializingBodyFailed);
        }
        bytes
    }};
}

/// Implementation for a converter between protobuf-encoded bodies and native
/// objects.
#[derive(Copy, Clone, Debug)]
pub struct ProtobufConverter;

impl Convert for ProtobufConverter {
    fn body_type(&self) -> BodyType {
        BodyType::Protobuf
    }

    fn body_to_operation(&self, body: RequestBody, opcode: Opcode) -> Result<NativeOperation> {
        match opcode {
            Opcode::ListProviders => Ok(NativeOperation::ListProviders(wire_to_native!(
                body.bytes(),
                list_providers_proto::Operation
            ))),
            Opcode::ListOpcodes => Ok(NativeOperation::ListOpcodes(wire_to_native!(
                body.bytes(),
                list_opcodes_proto::Operation
            ))),
            Opcode::Ping => Ok(NativeOperation::Ping(wire_to_native!(
                body.bytes(),
                ping_proto::Operation
            ))),
            Opcode::PsaGenerateKey => Ok(NativeOperation::PsaGenerateKey(wire_to_native!(
                body.bytes(),
                psa_generate_key_proto::Operation
            ))),
            Opcode::PsaImportKey => Ok(NativeOperation::PsaImportKey(wire_to_native!(
                body.bytes(),
                psa_import_key_proto::Operation
            ))),
            Opcode::PsaExportPublicKey => Ok(NativeOperation::PsaExportPublicKey(wire_to_native!(
                body.bytes(),
                psa_export_public_key_proto::Operation
            ))),
            Opcode::PsaDestroyKey => Ok(NativeOperation::PsaDestroyKey(wire_to_native!(
                body.bytes(),
                psa_destroy_key_proto::Operation
            ))),
            Opcode::PsaSignHash => Ok(NativeOperation::PsaSignHash(wire_to_native!(
                body.bytes(),
                psa_sign_hash_proto::Operation
            ))),
            Opcode::PsaVerifyHash => Ok(NativeOperation::PsaVerifyHash(wire_to_native!(
                body.bytes(),
                psa_verify_hash_proto::Operation
            ))),
        }
    }

    fn operation_to_body(&self, operation: NativeOperation) -> Result<RequestBody> {
        match operation {
            NativeOperation::ListProviders(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, list_providers_proto::Operation),
            )),
            NativeOperation::ListOpcodes(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, list_opcodes_proto::Operation),
            )),
            NativeOperation::Ping(operation) => Ok(RequestBody::from_bytes(native_to_wire!(
                operation,
                ping_proto::Operation
            ))),
            NativeOperation::PsaGenerateKey(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, psa_generate_key_proto::Operation),
            )),
            NativeOperation::PsaImportKey(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, psa_import_key_proto::Operation),
            )),
            NativeOperation::PsaExportPublicKey(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, psa_export_public_key_proto::Operation),
            )),
            NativeOperation::PsaDestroyKey(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, psa_destroy_key_proto::Operation),
            )),
            NativeOperation::PsaSignHash(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, psa_sign_hash_proto::Operation),
            )),
            NativeOperation::PsaVerifyHash(operation) => Ok(RequestBody::from_bytes(
                native_to_wire!(operation, psa_verify_hash_proto::Operation),
            )),
        }
    }

    fn body_to_result(&self, body: ResponseBody, opcode: Opcode) -> Result<NativeResult> {
        match opcode {
            Opcode::ListProviders => Ok(NativeResult::ListProviders(wire_to_native!(
                body.bytes(),
                list_providers_proto::Result
            ))),
            Opcode::ListOpcodes => Ok(NativeResult::ListOpcodes(wire_to_native!(
                body.bytes(),
                list_opcodes_proto::Result
            ))),
            Opcode::Ping => Ok(NativeResult::Ping(wire_to_native!(
                body.bytes(),
                ping_proto::Result
            ))),
            Opcode::PsaGenerateKey => Ok(NativeResult::PsaGenerateKey(wire_to_native!(
                body.bytes(),
                psa_generate_key_proto::Result
            ))),
            Opcode::PsaImportKey => Ok(NativeResult::PsaImportKey(wire_to_native!(
                body.bytes(),
                psa_import_key_proto::Result
            ))),
            Opcode::PsaExportPublicKey => Ok(NativeResult::PsaExportPublicKey(wire_to_native!(
                body.bytes(),
                psa_export_public_key_proto::Result
            ))),
            Opcode::PsaDestroyKey => Ok(NativeResult::PsaDestroyKey(wire_to_native!(
                body.bytes(),
                psa_destroy_key_proto::Result
            ))),
            Opcode::PsaSignHash => Ok(NativeResult::PsaSignHash(wire_to_native!(
                body.bytes(),
                psa_sign_hash_proto::Result
            ))),
            Opcode::PsaVerifyHash => Ok(NativeResult::PsaVerifyHash(wire_to_native!(
                body.bytes(),
                psa_verify_hash_proto::Result
            ))),
        }
    }

    fn result_to_body(&self, result: NativeResult) -> Result<ResponseBody> {
        match result {
            NativeResult::ListProviders(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                list_providers_proto::Result
            ))),
            NativeResult::ListOpcodes(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                list_opcodes_proto::Result
            ))),
            NativeResult::Ping(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                ping_proto::Result
            ))),
            NativeResult::PsaGenerateKey(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                psa_generate_key_proto::Result
            ))),
            NativeResult::PsaImportKey(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                psa_import_key_proto::Result
            ))),
            NativeResult::PsaExportPublicKey(result) => Ok(ResponseBody::from_bytes(
                native_to_wire!(result, psa_export_public_key_proto::Result),
            )),
            NativeResult::PsaDestroyKey(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                psa_destroy_key_proto::Result
            ))),
            NativeResult::PsaSignHash(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                psa_sign_hash_proto::Result
            ))),
            NativeResult::PsaVerifyHash(result) => Ok(ResponseBody::from_bytes(native_to_wire!(
                result,
                psa_verify_hash_proto::Result
            ))),
        }
    }
}
