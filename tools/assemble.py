"""Assemble the bundle's single support-module payload from the authored sources.

mixins/friendlybones/runtime.rs = src/core.rs (as `mod logic`) + src/runtime.rs + src/ui.rs.
`python tools/assemble.py --check` fails when the committed payload is stale.
"""
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "mixins/friendlybones/runtime.rs"


def payload() -> str:
    logic = (ROOT / "src/core.rs").read_text(encoding="utf-8").replace("//!", "//")
    runtime = (ROOT / "src/runtime.rs").read_text(encoding="utf-8")
    ui = (ROOT / "src/ui.rs").read_text(encoding="utf-8")
    return "mod logic {\n" + logic + "\n}\n" + runtime + "\n" + ui


if __name__ == "__main__":
    text = payload()
    if "--check" in sys.argv:
        if not OUT.is_file() or OUT.read_text(encoding="utf-8") != text:
            sys.exit("mixins/friendlybones/runtime.rs is stale: run tools/assemble.py")
    else:
        OUT.write_text(text, encoding="utf-8", newline="\n")
        print(f"wrote {OUT.relative_to(ROOT)}")
