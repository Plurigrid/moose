use crate::computation::{FixedpointRingMeanOp, HostPlacement, Placed, Placement};
use crate::error::Result;
use crate::kernels::{PlacementPlace, PlacementRingMean, RuntimeSession};
use crate::replicated::{Replicated128Tensor, Replicated64Tensor};
use crate::ring::{AbstractRingTensor, Ring128Tensor, Ring64Tensor};
use crate::standard::Float64Tensor;
use ndarray::prelude::*;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::num::Wrapping;
use std::ops::Mul;

pub type Fixed64Tensor = FixedTensor<Ring64Tensor, Replicated64Tensor>;

pub type Fixed128Tensor = FixedTensor<Ring128Tensor, Replicated128Tensor>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FixedTensor<RingTensorT, ReplicatedTensorT> {
    RingTensor(RingTensorT),
    ReplicatedTensor(ReplicatedTensorT),
}

impl<RingTensorT, ReplicatedTensorT> Placed for FixedTensor<RingTensorT, ReplicatedTensorT>
where
    RingTensorT: Placed,
    RingTensorT::Placement: Into<Placement>,
    ReplicatedTensorT: Placed,
    ReplicatedTensorT::Placement: Into<Placement>,
{
    type Placement = Placement;

    fn placement(&self) -> Result<Self::Placement> {
        match self {
            FixedTensor::RingTensor(x) => Ok(x.placement()?.into()),
            FixedTensor::ReplicatedTensor(x) => Ok(x.placement()?.into()),
        }
    }
}

pub trait Convert<T> {
    type Scale: One + Clone;
    fn encode(x: &T, scaling_factor: Self::Scale) -> Self;
    fn decode(x: &Self, scaling_factor: Self::Scale) -> T;
}

impl Convert<Float64Tensor> for Ring64Tensor {
    type Scale = u64;
    fn encode(x: &Float64Tensor, scaling_factor: Self::Scale) -> Ring64Tensor {
        let x_upshifted = &x.0 * (scaling_factor as f64);
        let x_converted: ArrayD<u64> = x_upshifted.mapv(|el| (el as i64) as u64);
        Ring64Tensor::from(x_converted)
    }
    fn decode(x: &Self, scaling_factor: Self::Scale) -> Float64Tensor {
        let x_upshifted: ArrayD<i64> = x.into();
        let x_converted = x_upshifted.mapv(|el| el as f64);
        Float64Tensor::from(x_converted / scaling_factor as f64)
    }
}

impl Convert<Float64Tensor> for Ring128Tensor {
    type Scale = u128;
    fn encode(x: &Float64Tensor, scaling_factor: Self::Scale) -> Ring128Tensor {
        let x_upshifted = &x.0 * (scaling_factor as f64);
        let x_converted: ArrayD<u128> = x_upshifted.mapv(|el| (el as i128) as u128);
        Ring128Tensor::from(x_converted)
    }

    fn decode(x: &Self, scaling_factor: Self::Scale) -> Float64Tensor {
        let x_upshifted: ArrayD<i128> = x.into();
        let x_converted = x_upshifted.mapv(|el| el as f64);
        Float64Tensor::from(x_converted / scaling_factor as f64)
    }
}

impl<T> AbstractRingTensor<T>
where
    Wrapping<T>: Clone + Zero + Mul<Wrapping<T>, Output = Wrapping<T>>,
    AbstractRingTensor<T>: Convert<Float64Tensor>,
{
    pub fn ring_mean(
        x: Self,
        axis: Option<usize>,
        scaling_factor: <AbstractRingTensor<T> as Convert<Float64Tensor>>::Scale,
    ) -> AbstractRingTensor<T> {
        let mean_weight = Self::compute_mean_weight(&x, &axis);
        let encoded_weight = AbstractRingTensor::<T>::encode(&mean_weight, scaling_factor);
        let operand_sum = x.sum(axis);
        operand_sum.mul(encoded_weight)
    }

    fn compute_mean_weight(x: &Self, &axis: &Option<usize>) -> Float64Tensor {
        let shape: &[usize] = x.0.shape();
        if let Some(ax) = axis {
            let dim_len = shape[ax] as f64;
            Float64Tensor::from(
                Array::from_elem([], 1.0 / dim_len)
                    .into_dimensionality::<IxDyn>()
                    .unwrap(),
            )
        } else {
            let dim_prod: usize = std::iter::Product::product(shape.iter());
            let prod_inv = 1.0 / dim_prod as f64;
            Float64Tensor::from(
                Array::from_elem([], prod_inv)
                    .into_dimensionality::<IxDyn>()
                    .unwrap(),
            )
        }
    }
}

