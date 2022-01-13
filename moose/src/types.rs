//! Moose recognized types

use crate::additive::{AdtShape, AdtTensor};
use crate::boolean::BoolTensor;
use crate::encrypted::{AbstractAesKey, AbstractAesTensor, FixedAesTensor};
use crate::fixedpoint::FixedTensor;
use crate::floatingpoint::FloatTensor;
use crate::host::{
    AbstractHostAesKey, AbstractHostFixedAesTensor, AbstractHostFixedTensor,
    AbstractHostRingTensor, HostBitArray128, HostBitArray224, HostTensor,
};
use crate::logical::AbstractTensor;
use crate::mirrored::{AbstractMirroredFixedTensor, Mirrored3Tensor};
use crate::replicated::{
    AbstractReplicatedAesKey, AbstractReplicatedFixedTensor, AbstractReplicatedRingTensor,
    AbstractReplicatedSetup, AbstractReplicatedShape, ReplicatedBitArray128,
};
pub use crate::{
    host::{HostShape, HostString},
    prim::{PrfKey, Seed},
};

// Logical types

pub type Tensor =
    AbstractTensor<Fixed64Tensor, Fixed128Tensor, Float32Tensor, Float64Tensor, BooleanTensor>;

moose_type!(Fixed64Tensor = FixedTensor<HostFixed64Tensor, Mirrored3Fixed64Tensor, ReplicatedFixed64Tensor>);
moose_type!(HostFixed64Tensor = AbstractHostFixedTensor<HostRing64Tensor>);
moose_type!(Mirrored3Fixed64Tensor = AbstractMirroredFixedTensor<Mirrored3Ring64Tensor>);
moose_type!(ReplicatedFixed64Tensor = AbstractReplicatedFixedTensor<ReplicatedRing64Tensor>);

moose_type!(Fixed128Tensor = FixedTensor<HostFixed128Tensor, Mirrored3Fixed128Tensor, ReplicatedFixed128Tensor>);
moose_type!(HostFixed128Tensor = AbstractHostFixedTensor<HostRing128Tensor>);
moose_type!(Mirrored3Fixed128Tensor = AbstractMirroredFixedTensor<Mirrored3Ring128Tensor>);
moose_type!(ReplicatedFixed128Tensor = AbstractReplicatedFixedTensor<ReplicatedRing128Tensor>);

moose_type!(Float32Tensor = FloatTensor<HostFloat32Tensor, Mirrored3Float32>);
moose_type!(HostFloat32Tensor = [atomic] HostTensor<f32>);
moose_type!(Mirrored3Float32 = Mirrored3Tensor<HostFloat32Tensor>);

moose_type!(Float64Tensor = FloatTensor<HostFloat64Tensor, Mirrored3Float64>);
moose_type!(HostFloat64Tensor = [atomic] HostTensor<f64>);
moose_type!(Mirrored3Float64 = Mirrored3Tensor<HostFloat64Tensor>);

moose_type!(BooleanTensor = BoolTensor<HostBitTensor, ReplicatedBitTensor>);
pub use crate::host::HostBitTensor;
moose_type!(ReplicatedBitTensor = AbstractReplicatedRingTensor<HostBitTensor>);

// Encrypted types

moose_type!(AesTensor = AbstractAesTensor<Fixed128AesTensor>);
moose_type!(Fixed128AesTensor = FixedAesTensor<HostFixed128AesTensor>);
moose_type!(HostFixed128AesTensor = AbstractHostFixedAesTensor<HostBitArray224>);

moose_type!(AesKey = AbstractAesKey<HostAesKey, ReplicatedAesKey>);
moose_type!(HostAesKey = AbstractHostAesKey<HostBitArray128>);
moose_type!(ReplicatedAesKey = AbstractReplicatedAesKey<ReplicatedBitArray128>);

// Ring types

moose_type!(ReplicatedRing64Tensor = AbstractReplicatedRingTensor<HostRing64Tensor>);
moose_type!(AdditiveRing64Tensor = AdtTensor<HostRing64Tensor>);
moose_type!(Mirrored3Ring64Tensor = Mirrored3Tensor<HostRing64Tensor>);
moose_type!(HostRing64Tensor = [atomic] AbstractHostRingTensor<u64>);

moose_type!(ReplicatedRing128Tensor = AbstractReplicatedRingTensor<HostRing128Tensor>);
moose_type!(AdditiveRing128Tensor = AdtTensor<HostRing128Tensor>);
moose_type!(Mirrored3Ring128Tensor = Mirrored3Tensor<HostRing128Tensor>);
moose_type!(HostRing128Tensor = [atomic] AbstractHostRingTensor<u128>);

// Misc mirrored types

moose_type!(Mirrored3BitTensor = Mirrored3Tensor<HostBitTensor>);

// Mist additive types

moose_type!(AdditiveBitTensor = AdtTensor<HostBitTensor>);
moose_type!(AdditiveShape = AdtShape<HostShape>);

// Misc replicated types

moose_type!(ReplicatedShape = AbstractReplicatedShape<HostShape>);
moose_type!(ReplicatedSetup = AbstractReplicatedSetup<PrfKey>);

// Misc host types

moose_type!(HostInt8Tensor = [atomic] HostTensor<i8>);
moose_type!(HostInt16Tensor = [atomic] HostTensor<i16>);
moose_type!(HostInt32Tensor = [atomic] HostTensor<i32>);
moose_type!(HostInt64Tensor = [atomic] HostTensor<i64>);
moose_type!(HostUint8Tensor = [atomic] HostTensor<u8>);
moose_type!(HostUint16Tensor = [atomic] HostTensor<u16>);
moose_type!(HostUint32Tensor = [atomic] HostTensor<u32>);
moose_type!(HostUint64Tensor = [atomic] HostTensor<u64>);

moose_type!(PrfKey);
moose_type!(Seed);
moose_type!(HostBitTensor);
moose_type!(HostString);
moose_type!(HostShape);
