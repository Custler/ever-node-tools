/*
* Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/
#[cfg(feature = "export_key")]
use ever_crypto::Ed25519KeyOption;
use std::env;
#[cfg(feature = "export_key")]
use std::convert::TryInto;
#[cfg(feature = "export_key")]
use ton_types::{error, fail, Result};

#[cfg(feature = "export_key")]
fn compute(typ: &str, key: &str) -> Result<()> {
    let key_bin = base64::decode(key)?;
    let key_bin: [u8; 32] = key_bin.try_into().map_err(
        |_| error!("Cannot decode key properly") 
    )?;
    let key = if typ.to_lowercase().as_str() == "pub" {
        println!("Public key: {}", key);
        Ed25519KeyOption::from_public_key(&key_bin)
    } else if typ.to_lowercase().as_str() == "pvt" {
        println!("Private key: {}", key);
        let key = Ed25519KeyOption::from_private_key(&key_bin)?;
        println!("Public key: {}", base64::encode(key.pub_key()?));
        println!("Extended private key: {}", base64::encode(key.export_key()?));
        key
    } else {
        fail!("Wrong key type: expected pub|pvt, found {}", typ)
    };
    println!("Key id: {}", base64::encode(key.id().data()));
    Ok(())
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: keyid pub|pvt <key in base64>");
        return
    };
    #[cfg(feature = "export_key")]
    compute(&args[1], &args[2]).unwrap_or_else(|e| println!("Key ID computing error: {}", e));
    #[cfg(not(feature = "export_key"))]
    println!("feature export_key is not active!");
}
