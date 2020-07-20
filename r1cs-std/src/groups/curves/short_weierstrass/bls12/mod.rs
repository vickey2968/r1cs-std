use algebra::{
    curves::bls12::{Bls12Parameters, G1Prepared, G2Prepared, TwistType},
    fields::Field,
    BitIterator, One, ProjectiveCurve,
};
use r1cs_core::{ConstraintSystem, SynthesisError};

use crate::{
    fields::{fp::FpGadget, fp2::Fp2Gadget, FieldGadget},
    groups::curves::short_weierstrass::AffineGadget,
    prelude::*,
    Vec,
};

use core::{borrow::Borrow, fmt::Debug, ops::Mul};

pub type G1Gadget<P> = AffineGadget<
    <P as Bls12Parameters>::G1Parameters,
    <P as Bls12Parameters>::Fp,
    FpGadget<<P as Bls12Parameters>::Fp>,
>;

pub type G2Gadget<P> =
    AffineGadget<<P as Bls12Parameters>::G2Parameters, <P as Bls12Parameters>::Fp, Fp2G<P>>;

#[derive(Derivative)]
#[derivative(
    Clone(bound = "G1Gadget<P>: Clone"),
    Debug(bound = "G1Gadget<P>: Debug")
)]
pub struct G1PreparedGadget<P: Bls12Parameters>(pub G1Gadget<P>);

impl<P: Bls12Parameters> AllocGadget<G1Prepared<P>, P::Fp> for G1PreparedGadget<P> {
    fn alloc_constant<T, CS: ConstraintSystem<P::Fp>>(
        mut cs: CS,
        t: T,
    ) -> Result<Self, SynthesisError>
    where
        T: Borrow<G1Prepared<P>>,
    {
        let obj = t.borrow();

        Ok(Self(G1Gadget::<P>::alloc_constant(
            &mut cs.ns(|| "g1"),
            &obj.0.into(),
        )?))
    }

    fn alloc<F, T, CS: ConstraintSystem<P::Fp>>(_cs: CS, _f: F) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<G1Prepared<P>>,
    {
        todo!()
    }

    fn alloc_input<F, T, CS: ConstraintSystem<P::Fp>>(
        _cs: CS,
        _f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<G1Prepared<P>>,
    {
        todo!()
    }
}

impl<P: Bls12Parameters> G1PreparedGadget<P> {
    pub fn get_value(&self) -> Option<G1Prepared<P>> {
        Some(G1Prepared::from(self.0.get_value().unwrap().into_affine()))
    }

    pub fn from_affine<CS: ConstraintSystem<P::Fp>>(
        _cs: CS,
        q: &G1Gadget<P>,
    ) -> Result<Self, SynthesisError> {
        Ok(G1PreparedGadget(q.clone()))
    }
}

impl<P: Bls12Parameters> ToBytesGadget<P::Fp> for G1PreparedGadget<P> {
    #[inline]
    fn to_bytes<CS: ConstraintSystem<P::Fp>>(
        &self,
        mut cs: CS,
    ) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.to_bytes(&mut cs.ns(|| "g_alpha to bytes"))
    }

    fn to_non_unique_bytes<CS: ConstraintSystem<P::Fp>>(
        &self,
        mut cs: CS,
    ) -> Result<Vec<UInt8>, SynthesisError> {
        self.0
            .to_non_unique_bytes(&mut cs.ns(|| "g_alpha to bytes"))
    }
}

type Fp2G<P> = Fp2Gadget<<P as Bls12Parameters>::Fp2Params, <P as Bls12Parameters>::Fp>;
type LCoeff<P> = (Fp2G<P>, Fp2G<P>);
#[derive(Derivative)]
#[derivative(
    Clone(bound = "Fp2Gadget<P::Fp2Params, P::Fp>: Clone"),
    Debug(bound = "Fp2Gadget<P::Fp2Params, P::Fp>: Debug")
)]
pub struct G2PreparedGadget<P: Bls12Parameters> {
    pub ell_coeffs: Vec<LCoeff<P>>,
}

impl<P: Bls12Parameters> AllocGadget<G2Prepared<P>, P::Fp> for G2PreparedGadget<P> {
    fn alloc_constant<T, CS: ConstraintSystem<P::Fp>>(
        mut cs: CS,
        t: T,
    ) -> Result<Self, SynthesisError>
    where
        T: Borrow<G2Prepared<P>>,
    {
        let obj = t.borrow();
        let mut res = Vec::<LCoeff<P>>::new();

        for (i, (x, y, z)) in obj.ell_coeffs.iter().enumerate() {
            let z_inverse = z.inverse().unwrap();

            let x_normalized = x.mul(&z_inverse);
            let y_normalized = y.mul(&z_inverse);

            let x_gadget =
                Fp2Gadget::alloc_constant(&mut cs.ns(|| format!("alloc_x#{}", i)), x_normalized)?;
            let y_gadget =
                Fp2Gadget::alloc_constant(&mut cs.ns(|| format!("alloc_y#{}", i)), y_normalized)?;

            res.push((x_gadget, y_gadget));
        }

        Ok(Self { ell_coeffs: res })
    }

    fn alloc<F, T, CS: ConstraintSystem<P::Fp>>(_cs: CS, _f: F) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<G2Prepared<P>>,
    {
        todo!()
    }

