use arcropolis_api::arc_callback;
use arcropolis_api::hash40;
use arcropolis_api::load_original_file;

use crate::moveset::*;

#[arc_callback]
pub fn my_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    unsafe {
        if hash == hash40("fighter/cloud/model/body/c00/sword_cloud_001_col.nutexb").as_u64() {
            SWORDS[0] = true;
        }
    }
    load_original_file(hash, &mut data)
}