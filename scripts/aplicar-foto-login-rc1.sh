#!/bin/bash
# Doors OS — aplica numa rc1/rc2 já instalada a foto do usuário na tela de login (teste do PR #3).
#   sudo bash aplicar-foto-login.sh             aplica
#   sudo bash aplicar-foto-login.sh --desfazer  volta como estava
# Depois de aplicar ou desfazer: reiniciar o computador.
set -eu
[ "$(id -u)" = 0 ] || { echo "Rode com sudo: sudo bash $0 ${1:-}"; exit 1; }
R=https://raw.githubusercontent.com/1tbps/doors-os/335d4f7
CONF=/etc/sddm.conf.d/zz-doors-foto.conf
TEMA=/usr/share/sddm/themes/doors

if [ "${1:-}" = --desfazer ]; then
  rm -f "$CONF"
  rm -rf "$TEMA"
  systemctl disable --now doors-fotos-login.path doors-fotos-login.service 2>/dev/null || true
  rm -f /etc/systemd/system/doors-fotos-login.path /etc/systemd/system/doors-fotos-login.service
  systemctl daemon-reload
  rm -f /usr/lib/doors/fotos-login
  rm -rf /var/lib/doors/fotos-login
  if [ -f /usr/share/pam-configs/doors-ad-foto ]; then
    pam-auth-update --package --remove doors-ad-foto
    rm -f /usr/share/pam-configs/doors-ad-foto
  fi
  rm -f /usr/lib/doors/ad-registrar-usuario
  echo "Desfeito. Reinicie o computador."
  exit 0
fi

[ -d /usr/share/sddm/themes/breeze ] || { echo "Falta o tema breeze do SDDM; nada foi mudado."; exit 1; }

echo "Baixando os arquivos..."
T=$(mktemp -d)
trap 'rm -rf "$T"' EXIT
baixar() { curl -fsSL "$R/$1" -o "$T/$2" && [ -s "$T/$2" ] || { echo "Falhou ao baixar $1; nada foi mudado."; exit 1; }; }
baixar packages/doors-os/kde/sddm/doors/Main.qml Main.qml
baixar packages/doors-os/kde/sddm/doors/DoorsLogin.qml DoorsLogin.qml
baixar packages/doors-os/kde/sddm/doors/metadata.desktop metadata.desktop
baixar packages/doors-os/kde/sddm/doors/theme.conf theme.conf
baixar config/desktop-common/includes.chroot/usr/lib/doors/fotos-login fotos-login
baixar config/desktop-common/includes.chroot/usr/lib/doors/ad-registrar-usuario ad-registrar-usuario
baixar config/desktop-common/includes.chroot/usr/share/pam-configs/doors-ad-foto doors-ad-foto
baixar config/desktop-common/includes.chroot/etc/systemd/system/doors-fotos-login.path doors-fotos-login.path
baixar config/desktop-common/includes.chroot/etc/systemd/system/doors-fotos-login.service doors-fotos-login.service

echo "Instalando..."
install -d "$TEMA" /usr/lib/doors
install -m 644 "$T"/Main.qml "$T"/DoorsLogin.qml "$T"/metadata.desktop "$T"/theme.conf "$TEMA"/
install -m 755 "$T"/fotos-login "$T"/ad-registrar-usuario /usr/lib/doors/
install -m 644 "$T"/doors-ad-foto /usr/share/pam-configs/
install -m 644 "$T"/doors-fotos-login.path "$T"/doors-fotos-login.service /etc/systemd/system/

# usuário do domínio nas Configurações > Usuários (a cada login)
pam-auth-update --package --enable doors-ad-foto
# fotos copiadas para a tela de login (agora e sempre que mudarem)
systemctl daemon-reload
systemctl enable --now doors-fotos-login.path doors-fotos-login.service

# tela de login: tema doors e, como na rc2, sem lista (abre com o último usuário e a foto dele)
install -d /etc/sddm.conf.d
{
  echo "# Doors OS: teste da foto na tela de login (aplicar-foto-login.sh). Apagar para desfazer."
  if [ ! -f /etc/sddm.conf.d/doors-dominio.conf ]; then
    printf '[Users]\nMaximumUid=999\nRememberLastUser=true\n\n'
  fi
  printf '[Theme]\nCurrent=doors\n'
} > "$CONF"

echo
echo "Pronto. Fotos copiadas para a tela de login:"
ls /var/lib/doors/fotos-login/ 2>/dev/null || true
echo "Reinicie o computador para ver a nova tela de login."