    fn alloc_input<F, T, CS: ConstraintSystem<P::Fp>>(
        _cs: CS,
        _f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<G2Prepared<P>>,
    {
        todo!()
    }
}

impl<P: Bls12Parameters> ToBytesGadget<P::Fp> for G2PreparedGadget<P> {
    #[inline]
    fn to_bytes<CS: ConstraintSystem<P::Fp>>(
        &self,
        mut cs: CS,
    ) -> Result<Vec<UInt8>, SynthesisError> {
        let mut bytes = Vec::new();
        for (i, coeffs) in self.ell_coeffs.iter().enumerate() {
            let mut cs = cs.ns(|| format!("Iteration {}", i));
            bytes.extend_from_slice(&coeffs.0.to_bytes(&mut cs.ns(|| "c0"))?);
            bytes.extend_from_slice(&coeffs.1.to_bytes(&mut cs.ns(|| "c1"))?);
        }
        Ok(bytes)
    }

    fn to_non_unique_bytes<CS: ConstraintSystem<P::Fp>>(
        &self,
        mut cs: CS,
    ) -> Result<Vec<UInt8>, SynthesisError> {
        let mut bytes = Vec::new();
        for (i, coeffs) in self.ell_coeffs.iter().enumerate() {
            let mut cs = cs.ns(|| format!("Iteration {}", i));
            bytes.extend_from_slice(&coeffs.0.to_non_unique_bytes(&mut cs.ns(|| "c0"))?);
            bytes.extend_from_slice(&coeffs.1.to_non_unique_bytes(&mut cs.ns(|| "c1"))?);
        }
        Ok(bytes)
    }
}

impl<P: Bls12Parameters> G2PreparedGadget<P> {
    pub fn from_affine<CS: ConstraintSystem<P::Fp>>(
        mut cs: CS,
        q: &G2Gadget<P>,
    ) -> Result<Self, SynthesisError> {
        let two_inv = P::Fp::one().double().inverse().unwrap();
        let zero = G2Gadget::<P>::zero(cs.ns(|| "zero"))?;
        q.enforce_not_equal(cs.ns(|| "enforce not zero"), &zero)?;
        let mut ell_coeffs = vec![];
        let mut r = q.clone();

        for (j, i) in BitIterator::new(P::X).skip(1).enumerate() {
            let mut cs = cs.ns(|| format!("Iteration {}", j));
            ell_coeffs.push(Self::double(cs.ns(|| "double"), &mut r, &two_inv)?);

            if i {
                ell_coeffs.push(Self::add(cs.ns(|| "add"), &mut r, &q)?);
            }
        }

        Ok(Self { ell_coeffs })
    }

    fn double<CS: ConstraintSystem<P::Fp>>(
        mut cs: CS,
        r: &mut G2Gadget<P>,
        two_inv: &P::Fp,
    ) -> Result<LCoeff<P>, SynthesisError> {
        let a = r.y.inverse(cs.ns(|| "Inverse"))?;
        let mut b = r.x.square(cs.ns(|| "square x"))?;
        let b_tmp = b.clone();
        b.mul_by_fp_constant_in_place(cs.ns(|| "mul by two_inv"), two_inv)?;
        b.add_in_place(cs.ns(|| "compute b"), &b_tmp)?;

        let c = a.mul(cs.ns(|| "compute c"), &b)?;
        let d = r.x.double(cs.ns(|| "compute d"))?;
        let x3 = c.square(cs.ns(|| "c^2"))?.sub(cs.ns(|| "sub d"), &d)?;
        let e = c
            .mul(cs.ns(|| "c*r.x"), &r.x)?
            .sub(cs.ns(|| "sub r.y"), &r.y)?;
        let c_x3 = c.mul(cs.ns(|| "c*x_3"), &x3)?;
        let y3 = e.sub(cs.ns(|| "e = c * x3"), &c_x3)?;
        let mut f = c;
        f.negate_in_place(cs.ns(|| "c = -c"))?;
        r.x = x3;
        r.y = y3;
        match P::TWIST_TYPE {
            TwistType::M => Ok((e, f)),
            TwistType::D => Ok((f, e)),
        }
    }

    fn add<CS: ConstraintSystem<P::Fp>>(
        mut cs: CS,
        r: &mut G2Gadget<P>,
        q: &G2Gadget<P>,
    ) -> Result<LCoeff<P>, SynthesisError> {
        let a =
            q.x.sub(cs.ns(|| "q.x - r.x"), &r.x)?
                .inverse(cs.ns(|| "calc a"))?;
        let b = q.y.sub(cs.ns(|| "q.y - r.y"), &r.y)?;
        let c = a.mul(cs.ns(|| "compute c"), &b)?;
        let d = r.x.add(cs.ns(|| "r.x + q.x"), &q.x)?;
        let x3 = c.square(cs.ns(|| "c^2"))?.sub(cs.ns(|| "sub d"), &d)?;

        let e =
            r.x.sub(cs.ns(|| "r.x - x3"), &x3)?
                .mul(cs.ns(|| "c * (r.x - x3)"), &c)?;
        let y3 = e.sub(cs.ns(|| "calc y3"), &r.y)?;
        let g = c
            .mul(cs.ns(|| "c*r.x"), &r.x)?
            .sub(cs.ns(|| "calc g"), &r.y)?;
        let mut f = c;
        f.negate_in_place(cs.ns(|| "c = -c"))?;
        r.x = x3;
        r.y = y3;
        match P::TWIST_TYPE {
            TwistType::M => Ok((g, f)),
            TwistType::D => Ok((f, g)),
        }
    }
}
