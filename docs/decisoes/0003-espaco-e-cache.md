# 0003 — Política de espaço, cache e limpeza

- Data: 2026-11-18 — Status: Adotado (operacional)
- Cache compartilhado: /mnt/disco/doors/cache/lb-trixie (bootstrap + pacotes .deb), usado por todas as edições. Nunca apagar sem verificar.
- Trabalho: /mnt/disco/doors/build/<edição> é descartável e recriado a cada build (~3–8 GB por edição).
- ISOs: /mnt/disco/doors/isos — manter as 2 últimas por edição; versões candidatas/release nunca são removidas automaticamente.
- Antes de cada build: exigir ≥ 40 GB livres em /mnt/disco/doors.
- Limpeza do cache de pacotes: quando > 30 GB, remover .deb não referenciados pelos builds atuais.
- Paralelismo: uma edição por vez (swap do host é 512 MiB).
- Temporários nunca em /tmp (tmpfs).
