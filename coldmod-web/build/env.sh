if [ "$COLDMOD_INSECURE" = "on" ]; then
    echo "window.COLDMOD_WS='ws://$COLDMOD_WEB_HOST/ws';" > build/js/host.js
else
    echo "window.COLDMOD_WS='wss://$COLDMOD_WEB_HOST/ws';" > build/js/host.js
fi
