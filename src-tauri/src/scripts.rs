// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

// Disable webview native context menu.
// Optional, injected when webview loaded.
pub const JS_INIT_SCRIPT: &'static str = r#"
    (function() {
        document.addEventListener("contextmenu",
            (e) => { e.preventDefault(); return false; },
            { capture: true }
        );
    })();
"#;
