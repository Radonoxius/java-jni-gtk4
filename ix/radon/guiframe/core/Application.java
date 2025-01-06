package ix.radon.guiframe.core;

import ix.radon.guiframe.ffi.NativeBridge;

public class Application {
    public String name;
    NativeBridge bridge;

    public Application(
        String name,
        String nativeLibAbsolutePath
        ) {
        this.name = name;

        bridge = new NativeBridge(nativeLibAbsolutePath);
        bridge.getApplication(this);
    }
}
