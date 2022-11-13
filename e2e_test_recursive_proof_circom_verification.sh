echo "****GENERATING RECURSIVE PLONKY2 PROOF****"
cargo test -r --color=always --package plonky2_circom_verifier --lib verifier::tests::test_recursive_verifier --no-fail-fast -- -Z unstable-options --show-output
echo "DONE ($((end - start))s)"
cd circom/e2e_tests && ./run.sh && cd .. && cd ..
