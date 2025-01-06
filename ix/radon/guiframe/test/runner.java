package ix.radon.guiframe.test;

import ix.radon.guiframe.core.Application;

public class runner {

    public static void main(String[] args) {
        Application app = new Application(
            "Test",
            "/home/radon/RustroverProjects/guiframe/target/release/libguiframe.so"
        );
    }
}
