"""Make a trained curriculum vault the repository's vault (Design 2.0's final vault).

    python tools/promote_vault.py data/training/curriculum/vaults/<time>
    python tools/promote_vault.py data/training/curriculum/vaults/<time> --yes

Without --yes it only prints what it would do. With --yes:
1. the current vault/ is zipped to research/vault_before_v2_<date>.zip (kept, like
   the v1 vault before its reset);
2. vault/ is replaced by a copy of the trained vault (its translation cache is
   left out: the shared one lives in eval/);
3. you commit and tag it yourself, for example:
       git add -A vault research && git commit -m "Vault: trained Design 2.0 vault" && git tag v2.0.0
"""
import datetime
import os
import shutil
import sys
import zipfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if not args:
        print(__doc__)
        return
    src = os.path.abspath(args[0])
    if not os.path.isdir(os.path.join(src, "procedures")):
        print(f"{src} does not look like a vault (no procedures/)")
        sys.exit(2)
    dst = os.path.join(ROOT, "vault")
    day = datetime.date.today().isoformat()
    backup = os.path.join(ROOT, "research", f"vault_before_v2_{day}.zip")
    n_proc = len(os.listdir(os.path.join(src, "procedures")))
    print(f"trained vault: {src} ({n_proc} procedures)")
    print(f"current vault/ -> {backup}")
    print(f"then vault/ <- copy of the trained vault")
    if "--yes" not in sys.argv:
        print("dry run: add --yes to do it")
        return
    if os.path.exists(backup):
        print(f"{backup} already exists; not overwriting")
        sys.exit(1)
    with zipfile.ZipFile(backup, "w", zipfile.ZIP_DEFLATED) as z:
        for base, _, files in os.walk(dst):
            for f in files:
                p = os.path.join(base, f)
                z.write(p, os.path.relpath(p, ROOT))
    shutil.rmtree(dst)
    shutil.copytree(src, dst, ignore=shutil.ignore_patterns("translate_cache.jsonl"))
    print(f"done: vault/ is the trained vault; the old one is in {backup}")


if __name__ == "__main__":
    main()
