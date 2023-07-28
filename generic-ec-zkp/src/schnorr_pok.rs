//! Schnorr Proof of Knowledge $\Pi^\text{sch}$
//!
//! Schnorr Proof of Knowledge is an interactive $\Sigma$ protocol that lets prover $\P$ convince
//! verifier $\V$ that it knows secret $x$ such as $X = x \cdot G$.
//!
//! ## Example
//!
//! 0. $\P$ knows a secret $x$ and wants to prove its knowledge.
//!    ```rust
//!    # use generic_ec::{Curve, SecretScalar, Point};
//!    # use rand::rngs::OsRng;
//!    # fn doc_fn<E: Curve>() {
//!    let x = SecretScalar::<E>::random(&mut OsRng);
//!    let X = Point::generator() * &x; // assumed to be known by verifier
//!    # }
//!    ```
//! 1. $\P$ generates and commits an ephemeral secret. Committed secret is sent to $\V$.
//!    ```rust
//!    # use generic_ec::Curve;
//!    # use generic_ec_zkp::schnorr_pok::*;
//!    # use rand::rngs::OsRng;
//!    # fn doc_fn<E: Curve>() {
//!    let (eph_secret, commit) = prover_commits_ephemeral_secret::<E, _>(&mut OsRng);
//!    send(commit);
//!    # }
//!    # fn send<T>(_: T) {}
//!    ```
//! 2. $\V$ receives commitment, and responds with challenge.
//!    ```rust
//!    # use generic_ec::Curve;
//!    # use generic_ec_zkp::schnorr_pok::*;
//!    # use rand::rngs::OsRng;
//!    # fn doc_fn<E: Curve>() {
//!    let commit: Commit<E> = receive();
//!    let challenge = Challenge::<E>::generate(&mut OsRng);
//!    send(challenge);
//!    # }
//!    # fn send<T>(_: T) {}
//!    # fn receive<T>() -> T { unimplemented!() }
//!    ```
//! 3. $\P$ receives a challenge and responds with proof.
//!    ```rust
//!    # use generic_ec::{Curve, SecretScalar};
//!    # use generic_ec_zkp::schnorr_pok::*;
//!    # use rand::rngs::OsRng;
//!    # fn doc_fn<E: Curve>() {
//!    # let (eph_secret, x): (ProverSecret<E>, SecretScalar<E>) = recall();
//!    let challenge: Challenge<E> = receive();
//!    let proof = prove(&eph_secret, &challenge, &x);
//!    send(proof);
//!    # }
//!    # fn send<T>(_: T) {}
//!    # fn receive<T>() -> T { unimplemented!() }
//!    # fn recall<T>() -> T { unimplemented!() }
//!    ```
//! 4. $\V$ receives a proof and verifies it.
//!    ```rust
//!    # use generic_ec::{Curve, Point};
//!    # use generic_ec_zkp::schnorr_pok::*;
//!    # use rand::rngs::OsRng;
//!    # fn doc_fn<E: Curve>() {
//!    # let (commit, challenge, X): (Commit<E>, Challenge<E>, Point<E>) = recall();
//!    let proof: Proof<E> = receive();
//!    proof.verify(&commit, &challenge, &X);
//!    # }
//!    # fn send<T>(_: T) {}
//!    # fn receive<T>() -> T { unimplemented!() }
//!    # fn recall<T>() -> T { unimplemented!() }
//!    ```
//!
//! ## Algorithm
//!
//! Schnor PoK is defined as:
//!
//! * Prove
//!   1. Prover samples $\alpha \gets \Z_q$ and sends $A = \alpha \cdot G$ to verifier
//!   2. Verifier replies with $e \gets \Z_q$
//!   3. Prover sends $z = \alpha + ex$
//! * Verification \
//!   Verifier checks that $z \cdot G \\? A + e \cdot X$

use generic_ec::{Curve, Point, Scalar, SecretScalar};
use rand_core::{CryptoRng, RngCore};
use subtle::ConstantTimeEq;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Committed prover ephemeral secret
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(bound = ""))]
pub struct Commit<E: Curve>(pub Point<E>);

/// Prover ephemeral secret
pub struct ProverSecret<E: Curve> {
    pub nonce: SecretScalar<E>,
}

/// Challenge generated by verifier
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(bound = ""))]
pub struct Challenge<E: Curve> {
    pub nonce: Scalar<E>,
}

impl<E: Curve> Challenge<E> {
    /// Generates a random challenge
    pub fn generate<R: RngCore>(rng: &mut R) -> Self {
        Self {
            nonce: Scalar::random(rng),
        }
    }
}

/// The proof that can convince $\V$ that $\P$ knows secret $x$
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(bound = ""))]
pub struct Proof<E: Curve>(pub Scalar<E>);

impl<E: Curve> Proof<E> {
    /// Verifies that prover knows secret $x$ such as $X = x \cdot G$
    #[allow(non_snake_case)]
    pub fn verify(
        &self,
        commit: &Commit<E>,
        challenge: &Challenge<E>,
        X: &Point<E>,
    ) -> Result<(), InvalidProof> {
        let lhs = Point::generator() * self.0;
        let rhs = commit.0 + challenge.nonce * X;
        if lhs.ct_eq(&rhs).into() {
            Ok(())
        } else {
            Err(InvalidProof)
        }
    }
}

/// Generates and commits prover ephemeral secret
pub fn prover_commits_ephemeral_secret<E: Curve, R: RngCore + CryptoRng>(
    rng: &mut R,
) -> (ProverSecret<E>, Commit<E>) {
    let secret = SecretScalar::random(rng);
    let public = Point::generator() * &secret;
    (ProverSecret { nonce: secret }, Commit(public))
}

/// Proves knowledge of `secret`
pub fn prove<E: Curve>(
    committed_secret: &ProverSecret<E>,
    challenge: &Challenge<E>,
    secret: impl AsRef<Scalar<E>>,
) -> Proof<E> {
    Proof(&committed_secret.nonce + challenge.nonce * secret.as_ref())
}

/// Invalid proof error
#[derive(Debug, Clone, Copy)]
pub struct InvalidProof;

impl core::fmt::Display for InvalidProof {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid Schnorr PoK proof")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidProof {}
