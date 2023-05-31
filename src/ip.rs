use crate::Error;
use ark_ec::{
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
    AffineRepr, VariableBaseMSM,
};
use ark_std::vec::Vec;

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
) -> Result<PairingOutput<E>, Error> {
    let miller_result = pairing_miller_affine::<E>(left, right).ok_or(Error::InvalidPairing)?;
    E::final_exponentiation(miller_result).ok_or(Error::InvalidPairing)
}

pub(crate) fn multiexponentiation<G: AffineRepr>(
    left: &[G],
    right: &[G::ScalarField],
) -> Result<G::Group, Error> {
    if left.len() != right.len() {
        // ToDo: check Error type!!!
        return Err(Error::InvalidKeyLength);
    }
    VariableBaseMSM::msm(left, right).map_err(|_| Error::InvalidIPVectorLength)
}
