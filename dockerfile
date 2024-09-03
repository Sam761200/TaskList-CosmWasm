FROM cosmwasm/workspace-optimizer:0.14.0

# Définir le dossier de travail
WORKDIR /code

# Commande par défaut pour compiler les contrats
CMD ["cargo", "wasm"]
