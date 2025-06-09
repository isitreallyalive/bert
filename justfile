alias c := cargo

[doc("cargo wrapper")]
@cargo package command *args:
  cargo {{command}} -p bert-{{package}} {{args}}