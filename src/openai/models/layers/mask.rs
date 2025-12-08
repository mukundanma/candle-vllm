use candle_core::{DType, Device, Tensor};

#[cfg(feature = "flash-attn")] // If flash-attn or metal is enabled, we don't implement this function.
                               // The actual implementation would be embedded in the flash or metal attention kernel.
pub fn get_attention_causal_mask(
    _: &Device,
    _: DType,
    _: &Tensor,
    _: &Vec<u32>,
    _: Option<usize>,
    _: bool,
) -> Option<Vec<Tensor>> {
    None
}

#[allow(unreachable_code)]
#[cfg(all(not(feature = "flash-attn"), feature = "cuda"))]
fn get_causal_mask_internal(
    device: &Device,
    dtype: DType,
    tgt_len: usize,
    sliding_window: Option<usize>,
) -> candle_core::Result<Tensor> {
    use attention_rs::mask::causal_mask;
    let mask = Tensor::zeros((tgt_len, tgt_len), dtype, device)?;
    let _ = causal_mask(&mask, sliding_window)?;
    mask.unsqueeze(0)?.unsqueeze(0)
}

#[cfg(all(not(feature = "flash-attn"), not(feature = "cuda")))]
fn get_causal_mask_internal(
    device: &Device,
    dtype: DType,
    tgt_len: usize,
    sliding_window: Option<usize>,
) -> candle_core::Result<Tensor> {
    let mut mask = Vec::with_capacity(tgt_len);
    for i in 0..tgt_len {
        let mut row = vec![f32::NEG_INFINITY; tgt_len];
        for j in 0..=i {
            if sliding_window.is_none() || i - j <= sliding_window.unwrap() {
                row[j] = 0.0;
            }
        }
        mask.push(row);
    }
    Tensor::from_vec(mask.concat(), (tgt_len, tgt_len), device)?.to_dtype(dtype)?.unsqueeze(0)?.unsqueeze(0)
}

#[cfg(not(feature = "flash-attn"))]
pub fn get_attention_causal_mask(
    device: &Device,
    dtype: DType,
    _: &Tensor,
    seqlens: &Vec<u32>,
    sliding_window: Option<usize>,
    is_prefill: bool,
) -> Option<Vec<Tensor>> {
    if !is_prefill {
        return None;
    }
    let mut offsets = vec![0u32];
    offsets.extend(seqlens.clone());
    let mut vec_mask = Vec::new();
    let mut start = 0;
    for (_, seq_offset) in seqlens.iter().enumerate() {
        let seq_len = seq_offset - start;
        let mask =
            get_causal_mask_internal(device, dtype, seq_len as usize, sliding_window).unwrap();
        vec_mask.push(mask);
        start = *seq_offset;
    }
    Some(vec_mask)
}
