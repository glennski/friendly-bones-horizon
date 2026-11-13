"""Build the two release files (docs/api/mod-files.md), deterministic:

- dist/friendlybones_<version>_minecraft-horizon-<game>-<hash>.hnmod: native code (horizon-native.json, mixins/friendlybones/) for horizon-native-mods/.
- dist/friendlybones-content_<version>_minecraft-horizon-<game>-<hash>.hmod: content (horizon-mod.toml, assets/, data/) for horizon-mods/.

The launcher of this Nightly builds native code from a combined .hmod but does not install its content,
so the two parts ship as two files.
"""
from pathlib import Path
import hashlib, json, subprocess, sys, zipfile

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = "mixins/friendlybones"
BUNDLE_SUFFIXES = {".rs", ".toml"}
BUNDLE_TEXT = ("LICENSE", "README")


def tree(top: str) -> list[str]:
    return [p.relative_to(ROOT).as_posix() for p in (ROOT / top).rglob("*") if p.is_file()]


def native_files() -> list[str]:
    names = ["horizon-native.json"]
    for p in (ROOT / BUNDLE).rglob("*"):
        if p.is_file() and (p.suffix in BUNDLE_SUFFIXES or p.name.startswith(BUNDLE_TEXT)):
            names.append(p.relative_to(ROOT).as_posix())
    return sorted(names)


def content_files() -> list[str]:
    return sorted(["horizon-mod.toml", *tree("assets"), *tree("data")])


def write(out: Path, names: list[str]) -> None:
    out.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        for name in names:
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            z.writestr(info, (ROOT / name).read_bytes())
    print(out.relative_to(ROOT), hashlib.sha256(out.read_bytes()).hexdigest())


def main() -> None:
    subprocess.run([sys.executable, str(ROOT / "tools/assemble.py"), "--check"], check=True)
    native = json.loads((ROOT / "horizon-native.json").read_text())
    mixin = (ROOT / BUNDLE / "horizon-mixin.toml").read_text()
    mod = (ROOT / "horizon-mod.toml").read_text()
    version = native["version"]
    for what, text in (("horizon-mixin.toml", mixin), ("horizon-mod.toml", mod)):
        if f'version = "{version}"' not in text:
            sys.exit(f"{what} does not say version {version}")
    if f'horizon_revision = "{native["horizon"]}"' not in mixin:
        sys.exit("horizon_revision in horizon-mixin.toml differs from horizon-native.json")
    dist = ROOT / "dist"
    for old in dist.glob("friendlybones*"):
        old.unlink()
    suffix = f"{version}_minecraft-horizon-{native['game']}-{native['horizon'][:7]}"  # Agents.md 7.3
    write(dist / f"friendlybones_{suffix}.hnmod", native_files())
    write(dist / f"friendlybones-content_{suffix}.hmod", content_files())


if __name__ == "__main__":
    main()
