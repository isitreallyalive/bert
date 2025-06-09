alias c := cargo

[doc("cargo wrapper")]
@cargo command package *args:
  cargo {{command}} -p bert-{{package}} {{args}}