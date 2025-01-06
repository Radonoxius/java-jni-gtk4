package ix.radon.guiframe.ffi;

import ix.radon.guiframe.core.*;

public class NativeBridge {

    public NativeBridge(String libPath) {
        System.load(libPath);
    }

    public native void getApplication(Application app);
}