modelled!(PlacementRingMean::ring_mean, HostPlacement, attributes[axis: Option<u32>, scaling_base: u64, scaling_exp: u32] (Ring64Tensor) -> Ring64Tensor, FixedpointRingMeanOp);
modelled!(PlacementRingMean::ring_mean, HostPlacement, attributes[axis: Option<u32>, scaling_base: u64, scaling_exp: u32] (Ring128Tensor) -> Ring128Tensor, FixedpointRingMeanOp);

kernel! {
    FixedpointRingMeanOp,
    [
        (HostPlacement, (Ring64Tensor) -> Ring64Tensor => attributes[axis, scaling_base, scaling_exp] Self::kernel_ring64tensor),
        (HostPlacement, (Ring128Tensor) -> Ring128Tensor => attributes[axis, scaling_base, scaling_exp] Self::kernel_ring128tensor),
    ]
}

impl FixedpointRingMeanOp {
    fn kernel_ring64tensor<S: RuntimeSession>(
        sess: &S,
        plc: &HostPlacement,
        axis: Option<u32>,
        scaling_base: u64,
        scaling_exp: u32,
        x: Ring64Tensor,
    ) -> Ring64Tensor
    where
        HostPlacement: PlacementPlace<S, Ring64Tensor>,
    {
        let scaling_factor = u64::pow(scaling_base, scaling_exp);
        let axis = axis.map(|a| a as usize);
        let mean = Ring64Tensor::ring_mean(x, axis, scaling_factor);
        plc.place(sess, mean)
    }

    fn kernel_ring128tensor<S: RuntimeSession>(
        sess: &S,
        plc: &HostPlacement,
        axis: Option<u32>,
        scaling_base: u64,
        scaling_exp: u32,
        x: Ring128Tensor,
    ) -> Ring128Tensor
    where
        HostPlacement: PlacementPlace<S, Ring128Tensor>,
    {
        let scaling_factor = u128::pow(scaling_base as u128, scaling_exp);
        let axis = axis.map(|a| a as usize);
        let mean = Ring128Tensor::ring_mean(x, axis, scaling_factor);
        plc.place(sess, mean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_fixedpoint() {
        let x = Float64Tensor::from(
            array![1.0, -2.0, 3.0, -4.0]
                .into_dimensionality::<IxDyn>()
                .unwrap(),
        );

        let scaling_factor = 2u64.pow(16);
        let x_encoded = Ring64Tensor::encode(&x, scaling_factor);
        assert_eq!(
            x_encoded,
            Ring64Tensor::from(vec![
                65536,
                18446744073709420544,
                196608,
                18446744073709289472
            ])
        );

        let x_decoded = Ring64Tensor::decode(&x_encoded, scaling_factor);
        assert_eq!(x_decoded, x);

        let scaling_factor_long = 2u128.pow(80);
        let x_encoded = Ring128Tensor::encode(&x, scaling_factor_long);
        assert_eq!(
            x_encoded,
            Ring128Tensor::from(vec![
                1208925819614629174706176,
                340282366920936045611735378173418799104,
                3626777458843887524118528,
                340282366920933627760096148915069386752
            ])
        );

        let x_decoded_long = Ring128Tensor::decode(&x_encoded, scaling_factor_long);
        assert_eq!(x_decoded_long, x);
    }

    #[test]
    fn ring_mean_with_axis() {
        let x_backing: Float64Tensor = Float64Tensor::from(
            array![[1., 2.], [3., 4.]]
                .into_dimensionality::<IxDyn>()
                .unwrap(),
        );
        let encoding_factor = 2u64.pow(16);
        let decoding_factor = 2u64.pow(32);
        let x = Ring64Tensor::encode(&x_backing, encoding_factor);
        let out = Ring64Tensor::ring_mean(x, Some(0), encoding_factor);
        let dec = Ring64Tensor::decode(&out, decoding_factor);
        assert_eq!(
            dec,
            Float64Tensor::from(array![2., 3.].into_dimensionality::<IxDyn>().unwrap())
        );
    }

    #[test]
    fn ring_mean_no_axis() {
        let x_backing: Float64Tensor = Float64Tensor::from(
            array![[1., 2.], [3., 4.]]
                .into_dimensionality::<IxDyn>()
                .unwrap(),
        );
        let encoding_factor = 2u64.pow(16);
        let decoding_factor = 2u64.pow(32);
        let x = Ring64Tensor::encode(&x_backing, encoding_factor);
        let out = Ring64Tensor::ring_mean(x, None, encoding_factor);
        let dec = Ring64Tensor::decode(&out, decoding_factor);
        assert_eq!(
            dec.0.into_shape((1,)).unwrap(),
            array![2.5].into_shape((1,)).unwrap()
        );
    }
}
