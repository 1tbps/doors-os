# Doors OS (KDE Plasma, X11 e Wayland): ' + c = ç no teclado internacional.
# Usuários sem ~/.XCompose próprio usam a tabela do Doors.
if [ -z "${XCOMPOSEFILE:-}" ] && [ ! -e "$HOME/.XCompose" ]; then
  XCOMPOSEFILE=/usr/share/doors/XCompose
  export XCOMPOSEFILE
fi
