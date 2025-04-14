# neutralinojs-zero
An empty Neutralinojs app, extend as you wish

```
neu create myapp --template neutralinojs/neutralinojs-zero
```


dev tools:
run with  --window-enable-inspector
or in config:
modes.window.enableInspector: boolean

# Git
make sure to install @neutralinojs/neu globally first, then run *neu update* to 
download the plattform binaries

make sure to run *npm i* in vue-src, tools and extensions 


make sure to not leave a specific websocket port in index.html 

This line must be present before building:

<script src="%PUBLIC_URL%/__neutralino_globals.js"></script>

replaced in dev mode by something like:
<script src="http://localhost:42075/__neutralino_globals.js"></script>

when stopping neu cli with CRTL-C, patch is not reverted. Stop beu with "q <enter>", then close windows. this will revert the patch. 

**NB**: in dev mode with %PUBLIC_URL% present, gui will show error. Right click window to reload, then no error.

**use tools/checkPatch before running "neu build"**


