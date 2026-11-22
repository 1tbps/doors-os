# Doors OS (KDE): categorias do menu com ícones coloridos (cópias em /usr/share/doors/kde-menu)
case ":${XDG_DATA_DIRS:-/usr/local/share:/usr/share}:" in
  *:/usr/share/doors/kde-menu:*) ;;
  *) XDG_DATA_DIRS="/usr/share/doors/kde-menu:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"; export XDG_DATA_DIRS ;;
esac
