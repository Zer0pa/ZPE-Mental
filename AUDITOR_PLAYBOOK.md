# Auditor Playbook

Use this order for the fastest honest public audit:

1. Read [README.md](README.md) for the bounded claim, non-claims, proof anchors, and replay commands.
2. Read [docs/BOUNDED_RELEASE_SCOPE.md](docs/BOUNDED_RELEASE_SCOPE.md) for the exact public boundary.
3. Inspect [validation/corpora/endogenous_forms.json](validation/corpora/endogenous_forms.json) and [validation/corpora/non_visual_prompts.json](validation/corpora/non_visual_prompts.json) to see the committed release inputs.
4. Run the quick-start commands from the README.
5. Compare the generated [proofs/artifacts/mental_release_matrix.json](proofs/artifacts/mental_release_matrix.json) and [validation/results/pytest.xml](validation/results/pytest.xml) against the README metrics.

What a public audit can establish:

- the repo installs cleanly
- the bounded corpus round-trips exactly
- the Rust-native path matches the Python reference path
- broad cognition is explicitly kept out of scope on the public surface
