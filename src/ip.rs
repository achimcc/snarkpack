use crate::Error;
use ark_ec::{
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
    AffineRepr, CurveGroup, Group, VariableBaseMSM,
};
// {msm::VariableBaseMSM, AffineCurve, PairingEngine};
use ark_ff::PrimeField;
use ark_std::{cfg_iter, vec::Vec};
use rayon::prelude::*;

pub(crate) fn pairing_miller_affine<E: Pairing>(
    left: &[E::G1Affine],
    right: &[E::G2Affine],
) -> Option<MillerLoopOutput<E>> {
    if left.len() != right.len() {
        return None;
    }
    let left = left
        .iter()
        .map(|e| E::G1Prepared::from(*e))
        .collect::<Vec<_>>();
    let right = right
        .iter()
        .map(|e| E::G2Prepared::from(*e))
        .collect::<Vec<_>>();
    // ToDo: remove
    // let pairs: Vec<(E::G1Prepared, E::G2Prepared)> = left
    //     .par_iter()
    //     .map(|e| E::G1Prepared::from(*e))
    //     .zip(right.par_iter().map(|e| E::G2Prepared::from(*e)))
    //     .collect::<Vec<_>>();

    Some(E::multi_miller_loop(left, right))
}

/// Returns the miller loop result of the inner pairing product
pub(crate) fn pairing<E: Pairing>(
    left: &[E::G1Affine],
    right: &[E::G2Affine],
) -> Option<PairingOutput<E>> {
    E::final_exponentiation(pairing_miller_affine::<E>(left, right)?)
}

pub(crate) fn multiexponentiation<G: AffineRepr>(
    left: &[G],
    right: &[G::ScalarField],
) -> Result<G::Group, usize> {
    if left.len() != right.len() {
        // ToDo: Error type usize, 0???
        return Err(0);
    }
    VariableBaseMSM::msm(left, right)
}
