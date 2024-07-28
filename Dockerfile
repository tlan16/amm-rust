FROM ghcr.io/joaovarelas/obfuscator-llvm-16.0:latest

WORKDIR /app
COPY . .

ENTRYPOINT \
    cargo rustc \
      -gnu \
      --release \
      -- \
      -Cdebuginfo=0 \
      -Cstrip=symbols \
      -Cpanic=abort \
      -Copt-level=3 \
      -Cllvm-args=-enable-allobf
