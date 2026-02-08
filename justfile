gen-ts-schema:
  @echo "Generating TypeScript schema..."
  cd shared && cargo run --bin gen-ts-schemas -F ts-rs,sql
  @echo "Moving file from shared/bindings/generated to panel/src/types"
  if [ ! -d panel/src/types ]; then mkdir -p panel/src/types; fi
  mv shared/bindings/generated/*.ts panel/src/types/
  @echo "Removing old generated files..."
  rm -rf shared/bindings/generated/*.ts
  @echo "TypeScript schema generation completed."