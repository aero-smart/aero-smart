gen-ts-schema:
  @echo "Generating TypeScript schema..."
  cd shared && cargo run --bin gen-ts-schemas -F ts-rs,sql
  @echo "Copying the schema from bindings/generated to panel/src/schemas..."
  @echo "Removing old schemas..."
  rm -rf panel/src/schemas/*
  if [ ! -d panel/src/schemas ]; then mkdir -p panel/src/schemas; fi
  cp bindings/generated/* panel/src/schemas/
  @echo "Removing temporary files..."
  rm -rf bindings/generated/*
  @echo "TypeScript schema generation completed."