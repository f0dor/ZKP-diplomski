use curve25519_dalek_ng::scalar::Scalar;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use merlin::Transcript;
use rand::rngs::OsRng;

fn main() {
    // === Setup Section ===
    /*
    In this section, we define the secret value (real_value) that we want to prove lies within the range [18, 100].
    We also generate random blinding factors for both proofs and define the range limits (min = 18, max = 100).
    We then calculate the shifts needed to create the proofs.
    */
    let real_value: u64 = 25; // The secret value to prove (between 18 and 100)
    let blinding1 = Scalar::random(&mut OsRng);
    let blinding2 = Scalar::random(&mut OsRng);

    let shift_min = 18;
    let shift_max = 100;

    // Calculate the shifted values to create the range proofs
    let value_minus_min = real_value - shift_min;
    let max_minus_value = shift_max - real_value;

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
     
    // === Proof 1 Generation (≥18) ===
    /*
    Here we generate the first proof that proves the real value is greater than or equal to 18.
    We use a range proof to show this while keeping the real value private using the blinding factor.
    */
    let mut transcript1 = Transcript::new(b"ValueAbove18");
    let (proof1, committed_value1) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut transcript1,
        value_minus_min,
        &blinding1,
        8,
    ).expect("Proof 1 generation failed");

    // === Proof 2 Generation (≤100) ===
    /*
    This section generates the second proof that proves the real value is less than or equal to 100.
    Similar to Proof 1, we use a range proof but this time showing the value is below the upper limit.
    */
    let mut transcript2 = Transcript::new(b"ValueBelow100");
    let (proof2, committed_value2) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut transcript2,
        max_minus_value,
        &blinding2,
        8,
    ).expect("Proof 2 generation failed");

    println!("Generated two proofs!");

    // === Verification Section ===
    /*
    This section simulates the verifier side where we verify both range proofs.
    We verify that the proofs are valid and check if the real value is within the expected range.
    */
    let mut verifier_transcript1 = Transcript::new(b"ValueAbove18");
    let result1 = proof1.verify_single(
        &bp_gens,
        &pc_gens,
        &mut verifier_transcript1,
        &committed_value1,
        8,
    );

    let mut verifier_transcript2 = Transcript::new(b"ValueBelow100");
    let result2 = proof2.verify_single(
        &bp_gens,
        &pc_gens,
        &mut verifier_transcript2,
        &committed_value2,
        8,
    );

    // === Final Output ===
    /*
    In this section, we print the verification results for both proofs.
    If both proofs are verified successfully, it confirms that the real value is indeed between 18 and 100.
    */
    println!("Proof 1 (≥18) verification: {:?}", result1.is_ok());
    println!("Proof 2 (≤100) verification: {:?}", result2.is_ok());

    // If both proofs are valid, confirm the value is within the range, otherwise output an error
    if result1.is_ok() && result2.is_ok() {
        println!("✅ Real value is between 18 and 100!");
    } else {
        println!("❌ Verification failed.");
    }
}
