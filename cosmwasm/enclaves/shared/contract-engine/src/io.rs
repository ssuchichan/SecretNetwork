use crate::contract_validation::ReplyParams;

/// This contains all the user-facing functions. In these functions we will be using
/// the consensus_io_exchange_keypair and a user-generated key to create a symmetric key
/// that is unique to the user and the enclave
///
use super::types::{IoNonce, SecretMessage};
use enclave_cosmwasm_v010_types::encoding::Binary;
use enclave_cosmwasm_v010_types::types::{CanonicalAddr, Coin};
use enclave_cosmwasm_v1_types::results::{Event, Reply, ReplyOn, SubMsgResponse, SubMsgResult};

use enclave_ffi_types::EnclaveError;

use enclave_crypto::{AESKey, Ed25519PublicKey, Kdf, SIVEncryptable, KEY_MANAGER};

use log::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use sha2::Digest;

/// The internal_reply_enclave_sig is being passed with the reply (Only if the reply is wasm reply)
/// This is used by the receiver of the reply to:
/// a. Verify the sender (Cotnract address)
/// b. Authenticate the reply.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
enum RawWasmOutput {
    Err {
        #[serde(rename = "Err")]
        err: Value,
        internal_msg_id: Option<Binary>,
        internal_reply_enclave_sig: Option<Binary>,
    },
    QueryOkV010 {
        #[serde(rename = "Ok")]
        ok: String,
    },
    QueryOkV1 {
        #[serde(rename = "ok")]
        ok: String,
    },
    OkV010 {
        #[serde(rename = "Ok")]
        ok: enclave_cosmwasm_v010_types::types::ContractResult,
        internal_reply_enclave_sig: Option<Binary>,
        internal_msg_id: Option<Binary>,
    },
    OkV1 {
        #[serde(rename = "Ok")]
        ok: enclave_cosmwasm_v1_types::results::Response,
        internal_reply_enclave_sig: Option<Binary>,
        internal_msg_id: Option<Binary>,
    },
    OkIBC {
        #[serde(rename = "Ok")]
        ok: enclave_cosmwasm_v1_types::ibc::IbcBasicResponse,
        internal_reply_enclave_sig: Option<Binary>,
        internal_msg_id: Option<Binary>,
    },
    OkIBCReceive {
        #[serde(rename = "Ok")]
        ok: enclave_cosmwasm_v1_types::ibc::IbcReceiveResponse,
        internal_reply_enclave_sig: Option<Binary>,
        internal_msg_id: Option<Binary>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct V010WasmOutput {
    #[serde(rename = "Ok")]
    pub ok: Option<enclave_cosmwasm_v010_types::types::ContractResult>,
    #[serde(rename = "Err")]
    pub err: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct V1WasmOutput {
    #[serde(rename = "Ok")]
    pub ok: Option<enclave_cosmwasm_v1_types::results::Response>,
    #[serde(rename = "Err")]
    pub err: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct IBCOutput {
    #[serde(rename = "Ok")]
    pub ok: Option<enclave_cosmwasm_v1_types::ibc::IbcBasicResponse>,
    #[serde(rename = "Err")]
    pub err: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct IBCReceiveOutput {
    #[serde(rename = "Ok")]
    pub ok: Option<enclave_cosmwasm_v1_types::ibc::IbcReceiveResponse>,
    #[serde(rename = "Err")]
    pub err: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct QueryOutput {
    #[serde(rename = "Ok")]
    pub ok: Option<String>,
    #[serde(rename = "Err")]
    pub err: Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct WasmOutput {
    pub v010: Option<V010WasmOutput>,
    pub v1: Option<V1WasmOutput>,
    pub ibc: Option<IBCOutput>,
    pub ibc_receive: Option<IBCReceiveOutput>,
    pub query: Option<QueryOutput>,
    pub internal_reply_enclave_sig: Option<Binary>,
    pub internal_msg_id: Option<Binary>,
}

pub fn calc_encryption_key(nonce: &IoNonce, user_public_key: &Ed25519PublicKey) -> AESKey {
    let enclave_io_key = KEY_MANAGER.get_consensus_io_exchange_keypair().unwrap();

    let tx_encryption_ikm = enclave_io_key.diffie_hellman(user_public_key);

    let tx_encryption_key = AESKey::new_from_slice(&tx_encryption_ikm).derive_key_from_this(nonce);

    trace!("rust tx_encryption_key {:?}", tx_encryption_key.get());

    tx_encryption_key
}

fn encrypt_serializable<T>(
    key: &AESKey,
    val: &T,
    reply_params: &Option<ReplyParams>,
) -> Result<String, EnclaveError>
where
    T: ?Sized + Serialize,
{
    let serialized: String = serde_json::to_string(val).map_err(|err| {
        debug!("got an error while trying to encrypt output error {}", err);
        EnclaveError::EncryptionError
    })?;

    let trimmed = serialized.trim_start_matches('"').trim_end_matches('"');

    encrypt_preserialized_string(key, trimmed, reply_params)
}

// use this to encrypt a String that has already been serialized.  When that is the case, if
// encrypt_serializable is called instead, it will get double serialized, and any escaped
// characters will be double escaped
fn encrypt_preserialized_string(
    key: &AESKey,
    val: &str,
    reply_params: &Option<ReplyParams>,
) -> Result<String, EnclaveError> {
    let serialized = match reply_params {
        Some(ReplyParams {
            recipient_contract_hash,
            ..
        }) => {
            let mut ser = vec![];
            ser.extend_from_slice(&recipient_contract_hash);
            ser.extend_from_slice(val.as_bytes());
            ser
        }
        None => val.as_bytes().to_vec(),
    };
    let encrypted_data = key
        .encrypt_siv(serialized.as_slice(), None)
        .map_err(|err| {
            debug!(
                "got an error while trying to encrypt output error {:?}: {}",
                err, err
            );
            EnclaveError::EncryptionError
        })?;

    Ok(b64_encode(encrypted_data.as_slice()))
}

fn b64_encode(data: &[u8]) -> String {
    base64::encode(data)
}

pub fn finalize_raw_output(raw_output: RawWasmOutput) -> WasmOutput {
    return match raw_output {
        RawWasmOutput::Err {
            err,
            internal_msg_id,
            internal_reply_enclave_sig,
        } => {
            if is_query_output {
                WasmOutput {
                    v010: None,
                    v1: None,
                    ibc: None,
                    ibc_receive: None,
                    query: Some(QueryOutput {
                        ok: None,
                        err: Some(err),
                    }),
                    internal_reply_enclave_sig: None,
                    internal_msg_id: None,
                }
            } else {
                WasmOutput {
                    v010: Some(V010WasmOutput {
                        err: Some(err),
                        ok: None,
                    }),
                    v1: None,
                    ibc: None,
                    ibc_receive: None,
                    query: None,
                    internal_reply_enclave_sig,
                    internal_msg_id,
                }
            }
        }
        RawWasmOutput::OkV010 {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => WasmOutput {
            v010: Some(V010WasmOutput {
                err: None,
                ok: Some(ok),
            }),
            v1: None,
            ibc: None,
            ibc_receive: None,
            query: None,
            internal_reply_enclave_sig,
            internal_msg_id,
        },
        RawWasmOutput::OkV1 {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => WasmOutput {
            v010: None,
            v1: Some(V1WasmOutput {
                err: None,
                ok: Some(ok),
            }),
            ibc: None,
            ibc_receive: None,
            query: None,
            internal_reply_enclave_sig,
            internal_msg_id,
        },
        RawWasmOutput::QueryOkV010 { ok } | RawWasmOutput::QueryOkV1 { ok } => WasmOutput {
            v010: None,
            v1: None,
            ibc: None,
            ibc_receive: None,
            query: Some(QueryOutput {
                ok: Some(ok),
                err: None,
            }),
            internal_reply_enclave_sig: None,
            internal_msg_id: None,
        },
        RawWasmOutput::OkIBC {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => WasmOutput {
            v010: None,
            v1: None,
            ibc: Some(IBCOutput {
                err: None,
                ok: Some(ok),
            }),
            ibc_receive: None,
            query: None,
            internal_reply_enclave_sig,
            internal_msg_id,
        },
        RawWasmOutput::OkIBCReceive {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => WasmOutput {
            v010: None,
            v1: None,
            ibc: None,
            ibc_receive: Some(IBCReceiveOutput {
                err: None,
                ok: Some(ok),
            }),
            query: None,
            internal_reply_enclave_sig,
            internal_msg_id,
        },
    };
}

pub fn encrypt_output(
    output: Vec<u8>,
    secret_msg: &SecretMessage,
    contract_addr: &CanonicalAddr,
    contract_hash: &str,
    reply_params: Option<ReplyParams>,
    sender_addr: &CanonicalAddr,
    is_query_output: bool,
) -> Result<Vec<u8>, EnclaveError> {
    // When encrypting an output we might encrypt an output that is a reply to a caller contract (Via "Reply" endpoint).
    // Therefore if reply_recipient_contract_hash is not "None" we append it to any encrypted data besided submessages that are irrelevant for replies.
    // More info in: https://github.com/CosmWasm/cosmwasm/blob/v1.0.0/packages/std/src/results/submessages.rs#L192-L198
    let encryption_key = calc_encryption_key(&secret_msg.nonce, &secret_msg.user_public_key);
    trace!(
        "Output before encryption: {:?}",
        String::from_utf8_lossy(&output)
    );

    let mut output: RawWasmOutput = serde_json::from_slice(&output).map_err(|err| {
        warn!("got an error while trying to deserialize output bytes into json");
        trace!("output: {:?} error: {:?}", output, err);
        EnclaveError::FailedToDeserialize
    })?;

    match &mut output {
        RawWasmOutput::Err {
            err,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => {
            let encrypted_err = encrypt_serializable(&encryption_key, err, &reply_params)?;

            // Putting the error inside a 'generic_err' envelope, so we can encrypt the error itself
            *err = json!({"generic_err":{"msg":encrypted_err}});

            let msg_id = match reply_params {
                Some(ref r) => {
                    let encrypted_id = Binary::from_base64(&encrypt_preserialized_string(
                        &encryption_key,
                        &r.sub_msg_id.to_string(),
                        &reply_params,
                    )?)?;

                    Some(encrypted_id)
                }
                None => None,
            };

            *internal_msg_id = msg_id.clone();

            *internal_reply_enclave_sig = match reply_params {
                Some(_) => {
                    let reply = Reply {
                        id: msg_id.unwrap(),
                        result: SubMsgResult::Err(encrypted_err),
                    };
                    let reply_as_vec = serde_json::to_vec(&reply).map_err(|err| {
                        warn!(
                            "got an error while trying to serialize reply into bytes for internal_reply_enclave_sig  {:?}: {}",
                            reply, err
                        );
                        EnclaveError::FailedToSerialize
                    })?;
                    let tmp_secret_msg = SecretMessage {
                        nonce: secret_msg.nonce,
                        user_public_key: secret_msg.user_public_key,
                        msg: reply_as_vec,
                    };

                    Some(Binary::from(
                        create_callback_signature(sender_addr, &tmp_secret_msg, &[]).as_slice(),
                    ))
                }
                None => None, // Not a reply, we don't need enclave sig
            }
        }

        RawWasmOutput::QueryOkV010 { ok } | RawWasmOutput::QueryOkV1 { ok } => {
            *ok = encrypt_serializable(&encryption_key, ok, &reply_params)?;
        }

        // Encrypt all Wasm messages (keeps Bank, Staking, etc.. as is)
        RawWasmOutput::OkV010 {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => {
            for msg in &mut ok.messages {
                if let enclave_cosmwasm_v010_types::types::CosmosMsg::Wasm(wasm_msg) = msg {
                    encrypt_v010_wasm_msg(
                        wasm_msg,
                        secret_msg.nonce,
                        secret_msg.user_public_key,
                        contract_addr,
                    )?;
                }
            }

            // v0.10: The logs that will be emitted as part of a "wasm" event.
            for log in ok.log.iter_mut().filter(|log| log.encrypted) {
                log.key = encrypt_preserialized_string(&encryption_key, &log.key, &None)?;
                log.value = encrypt_preserialized_string(&encryption_key, &log.value, &None)?;
            }

            if let Some(data) = &mut ok.data {
                *data = Binary::from_base64(&encrypt_serializable(
                    &encryption_key,
                    data,
                    &reply_params,
                )?)?;
            }

            let msg_id = match reply_params {
                Some(ref r) => {
                    let encrypted_id = Binary::from_base64(&encrypt_preserialized_string(
                        &encryption_key,
                        &r.sub_msg_id.to_string(),
                        &reply_params,
                    )?)?;

                    Some(encrypted_id)
                }
                None => None,
            };

            *internal_msg_id = msg_id.clone();

            *internal_reply_enclave_sig = match reply_params {
                Some(_) => {
                    let events = match ok.log.len() {
                        0 => vec![],
                        _ => vec![Event {
                            ty: "wasm".to_string(),
                            attributes: ok.log.clone(),
                        }],
                    };

                    let reply = Reply {
                        id: msg_id.unwrap(),
                        result: SubMsgResult::Ok(SubMsgResponse {
                            events,
                            data: ok.data.clone(),
                        }),
                    };

                    let reply_as_vec = serde_json::to_vec(&reply).map_err(|err| {
                        warn!(
                            "got an error while trying to serialize reply into bytes for internal_reply_enclave_sig  {:?}: {}",
                            reply, err
                        );
                        EnclaveError::FailedToSerialize
                    })?;
                    let tmp_secret_msg = SecretMessage {
                        nonce: secret_msg.nonce,
                        user_public_key: secret_msg.user_public_key,
                        msg: reply_as_vec,
                    };

                    Some(Binary::from(
                        create_callback_signature(sender_addr, &tmp_secret_msg, &[]).as_slice(),
                    ))
                }
                None => None, // Not a reply, we don't need enclave sig
            }
        }
        RawWasmOutput::OkV1 {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => {
            for sub_msg in &mut ok.messages {
                if let enclave_cosmwasm_v1_types::results::CosmosMsg::Wasm(wasm_msg) =
                    &mut sub_msg.msg
                {
                    encrypt_v1_wasm_msg(
                        wasm_msg,
                        &sub_msg.reply_on,
                        sub_msg.id,
                        secret_msg.nonce,
                        secret_msg.user_public_key,
                        contract_addr,
                        contract_hash,
                    )?;

                    // The ID can be extracted from the encrypted wasm msg
                    // We don't encrypt it here to remain with the same type (u64)
                    sub_msg.id = 0;
                }
            }

            // v1: The attributes that will be emitted as part of a "wasm" event.
            for attr in ok.attributes.iter_mut().filter(|attr| attr.encrypted) {
                attr.key = encrypt_preserialized_string(&encryption_key, &attr.key, &None)?;
                attr.value = encrypt_preserialized_string(&encryption_key, &attr.value, &None)?;
            }

            // v1: Extra, custom events separate from the main wasm one. These will have "wasm-"" prepended to the type.
            for event in ok.events.iter_mut() {
                for attr in event.attributes.iter_mut().filter(|attr| attr.encrypted) {
                    attr.key = encrypt_preserialized_string(&encryption_key, &attr.key, &None)?;
                    attr.value = encrypt_preserialized_string(&encryption_key, &attr.value, &None)?;
                }
            }

            if let Some(data) = &mut ok.data {
                *data = Binary::from_base64(&encrypt_serializable(
                    &encryption_key,
                    data,
                    &reply_params,
                )?)?;
            }

            let msg_id = match reply_params {
                Some(ref r) => {
                    let encrypted_id = Binary::from_base64(&encrypt_preserialized_string(
                        &encryption_key,
                        &r.sub_msg_id.to_string(),
                        &reply_params,
                    )?)?;

                    Some(encrypted_id)
                }
                None => None,
            };

            *internal_msg_id = msg_id.clone();

            *internal_reply_enclave_sig = match reply_params {
                Some(_) => {
                    let mut events: Vec<Event> = vec![];

                    if ok.attributes.len() > 0 {
                        events.push(Event {
                            ty: "wasm".to_string(),
                            attributes: ok.attributes.clone(),
                        })
                    }

                    events.extend_from_slice(&ok.events.clone().as_slice());
                    let custom_contract_event_prefix: String = "wasm-".to_string();
                    for event in events.iter_mut() {
                        if event.ty != "wasm" {
                            event.ty = custom_contract_event_prefix.clone() + event.ty.as_str();
                        }
                    }

                    let reply = Reply {
                        id: msg_id.unwrap(),
                        result: SubMsgResult::Ok(SubMsgResponse {
                            events,
                            data: ok.data.clone(),
                        }),
                    };

                    let reply_as_vec = serde_json::to_vec(&reply).map_err(|err| {
                        warn!(
                            "got an error while trying to serialize reply into bytes for internal_reply_enclave_sig  {:?}: {}",
                            reply, err
                        );
                        EnclaveError::FailedToSerialize
                    })?;

                    let tmp_secret_msg = SecretMessage {
                        nonce: secret_msg.nonce,
                        user_public_key: secret_msg.user_public_key,
                        msg: reply_as_vec,
                    };

                    Some(Binary::from(
                        create_callback_signature(sender_addr, &tmp_secret_msg, &[]).as_slice(),
                    ))
                }
                None => None, // Not a reply, we don't need enclave sig
            }
        }
        RawWasmOutput::OkIBC {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => {
            for sub_msg in &mut ok.messages {
                if let enclave_cosmwasm_v1_types::results::CosmosMsg::Wasm(wasm_msg) =
                    &mut sub_msg.msg
                {
                    encrypt_v1_wasm_msg(
                        wasm_msg,
                        &sub_msg.reply_on,
                        sub_msg.id,
                        secret_msg.nonce,
                        secret_msg.user_public_key,
                        contract_addr,
                        contract_hash,
                    )?;

                    // The ID can be extracted from the encrypted wasm msg
                    // We don't encrypt it here to remain with the same type (u64)
                    sub_msg.id = 0;
                }
            }

            // v1: The attributes that will be emitted as part of a "wasm" event.
            for attr in ok.attributes.iter_mut().filter(|attr| attr.encrypted) {
                attr.key = encrypt_preserialized_string(&encryption_key, &attr.key, &None)?;
                attr.value = encrypt_preserialized_string(&encryption_key, &attr.value, &None)?;
            }

            // v1: Extra, custom events separate from the main wasm one. These will have "wasm-"" prepended to the type.
            for event in ok.events.iter_mut() {
                for attr in event.attributes.iter_mut().filter(|attr| attr.encrypted) {
                    attr.key = encrypt_preserialized_string(&encryption_key, &attr.key, &None)?;
                    attr.value = encrypt_preserialized_string(&encryption_key, &attr.value, &None)?;
                }
            }

            let msg_id = match reply_params {
                Some(ref r) => {
                    let encrypted_id = Binary::from_base64(&encrypt_preserialized_string(
                        &encryption_key,
                        &r.sub_msg_id.to_string(),
                        &reply_params,
                    )?)?;

                    Some(encrypted_id)
                }
                None => None,
            };

            *internal_msg_id = msg_id.clone();

            *internal_reply_enclave_sig = match reply_params {
                Some(_) => {
                    let mut events: Vec<Event> = vec![];

                    if ok.attributes.len() > 0 {
                        events.push(Event {
                            ty: "wasm".to_string(),
                            attributes: ok.attributes.clone(),
                        })
                    }

                    events.extend_from_slice(&ok.events.clone().as_slice());
                    let custom_contract_event_prefix: String = "wasm-".to_string();
                    for event in events.iter_mut() {
                        if event.ty != "wasm" {
                            event.ty = custom_contract_event_prefix.clone() + event.ty.as_str();
                        }
                    }

                    let reply = Reply {
                        id: msg_id.unwrap(),
                        result: SubMsgResult::Ok(SubMsgResponse { events, data: None }),
                    };

                    let reply_as_vec = serde_json::to_vec(&reply).map_err(|err| {
                        warn!(
                            "got an error while trying to serialize reply into bytes for internal_reply_enclave_sig  {:?}: {}",
                            reply, err
                        );
                        EnclaveError::FailedToSerialize
                    })?;

                    let tmp_secret_msg = SecretMessage {
                        nonce: secret_msg.nonce,
                        user_public_key: secret_msg.user_public_key,
                        msg: reply_as_vec,
                    };

                    Some(Binary::from(
                        create_callback_signature(sender_addr, &tmp_secret_msg, &[]).as_slice(),
                    ))
                }
                None => None, // Not a reply, we don't need enclave sig
            }
        }
        RawWasmOutput::OkIBCReceive {
            ok,
            internal_reply_enclave_sig,
            internal_msg_id,
        } => {
            for sub_msg in &mut ok.messages {
                if let enclave_cosmwasm_v1_types::results::CosmosMsg::Wasm(wasm_msg) =
                    &mut sub_msg.msg
                {
                    encrypt_v1_wasm_msg(
                        wasm_msg,
                        &sub_msg.reply_on,
                        sub_msg.id,
                        secret_msg.nonce,
                        secret_msg.user_public_key,
                        contract_addr,
                        contract_hash,
                    )?;

                    // The ID can be extracted from the encrypted wasm msg
                    // We don't encrypt it here to remain with the same type (u64)
                    sub_msg.id = 0;
                }
            }

            // v1: The attributes that will be emitted as part of a "wasm" event.
            for attr in ok.attributes.iter_mut().filter(|attr| attr.encrypted) {
                attr.key = encrypt_preserialized_string(&encryption_key, &attr.key, &None)?;
                attr.value = encrypt_preserialized_string(&encryption_key, &attr.value, &None)?;
            }

            // v1: Extra, custom events separate from the main wasm one. These will have "wasm-"" prepended to the type.
            for event in ok.events.iter_mut() {
                for attr in event.attributes.iter_mut().filter(|attr| attr.encrypted) {
                    attr.key = encrypt_preserialized_string(&encryption_key, &attr.key, &None)?;
                    attr.value = encrypt_preserialized_string(&encryption_key, &attr.value, &None)?;
                }
            }

            ok.acknowledgement = Binary::from_base64(&encrypt_serializable(
                &encryption_key,
                &ok.acknowledgement,
                &reply_params,
            )?)?;

            let msg_id = match reply_params {
                Some(ref r) => {
                    let encrypted_id = Binary::from_base64(&encrypt_preserialized_string(
                        &encryption_key,
                        &r.sub_msg_id.to_string(),
                        &reply_params,
                    )?)?;

                    Some(encrypted_id)
                }
                None => None,
            };

            *internal_msg_id = msg_id.clone();

            *internal_reply_enclave_sig = match reply_params {
                Some(_) => {
                    let mut events: Vec<Event> = vec![];

                    if ok.attributes.len() > 0 {
                        events.push(Event {
                            ty: "wasm".to_string(),
                            attributes: ok.attributes.clone(),
                        })
                    }

                    events.extend_from_slice(&ok.events.clone().as_slice());
                    let custom_contract_event_prefix: String = "wasm-".to_string();
                    for event in events.iter_mut() {
                        if event.ty != "wasm" {
                            event.ty = custom_contract_event_prefix.clone() + event.ty.as_str();
                        }
                    }

                    let reply = Reply {
                        id: msg_id.unwrap(),
                        result: SubMsgResult::Ok(SubMsgResponse {
                            events,
                            data: Some(ok.acknowledgement.clone()),
                        }),
                    };

                    let reply_as_vec = serde_json::to_vec(&reply).map_err(|err| {
                        warn!(
                            "got an error while trying to serialize reply into bytes for internal_reply_enclave_sig  {:?}: {}",
                            reply, err
                        );
                        EnclaveError::FailedToSerialize
                    })?;

                    let tmp_secret_msg = SecretMessage {
                        nonce: secret_msg.nonce,
                        user_public_key: secret_msg.user_public_key,
                        msg: reply_as_vec,
                    };

                    Some(Binary::from(
                        create_callback_signature(sender_addr, &tmp_secret_msg, &[]).as_slice(),
                    ))
                }
                None => None, // Not a reply, we don't need enclave sig
            }
        }
    };

    let final_output = finalize_raw_output(output);
    trace!("WasmOutput: {:?}", final_output);

    let encrypted_output = serde_json::to_vec(&final_output).map_err(|err| {
        debug!(
            "got an error while trying to serialize output json into bytes {:?}: {}",
            final_output, err
        );
        EnclaveError::FailedToSerialize
    })?;

    Ok(encrypted_output)
}

fn encrypt_v010_wasm_msg(
    wasm_msg: &mut enclave_cosmwasm_v010_types::types::WasmMsg,
    nonce: IoNonce,
    user_public_key: Ed25519PublicKey,
    contract_addr: &CanonicalAddr,
) -> Result<(), EnclaveError> {
    match wasm_msg {
        enclave_cosmwasm_v010_types::types::WasmMsg::Execute {
            msg,
            callback_code_hash,
            callback_sig,
            send,
            ..
        }
        | enclave_cosmwasm_v010_types::types::WasmMsg::Instantiate {
            msg,
            callback_code_hash,
            callback_sig,
            send,
            ..
        } => {
            let mut hash_appended_msg = callback_code_hash.as_bytes().to_vec();
            hash_appended_msg.extend_from_slice(msg.as_slice());

            let mut msg_to_pass = SecretMessage::from_base64(
                Binary(hash_appended_msg).to_base64(),
                nonce,
                user_public_key,
            )?;

            msg_to_pass.encrypt_in_place()?;
            *msg = Binary::from(msg_to_pass.to_vec().as_slice());

            *callback_sig = Some(create_callback_signature(contract_addr, &msg_to_pass, send));
        }
    }

    Ok(())
}

fn encrypt_v1_wasm_msg(
    wasm_msg: &mut enclave_cosmwasm_v1_types::results::WasmMsg,
    reply_on: &ReplyOn,
    msg_id: u64, // In every submessage there is a field called "id", currently used only by "reply".
    nonce: IoNonce,
    user_public_key: Ed25519PublicKey,
    contract_addr: &CanonicalAddr,
    reply_recipient_contract_hash: &str,
) -> Result<(), EnclaveError> {
    match wasm_msg {
        enclave_cosmwasm_v1_types::results::WasmMsg::Execute {
            msg,
            code_hash,
            callback_sig,
            funds,
            ..
        }
        | enclave_cosmwasm_v1_types::results::WasmMsg::Instantiate {
            msg,
            code_hash,
            callback_sig,
            funds,
            ..
        } => {
            // On cosmwasm v1 submessages' outputs can be sent back to the original caller by using "Reply"
            // The output is encrypted but the historically wasn't meant to be  sent back to the enclave as an input of another contract
            // To support "sending back" behavior, the enclave expects every encrypted input to be prepended with the recipient wasm hash.
            // In this context, we prepend the message with both hashes to signal to the next wasm call that its output is going to be an input to this contract as a "Reply"
            // On the other side when decrypting the input, the enclave will try to parse the message as usual, if the message (After reading the first code-hash) can't be parsed into json,
            // it will treat the next 64 bytes as a recipient code-hash and prepend this code-hash to its output.
            let mut hash_appended_msg = code_hash.as_bytes().to_vec();
            if *reply_on != ReplyOn::Never {
                hash_appended_msg.extend_from_slice(
                    enclave_cosmwasm_v1_types::results::REPLY_ENCRYPTION_MAGIC_BYTES,
                );
                hash_appended_msg.extend_from_slice(&msg_id.to_be_bytes());
                hash_appended_msg.extend_from_slice(reply_recipient_contract_hash.as_bytes());
            }
            hash_appended_msg.extend_from_slice(msg.as_slice());

            let mut msg_to_pass = SecretMessage::from_base64(
                Binary(hash_appended_msg).to_base64(),
                nonce,
                user_public_key,
            )?;

            msg_to_pass.encrypt_in_place()?;

            *msg = Binary::from(msg_to_pass.to_vec().as_slice());

            *callback_sig = Some(create_callback_signature(
                contract_addr,
                &msg_to_pass,
                &funds
                    .iter()
                    .map(|coin| enclave_cosmwasm_v010_types::types::Coin {
                        denom: coin.denom.clone(),
                        amount: enclave_cosmwasm_v010_types::math::Uint128(coin.amount.u128()),
                    })
                    .collect::<Vec<enclave_cosmwasm_v010_types::types::Coin>>()[..],
            ));
        }
    }

    Ok(())
}

pub fn create_callback_signature(
    contract_addr: &CanonicalAddr,
    msg_to_sign: &SecretMessage,
    funds_to_send: &[Coin],
) -> Vec<u8> {
    // Hash(Enclave_secret | sender(current contract) | msg_to_pass | sent_funds)
    let mut callback_sig_bytes = KEY_MANAGER
        .get_consensus_callback_secret()
        .unwrap()
        .get()
        .to_vec();

    callback_sig_bytes.extend(contract_addr.as_slice());
    callback_sig_bytes.extend(msg_to_sign.msg.as_slice());
    callback_sig_bytes.extend(serde_json::to_vec(funds_to_send).unwrap());

    sha2::Sha256::digest(callback_sig_bytes.as_slice()).to_vec()
}
