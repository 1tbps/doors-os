#!/bin/bash
# Doors OS (KDE): teste do contorno "flutuando" (vão + contorno de 3 px por fora).
#   bash teste-contorno.sh ligar [vão] -> liga o contorno flutuante (vão em px, padrão 2)
#   bash teste-contorno.sh desligar  -> volta ao contorno normal (2 px por dentro)
# Precisa do pacote doors-kwin-cantos 0.11.0+doors1~teste1 (ou mais novo) instalado.
set -e
G=(--file kwinrc --group Round-Corners)
recarregar() {
  gdbus call --session --dest org.kde.KWin --object-path /Effects \
    --method org.kde.kwin.Effects.reconfigureEffect kwin4_effect_shapecorners >/dev/null
}
case "$1" in
  ligar)
    cor=$(kreadconfig6 "${G[@]}" --key OutlineColor --default 47,120,181)
    kwriteconfig6 "${G[@]}" --key OutlineThickness 0
    kwriteconfig6 "${G[@]}" --key InactiveOutlineThickness 0
    kwriteconfig6 "${G[@]}" --key OuterOutlineGap "${2:-2}"
    kwriteconfig6 "${G[@]}" --key OuterOutlineThickness 3
    kwriteconfig6 "${G[@]}" --key OuterOutlineColor "$cor"
    kwriteconfig6 "${G[@]}" --key ActiveOuterOutlineUsePalette false
    kwriteconfig6 "${G[@]}" --key ActiveOuterOutlineUseCustom true
    kwriteconfig6 "${G[@]}" --key ActiveOuterOutlineAlpha 255
    kwriteconfig6 "${G[@]}" --key InactiveOuterOutlineThickness 3
    kwriteconfig6 "${G[@]}" --key InactiveOuterOutlineColor 0,0,0
    kwriteconfig6 "${G[@]}" --key InactiveOuterOutlineUsePalette false
    kwriteconfig6 "${G[@]}" --key InactiveOuterOutlineUseCustom true
    kwriteconfig6 "${G[@]}" --key InactiveOuterOutlineAlpha 60
    recarregar; echo "Contorno flutuante ligado (vão de ${2:-2} px, contorno de 3 px)." ;;
  desligar)
    for k in OutlineThickness InactiveOutlineThickness OuterOutlineGap OuterOutlineThickness \
             OuterOutlineColor ActiveOuterOutlineUsePalette ActiveOuterOutlineUseCustom ActiveOuterOutlineAlpha \
             InactiveOuterOutlineThickness InactiveOuterOutlineColor InactiveOuterOutlineUsePalette \
             InactiveOuterOutlineUseCustom InactiveOuterOutlineAlpha; do
      kwriteconfig6 "${G[@]}" --key "$k" --delete
    done
    recarregar; echo "Contorno normal de volta." ;;
  *) echo "uso: bash $0 ligar | desligar"; exit 1 ;;
esac
